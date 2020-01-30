use cgmath::Point2;
use ggez::{graphics, Context, GameResult};
use ggez::graphics::{Scale, Text};
use rand::Rng;

const BOTTOM_EDGE_BOUND: f32 = 720.0;

const FONT_SIZE: f32 = 40.0;

const SCORE_BOX_HEIGHT: f32 = FONT_SIZE + 10.0;

const NEUTRAL_COLOR: graphics::Color = graphics::Color::new(1.0, 1.0, 1.0, 1.0);
const WARNING_COLOR: graphics::Color = graphics::Color::new(1.0, 1.0, 0.0, 1.0);
const HIGH_RISK_COLOR: graphics::Color = graphics::Color::new(1.0, 0.0, 0.0, 1.0);

pub struct Word {
    pub x_pos: f32,
    y_pos: f32,
    pub associated_string: String,
    speed: f32,
}

impl Word {

    pub fn new(_context: &mut Context, word: &str, speed: f32) -> GameResult<Word> {
        let mut rng = rand::thread_rng();
        let random_y_pos = rng.gen_range(0.0, BOTTOM_EDGE_BOUND - 2.0 * SCORE_BOX_HEIGHT);

        Ok(Word { 
            x_pos: -250.0, 
            y_pos: random_y_pos,
            associated_string: String::from(word),
            speed: speed,
        })
    }

    pub fn new_text_input_word(_context: &mut Context) -> GameResult<Word> {

        Ok(Word { 
            x_pos: 10.0, 
            y_pos: 673.0,
            associated_string: String::new(),
            speed: 0.0,
        })
    }

    pub fn draw(&mut self, context: &mut Context) -> GameResult<()>  {
        let mut color = NEUTRAL_COLOR;
        if self.x_pos > 600.0 && self.y_pos <= 620.0 {
            color = WARNING_COLOR;
        }
        if self.x_pos > 950.0 && self.y_pos <= 640.0 {
            color = HIGH_RISK_COLOR;
        }
        
        let word = &self.associated_string;
        let text = Text::new(graphics::TextFragment {
            text: word.to_string(),
            color: Some(color),
            font: Some(graphics::Font::default()),
            scale: Some(Scale::uniform(FONT_SIZE)),
        });

        let destination_point = Point2::new(self.x_pos, self.y_pos);
        graphics::draw(context, &text, (destination_point,))
            .expect("words.rs/Word/draw(): Couldn't draw Text");
        self.x_pos += self.speed;
        
        Ok(())
    }
}