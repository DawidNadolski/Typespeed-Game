use cgmath::Point2;

use ggez::{event, graphics, Context, GameResult};

use rand::{Rng, seq::SliceRandom};

use std::env;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

const RIGHT_EDGE_BOUND: f32 = 1280.0;
const BOTTOM_EDGE_BOUND: f32 = 720.0;

const BACKGROUND_COLOR: graphics::Color = graphics::Color::new(0.1, 0.2, 0.3, 1.0);

const FONT_SIZE: f32 = 50.0;

const SCORE_BOX: graphics::Rect = graphics::Rect::new(0.0, 670.0, 1280.0, 50.0);
const SCORE_BOX_COLOR: graphics::Color = graphics::Color::new(0.0, 0.0, 1.0, 1.0);
const SCORE_BOX_HEIGHT: f32 = FONT_SIZE;

const INITIAL_SPAWN_TIME: f32 = 3.0;

fn lines_from_file(filename: impl AsRef<Path>) -> String {
    let mut file = File::open(filename).expect("main.rs/lines_from_file(): Can't open file");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("main.rs/lines_from_file(): Can't read file");

    content
}

struct Word {
    x_pos: f32,
    y_pos: f32,
    text: graphics::Text,
    associated_string: String
}

impl Word {
    fn new(context: &mut Context, word: &str) -> GameResult<Word> {
        let font = graphics::Font::new(context, "/DejaVuSerif.ttf")?;
        let text = graphics::Text::new((word, font, FONT_SIZE));
        let mut rng = rand::thread_rng();
        let random_y_pos = rng.gen_range(0.0, BOTTOM_EDGE_BOUND - SCORE_BOX_HEIGHT);

        Ok(Word { 
            x_pos: -250.0, 
            y_pos: random_y_pos,
            text: text,
            associated_string: String::from(word)
        })
    }

    fn draw(&mut self, context: &mut Context) -> GameResult<()>  {
        let destination_point = Point2::new(self.x_pos, self.y_pos);
        graphics::draw(context, &self.text, (destination_point,))?;
        self.x_pos += 1.0;
        
        Ok(())
    }
}

struct MainState {
    words: Vec<Word>,
    vocabulary: Vec<String>,
    current_typed_word: String,
    spawn_time: f32,
    last_update: Instant,
}

impl MainState {

    fn new(_context: &mut Context) -> GameResult<MainState> {

        let path = Path::new("/Users/Dawid/Nauka/Game Dev/Typespeed/resources/english_vocabulary.txt");
        let words_from_file = lines_from_file(path);
        let vocabulary: Vec<String> = words_from_file.split_whitespace().map(|x| x.to_string()).collect();

        Ok(MainState {
            words: Vec::new(),
            vocabulary: vocabulary,
            current_typed_word: String::new(),
            spawn_time: INITIAL_SPAWN_TIME,
            last_update: Instant::now(),
        })
    }

    fn draw(&mut self, context: &mut Context) -> GameResult<()> {
        graphics::clear(context, BACKGROUND_COLOR);
        
        let score_box: graphics::Mesh = graphics::Mesh::new_rectangle(context, graphics::DrawMode::fill(), SCORE_BOX, SCORE_BOX_COLOR)?;
        graphics::draw(context, &score_box, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;

        for word in &mut self.words {
            word.draw(context).expect("main.rs/MainState/draw(): Couldn't draw word");
        }  
        
        Ok(())
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, context: &mut Context) -> GameResult {
        if let Some(index) = self.words.iter().position(|word| word.x_pos > RIGHT_EDGE_BOUND) {
            self.words.remove(index);
        }
        
        if  Instant::now().duration_since(self.last_update) > Duration::from_secs_f32(self.spawn_time) {
            self.words.push(Word::new(context, self.vocabulary.choose(&mut rand::thread_rng()).unwrap()).unwrap());
            self.last_update = Instant::now();
        }
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        self.draw(context)?;
        graphics::present(context)?;
        Ok(())
    }

    fn text_input_event(&mut self, _context: &mut Context, ch: char) {
        if ch != 13 as char {
            self.current_typed_word.push(ch);
        } else {   
            if let Some(index) = self.words.iter().position(|word| word.associated_string == self.current_typed_word) {
                self.words.remove(index);
            } else {
                println!("Index not found!");
            }

            self.current_typed_word = String::new();
        }
    }
}

pub fn main() -> GameResult {
    let resource_directory = if let Ok(manifest_directory) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = PathBuf::from(manifest_directory);
        path.push("resources");
        path
    } else {
        PathBuf::from("./resources")
    };

    let context_builder = ggez::ContextBuilder::new("Typespeed", "Dawid Nadolski")
        .add_resource_path(resource_directory)
        .window_setup(ggez::conf::WindowSetup::default().title("Typespeed! Type as fast as you can!"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(1280.0, 720.0));
    let (context, event_loop) = &mut context_builder.build()?;

    let state = &mut MainState::new(context)?;
    event::run(context, event_loop, state)
}