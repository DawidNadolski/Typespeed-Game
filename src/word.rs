use cgmath::Point2;
use ggez::{graphics, Context, GameResult};
use ggez::graphics::{Scale, Text};

const FONT_SIZE: f32 = 40.0;

const NEUTRAL_COLOR: graphics::Color = graphics::Color::new(1.0, 1.0, 1.0, 1.0);
const WARNING_COLOR: graphics::Color = graphics::Color::new(240.0/255.0, 207.0/255.0, 101.0/255.0, 1.0);
const HIGH_RISK_COLOR: graphics::Color = graphics::Color::new(252.0/255.0, 68.0/255.0, 15.0/255.0, 1.0);

pub struct Word {
    pub x_pos: f32,
    y_pos: f32,
    pub associated_string: String,
    speed: f32,
    pub rank: i32,
}

impl Word {

    pub fn new(word: &str, speed: f32, y_pos: f32) -> GameResult<Word> {

        Ok(Word { 
            x_pos: -250.0, 
            y_pos: y_pos,
            associated_string: String::from(word),
            speed: speed,
            rank: word.len() as i32,
        })
    }

    pub fn new_text_input() -> GameResult<Word> {

        Ok(Word { 
            x_pos: 15.0, 
            y_pos: 673.0,
            associated_string: String::new(),
            speed: 0.0,
            rank: 0,
        })
    }

    pub fn new_score_label() -> GameResult<Word> {
        
        Ok(Word {
            x_pos: 617.0,
            y_pos: 673.0,
            associated_string: String::from("Score: 0"),
            speed: 0.0,
            rank: 0,
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