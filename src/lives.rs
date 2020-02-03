use ggez::{graphics, Context, GameResult};
use ggez::graphics::{spritebatch::SpriteBatch, Image};

const HEARTS_SPACE: f32 = 40.0;
const HEART_X_POS: f32 = 855.0;
const HEART_Y_POS: f32 = 670.0;

pub struct Lives {
    pub hearts: Vec<mint::Point2<f32>>,
    lives_batch: SpriteBatch,
}

impl Lives {
    pub fn new(context: &mut Context) -> GameResult<Lives> {
        let mut hearts_vec: Vec<mint::Point2<f32>> = Vec::new();

        for ith_heart in 0..10 {
            hearts_vec.push(mint::Point2 {
                x: HEART_X_POS + (ith_heart as f32) * HEARTS_SPACE, 
                y: HEART_Y_POS
            });
        }

        let texture = Image::new(context, "/heart.png")
            .expect("lives.rs/Lives/new(): Couldn't load heart.png");
        let lives_batch = SpriteBatch::new(texture.clone());

        Ok(Lives{
            hearts: hearts_vec,
            lives_batch: lives_batch,
        })
    }

    pub fn draw(&mut self, context: &mut Context) -> GameResult<()> {
        self.lives_batch.clear();

        for heart in &self.hearts {
            self.lives_batch.add((*heart,));
        }

        graphics::draw(context, &self.lives_batch, (mint::Point2 {x: 0.0, y: 0.0},))?;

        Ok(())
    }
}