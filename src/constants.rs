use ggez::{event, graphics, Context, GameResult};

pub const BACKGROUND_COLOR: graphics::Color = graphics::Color::new(63.0/255.0, 94.0/255.0, 90.0/255.0, 1.0);
pub const SCORE_BOX_FIELD: graphics::Rect = graphics::Rect::new(0.0, 669.0, 600.0, 51.0);
pub const SCORE_BOX_COLOR: graphics::Color = graphics::Color::new(56.0/255.0, 66.0/255.0, 59.0/255.0, 1.0);
pub const BOTTOM_LINE_CORDS: &[mint::Point2<f32>] = &[(mint::Point2 {x: 0.0, y: 667.0}), (mint::Point2 {x: 1280.0, y: 667.0})];
pub const MIDDLE_LINE_CORDS: &[mint::Point2<f32>] = &[(mint::Point2 {x: 602.0, y: 667.0}), (mint::Point2 {x: 602.0, y: 720.0})];

pub const INITIAL_TIME_TO_SPAWN: f32 = 2.0;
pub const INITIAL_WORD_SPEED: f32 = 1.0;

pub const RIGHT_EDGE_BOUND: f32 = 1280.0;

pub const HEARTS_SPACE: f32 = 40.0;
pub const HEART_X_POS: f32 = 855.0;
pub const HEART_Y_POS: f32 = 670.0;

pub const BOTTOM_EDGE_BOUND: f32 = 720.0;

pub const FONT_SIZE: f32 = 40.0;

pub const SCORE_BOX_HEIGHT: f32 = FONT_SIZE + 10.0;

pub const NEUTRAL_COLOR: graphics::Color = graphics::Color::new(1.0, 1.0, 1.0, 1.0);
pub const WARNING_COLOR: graphics::Color = graphics::Color::new(240.0/255.0, 207.0/255.0, 101.0/255.0, 1.0);
pub const HIGH_RISK_COLOR: graphics::Color = graphics::Color::new(252.0/255.0, 68.0/255.0, 15.0/255.0, 1.0);
