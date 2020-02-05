use cgmath::Point2;
use ggez::{graphics, Context, GameResult};
use ggez::graphics::{Scale, Text};

const COLOR: graphics::Color = graphics::Color::new(252.0/255.0, 68.0/255.0, 15.0/255.0, 1.0);

pub struct GameStats {
    score: i32,
    matched_words: i32,
    level: i32,
}

impl GameStats {

    pub fn new(score: i32, matched_words: i32, level: i32) -> GameResult<GameStats> {

        Ok(GameStats { 
            score: score,
            matched_words: matched_words,
            level: level
        })
    }

    pub fn draw(&mut self, context: &mut Context) -> GameResult<()>  {

        let gameover = Text::new(graphics::TextFragment {
            text: String::from("GAME OVER!"),
            color: Some(COLOR),
            font: Some(graphics::Font::default()),
            scale: Some(Scale::uniform(100.0)),
        });
        let gameover_cords = Point2::new(350.0, 70.0);
        graphics::draw(context, &gameover, (gameover_cords,))
            .expect("words.rs/Word/draw(): Couldn't draw Text");

        
        let score = Text::new(graphics::TextFragment {
            text: format!("Score: {}", self.score.to_string()),
            color: Some(COLOR),
            font: Some(graphics::Font::default()),
            scale: Some(Scale::uniform(70.0)),
        });
        let score_cords = Point2::new(400.0, 250.0);
        graphics::draw(context, &score, (score_cords,))
            .expect("words.rs/Word/draw(): Couldn't draw Text");
        
        let matched_words = Text::new(graphics::TextFragment {
            text: format!("Matched words: {}", self.matched_words.to_string()),
            color: Some(COLOR),
            font: Some(graphics::Font::default()),
            scale: Some(Scale::uniform(70.0)),
        });
        let matched_words_cords = Point2::new(400.0, 400.0);
        graphics::draw(context, &matched_words, (matched_words_cords,))
            .expect("words.rs/Word/draw(): Couldn't draw Text");

        let achieved_level = Text::new(graphics::TextFragment {
            text: format!("Achieved level: {}", self.level.to_string()),
            color: Some(COLOR),
            font: Some(graphics::Font::default()),
            scale: Some(Scale::uniform(70.0)),
        });
        let achieved_level_cords = Point2::new(400.0, 550.0);
        graphics::draw(context, &achieved_level, (achieved_level_cords,))
            .expect("words.rs/Word/draw(): Couldn't draw Text");
        
        Ok(())
    }
}