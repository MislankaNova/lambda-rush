// Orbis Tertius

extern crate rand;
extern crate sfml;

use self::rand::*;

use self::sfml::system::*;

pub type Behaviour = fn(&mut Agent, State, &mut XorShiftRng) -> Action;

pub static SQRT_2_DIV_2     : f32 = 0.70710678118;

pub static MARGIN_WIDTH   : f32 = 320.0;
pub static DISPLAY_WIDTH  : f32 = 640.0;
pub static DISPLAY_HEIGHT : f32 = 480.0;
pub static DISPLAY_X_OFFSET : f32 = 0.0;
pub static DISPLAY_Y_OFFSET : f32 = 0.0;

pub static DISPLAY_OFFSET : Vector2f = Vector2f{
    x : 0.0,
    y : 0.0
};

pub mod behaviour;

pub mod vector;
pub mod resource;
pub mod background;
pub mod body;
pub mod idea;
pub mod agent;
pub mod force;
pub mod action;
pub mod command;
pub mod rush;

pub use vector::*;
pub use resource::*;
pub use background::*;
pub use body::*;
pub use idea::*;
pub use agent::*;
pub use force::*;
pub use action::*;
pub use command::*;
pub use rush::*;
