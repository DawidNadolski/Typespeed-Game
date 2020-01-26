use cgmath;
use ggez;
use rand;

use ggez::event;
use ggez::graphics;
use ggez::{Context, GameResult};
use rand::Rng;
use std::env;
use std::path;


struct Word {
    x_pos: f32,
    y_pos: f32,
    text: graphics::Text,
}

impl Word {
    fn new(context: &mut Context, raw_text: &str) -> GameResult<Word> {
        let font = graphics::Font::new(context, "/DejaVuSerif.ttf")?;
        let word = graphics::Text::new((raw_text, font, 50.0));
        let mut rng = rand::thread_rng();
        let random_y_pos = rng.gen_range(0.0, 720.0);

        Ok(Word { 
            x_pos: 0.0, 
            y_pos: random_y_pos,
            text: word
        })
    }

    fn draw(&mut self, context: &mut Context) -> GameResult<()>  {

        let dest_point = cgmath::Point2::new(self.x_pos, self.y_pos);
        self.x_pos += 1.0;
        graphics::draw(context, &self.text, (dest_point,))?;
        Ok(())
    }
}

struct MainState {
    words: Vec<Word>
}

impl MainState {
    fn new(context: &mut Context) -> GameResult<MainState> {
        let mut words_vec = Vec::new();
        words_vec.push(Word::new(context, "Chuj").unwrap());
        words_vec.push(Word::new(context, "Dupa").unwrap());
        words_vec.push(Word::new(context, "Kurwa").unwrap());
        words_vec.push(Word::new(context, "Cipa").unwrap());
        words_vec.push(Word::new(context, "XD").unwrap());

        let s = MainState { words: words_vec };

        Ok(s)
    }

    fn draw(&mut self, context: &mut Context) -> GameResult<()> {
        graphics::clear(context, [0.1, 0.2, 0.3, 1.0].into());

        for word in &mut self.words {
            word.draw(context);
        }  
        
        Ok(())
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _context: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        
        self.draw(context)?;
        graphics::present(context)?;
        Ok(())
    }
}

pub fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let cb = ggez::ContextBuilder::new("Typespeed", "Dawid Nadolski")
        .add_resource_path(resource_dir)
        .window_setup(ggez::conf::WindowSetup::default().title("Typespeed! Type as fast as you can!"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(1280.0, 720.0));
    let (context, event_loop) = &mut cb.build()?;

    let state = &mut MainState::new(context)?;
    event::run(context, event_loop, state)
}