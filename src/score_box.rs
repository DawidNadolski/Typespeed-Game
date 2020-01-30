mod word;
use word::*;

use ggez::{event, graphics, Context, GameResult};

use rand::seq::SliceRandom;

use std::fs::File;
use std::io::Read;
use std::env;
use std::path::{Path,PathBuf};
use std::time::{Duration, Instant};

pub struct Word {
    currently_typed_word: Word,
    
}

impl Word {

    pub fn new(context: &mut Context, word: &str, speed: f32) -> GameResult<Word> {
        let font = graphics::Font::new(context, "/DejaVuSerif.ttf")?;
        let text = graphics::Text::new((word, font, FONT_SIZE));
        let mut rng = rand::thread_rng();
        let random_y_pos = rng.gen_range(0.0, BOTTOM_EDGE_BOUND - 2.0 * SCORE_BOX_HEIGHT);

        Ok(Word { 
            x_pos: -250.0, 
            y_pos: random_y_pos,
            text: text,
            associated_string: String::from(word),
            speed: speed,
        })
    }

    pub fn new(context: &mut Context, word: &str) -> GameResult<Word> {
        let font = graphics::Font::new(context, "/DejaVuSerif.ttf")?;
        let text = graphics::Text::new((word, font, FONT_SIZE));
        let mut rng = rand::thread_rng();
        let random_y_pos = rng.gen_range(0.0, BOTTOM_EDGE_BOUND - 2.0 * SCORE_BOX_HEIGHT);

        Ok(Word { 
            x_pos: -250.0, 
            y_pos: random_y_pos,
            text: text,
            associated_string: String::from(word),
            speed: speed,
        })
    }

    pub fn draw(&mut self, context: &mut Context) -> GameResult<()>  {
        let destination_point = Point2::new(self.x_pos, self.y_pos);
        graphics::draw(context, &self.text, (destination_point,))?;
        self.x_pos += self.speed;
        
        Ok(())
    }
}