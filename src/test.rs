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
    fn new(ctx: &mut Context, raw_text: &str) -> GameResult<Word> {
        let font = graphics::Font::new(ctx, "/DejaVuSerif.ttf")?;
        let word = graphics::Text::new(("Hello", font, 20.0));
        let mut rng = rand::thread_rng();
        let pos = rng.gen_range(0.0, 720.0);

        Ok(Word { 
            x_pos: 0.0, 
            y_pos: pos,
            text: word
        })
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()>  {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        let dest_point = cgmath::Point2::new(self.x_pos, 500.0);
        self.x_pos += 1.0;
        graphics::draw(ctx, &self.text, (dest_point,));

        Ok(())
    }
}

struct MainState {
    words: Vec<Word>
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let mut words_vec = Vec::new();
        words_vec.push(Word::new(ctx, "Pyszczek").unwrap());
        words_vec.push(Word::new(ctx, "Kaczorek").unwrap());
        words_vec.push(Word::new(ctx, "Groszek").unwrap());
        words_vec.push(Word::new(ctx, "Wiktoria").unwrap());
        words_vec.push(Word::new(ctx, "Dawid").unwrap());

        let s = MainState { words: words_vec };

        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        // for word in self.words.iter() {
        //     word.draw(ctx);
        // }  
        
        graphics::present(ctx)?;

        Ok(())
    }
}

pub fn main() -> GameResult {
    // GGEZ will search for files in resources
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
    let (ctx, event_loop) = &mut cb.build()?;

    let state = &mut MainState::new(ctx)?;
    event::run(ctx, event_loop, state)
}