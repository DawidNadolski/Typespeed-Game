mod word;
mod lives;
mod game_stats;
use word::*;
use lives::*;
use game_stats::*;

use ggez::{event, graphics, Context, GameResult};

use rand::seq::SliceRandom;

use std::fs::File;
use std::io::Read;
use std::env;
use std::path::{Path,PathBuf};
use std::time::{Duration, Instant};

const BACKGROUND_COLOR: graphics::Color = graphics::Color::new(63.0/255.0, 94.0/255.0, 90.0/255.0, 1.0);
const RIGHT_EDGE_BOUND: f32 = 1280.0;

const SCORE_BOX_FIELD: graphics::Rect = graphics::Rect::new(0.0, 669.0, 1280.0, 51.0);
const SCORE_BOX_COLOR: graphics::Color = graphics::Color::new(56.0/255.0, 66.0/255.0, 59.0/255.0, 1.0);
const BOTTOM_LINE_CORDS: &[mint::Point2<f32>] = &[(mint::Point2 {x: 0.0, y: 667.0}), (mint::Point2 {x: 1280.0, y: 667.0})];
const MIDDLE_LINE_CORDS: &[mint::Point2<f32>] = &[(mint::Point2 {x: 602.0, y: 667.0}), (mint::Point2 {x: 602.0, y: 720.0})];

const INITIAL_TIME_TO_SPAWN: f32 = 2.5;
const INITIAL_WORD_SPEED: f32 = 0.7;
const SPEED_PER_LEVEL: f32 = 0.30;
const TIME_PER_LEVEL: f32 = 1.5;

const ENTER: char = 13 as char;
const BACKSPACE: char = 127 as char;

struct GameState {
    words: Vec<Word>,
    possible_heights: Vec<f32>,
    vocabulary: Vec<String>,
    entered_word: Word,
    seconds_to_spawn: f32,
    last_update: Instant,
    words_speed: f32,
    score: i32,
    matched_words: i32,
    level: i32,
    score_label: Word,
    life: Lives,
    gameover: bool,
}

impl GameState {

    fn new(context: &mut Context) -> GameResult<GameState> {

        let path = Path::new("/Users/Dawid/Nauka/Game Dev/Typespeed/resources/english_vocabulary.txt");
        let words_from_file = lines_from_file(path);
        let vocabulary: Vec<String> = words_from_file.split_whitespace().map(|x| x.to_string()).collect();
        let heights: Vec<f32> = vec![0.3, 41.6, 82.9, 124.2, 165.5, 206.8, 248.1, 289.4, 330.7, 372.0, 413.3, 454.6, 495.9, 537.2, 578.5];

        Ok(GameState {
            words: Vec::new(),
            possible_heights: heights,
            vocabulary: vocabulary,
            entered_word: Word::new_text_input().unwrap(),
            seconds_to_spawn: INITIAL_TIME_TO_SPAWN,
            last_update: Instant::now(),
            score: 0,
            matched_words: 0,
            words_speed: INITIAL_WORD_SPEED,
            level: 1,
            score_label: Word::new_score_label().unwrap(),
            life: Lives::new(context).unwrap(),
            gameover: false,
        })
    }

