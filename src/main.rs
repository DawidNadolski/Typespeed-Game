mod word;
use word::*;

use ggez::{event, graphics, Context, GameResult};

use rand::seq::SliceRandom;

use std::fs::File;
use std::io::Read;
use std::env;
use std::path::{Path,PathBuf};
use std::time::{Duration, Instant};

const BACKGROUND_COLOR: graphics::Color = graphics::Color::new(0.1, 0.2, 0.3, 1.0);

const SCORE_BOX_FIELD: graphics::Rect = graphics::Rect::new(0.0, 670.0, 640.0, 50.0);
const SCORE_BOX_COLOR: graphics::Color = graphics::Color::new(0.4, 0.2, 0.0, 1.0);

const INITIAL_TIME_TO_SPAWN: f32 = 3.0;
const INITIAL_WORD_SPEED: f32 = 1.0;

const RIGHT_EDGE_BOUND: f32 = 1280.0;

struct GameState {
    words: Vec<Word>,
    vocabulary: Vec<String>,
    entered_word: Word,
    seconds_to_spawn: f32,
    last_update: Instant,
    words_speed: f32
}

impl GameState {

    fn new(context: &mut Context) -> GameResult<GameState> {

        let path = Path::new("/Users/Dawid/Nauka/Game Dev/Typespeed/resources/english_vocabulary.txt");
        let words_from_file = lines_from_file(path);
        let vocabulary: Vec<String> = words_from_file.split_whitespace().map(|x| x.to_string()).collect();

        Ok(GameState {
            words: Vec::new(),
            vocabulary: vocabulary,
            entered_word: Word::new_text_input_word(context).unwrap(),
            seconds_to_spawn: INITIAL_TIME_TO_SPAWN,
            last_update: Instant::now(),
            words_speed: INITIAL_WORD_SPEED
        })
    }

    fn draw(&mut self, context: &mut Context) -> GameResult<()> {
        graphics::clear(context, BACKGROUND_COLOR);

        let score_box: graphics::Mesh = graphics::Mesh::new_rectangle(context, graphics::DrawMode::fill(), SCORE_BOX_FIELD, SCORE_BOX_COLOR)
            .expect("main.rs/GameState/draw(): Couldn't create ScoreBox rectangle");
        graphics::draw(context, &score_box, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))
            .expect("main.rs/GameState/draw(): Couldn't draw ScoreBox");

        for word in &mut self.words {
            word.draw(context)
                .expect("main.rs/GameState/draw(): Couldn't draw word");
        }

        self.entered_word.draw(context)
            .expect("main.rs/GameState/draw(): Couldn't draw typed word");
        
        Ok(())
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, context: &mut Context) -> GameResult {
        if let Some(index) = self.words.iter().position(|word| word.x_pos > RIGHT_EDGE_BOUND) {
            self.words.remove(index);
        }
        
        if Instant::now().duration_since(self.last_update) > Duration::from_secs_f32(self.seconds_to_spawn) {
            let new_word = self.vocabulary.choose(&mut rand::thread_rng()).unwrap();
            self.words.push(Word::new(context, new_word, self.words_speed).unwrap());
            self.last_update = Instant::now();
        }
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        self.draw(context)
            .expect("main.rs/EventHandler/draw(): Couldn't draw GameState");
        graphics::present(context)
            .expect("main.rs/EventHandler/draw(): Couldn't draw present context");
        Ok(())
    }

    fn text_input_event(&mut self, _context: &mut Context, ch: char) {
        if ch != 13 as char {
            self.entered_word.associated_string.push(ch);
        } else {   
            if let Some(index) = self.words.iter().position(|word| word.associated_string == self.entered_word.associated_string) {
                self.words.remove(index);
            } else {
                println!("Entered word doesn't match with any on the screen");
            }

            self.entered_word.associated_string = String::new();
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
    let (context, event_loop) = &mut context_builder.build()
        .expect("main.rs/main(): Couldn't create context and event_loop from context_builder");

    let state = &mut GameState::new(context)
        .expect("main.rs/main(): Couldn't create GameState from context");
    event::run(context, event_loop, state)
}

fn lines_from_file(filename: impl AsRef<Path>) -> String {
    let mut file = File::open(filename)
        .expect("main.rs/lines_from_file(): Couldn't open file");
    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("main.rs/lines_from_file(): Couldn't read file");

    content
}