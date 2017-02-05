// Definitions of agents

use *;

extern crate sfml;

use self::sfml::system::Vector2f;

#[derive(Clone, PartialEq)]
pub struct Idea {
    pub texture_names : Vec<&'static str>,
    pub body          : IdealBody,
    pub offset        : Vector2f
}