    fn draw(&mut self, context: &mut Context) -> GameResult<()> {
        graphics::clear(context, BACKGROUND_COLOR);

        let score_box: graphics::Mesh = graphics::Mesh::new_rectangle(context, graphics::DrawMode::fill(), SCORE_BOX_FIELD, SCORE_BOX_COLOR)
            .expect("main.rs/GameState/draw(): Couldn't create ScoreBox rectangle");
        let bottom_line: graphics::Mesh = graphics::Mesh::new_line(context, BOTTOM_LINE_CORDS, 5.0, [0.0, 0.0, 0.0, 1.0].into())
            .expect("main.rs/GameState/draw(): Couldn't create bottom_line");
        let middle_line: graphics::Mesh = graphics::Mesh::new_line(context, MIDDLE_LINE_CORDS, 5.0, [0.0, 0.0, 0.0, 1.0].into())
            .expect("main.rs/GameState/draw(): Couldn't create middle_line");    
        
        graphics::draw(context, &score_box, (mint::Point2 { x: 0.0, y: 0.0 },))
            .expect("main.rs/GameState/draw(): Couldn't draw ScoreBox");
        graphics::draw(context, &bottom_line, (mint::Point2 { x: 0.0, y: 0.0 },))
            .expect("main.rs/GameState/draw(): Couldn't draw bottom_line");
        graphics::draw(context, &middle_line, (mint::Point2 { x: 0.0, y: 0.0 },))
            .expect("main.rs/GameState/draw(): Couldn't draw middle_line");
        
        for word in &mut self.words {
            word.draw(context)
                .expect("main.rs/GameState/draw(): Couldn't draw word");
        }

        self.entered_word.draw(context)
            .expect("main.rs/GameState/draw(): Couldn't draw typed word");
        
        self.score_label.draw(context)
            .expect("main.rs/GameState/draw(): Couldn't draw typed word");

        self.life.draw(context)
            .expect("main.rs/GameState/draw(): Couldn't draw life");
        
        Ok(())
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, _context: &mut Context) -> GameResult {
        
        if !self.gameover {
            if let Some(index) = self.words.iter().position(|word| word.x_pos > RIGHT_EDGE_BOUND) {
                self.words.remove(index);
                self.life.hearts.pop();
            }

            if self.life.hearts.len() == 0 { 
                self.gameover = true; 
            }

            if self.score > self.level * 100 {
                self.level += 1;
                self.seconds_to_spawn /= TIME_PER_LEVEL as f32;
                self.words_speed += SPEED_PER_LEVEL as f32;
            }

            if Instant::now().duration_since(self.last_update) > Duration::from_secs_f32(self.seconds_to_spawn) {

                if self.possible_heights.is_empty() {
                    self.possible_heights = vec![0.3, 41.6, 82.9, 124.2, 165.5, 206.8, 248.1, 289.4, 330.7, 372.0, 413.3, 454.6, 495.9, 537.2, 578.5];
                }

                let random_word = self.vocabulary.choose(&mut rand::thread_rng()).unwrap();
                let random_y_pos = self.possible_heights.choose(&mut rand::thread_rng()).unwrap();
                let new_word = Word::new(random_word, self.words_speed, *random_y_pos).unwrap();
                
                self.words.push(new_word);
                
                if let Some(index) = self.possible_heights.iter().position(|y_pos| y_pos == random_y_pos) {
                    self.possible_heights.remove(index);
                }
                
                self.last_update = Instant::now();
            }
        }

        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        if !self.gameover {
            self.draw(context)
                .expect("main.rs/EventHandler/draw(): Couldn't draw GameState");
        } else {
            graphics::clear(context, BACKGROUND_COLOR);
            let mut go = GameStats::new(self.score, self.matched_words, self.level).unwrap();
            go.draw(context)
                .expect("main.rs/EventHandler/draw(): Couldn't draw GameStats");
        }
        graphics::present(context)
                .expect("main.rs/EventHandler/draw(): Couldn't draw present context");

        Ok(())
    }

    fn text_input_event(&mut self, _context: &mut Context, ch: char) {
        if ch == BACKSPACE {
            self.entered_word.associated_string.pop();
        } else if ch != ENTER as char && self.entered_word.associated_string.len() < 16 {
            self.entered_word.associated_string.push(ch);
        } else {   
            if let Some(index) = self.words.iter().position(|word| word.associated_string == self.entered_word.associated_string) {
                self.score += self.words[index].rank;
                self.matched_words += 1;
                self.score_label.associated_string = format!("Score: {}", self.score.to_string());
                self.words.remove(index);
            } else {
                self.life.hearts.pop();
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