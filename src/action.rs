// An action to be perfomed in the world.

use *;

extern crate sfml;

use self::sfml::system::*;
use self::sfml::window::*;
use self::sfml::graphics::*;

#[derive(Copy, Clone, PartialEq)]
pub struct SpawnRule {
    pub idea_name : &'static str,
    //pub texture_name : &'static str,
    pub behaviour_name : Option<&'static str>,
    pub position  : Vector2f,
    pub velocity  : Vector2f,
    pub angular   : f32,
    pub rotation  : f32,
    //pub size     : f32,
    //pub fast      : bool,
    pub damage    : i32,
    pub life      : i32
}

pub struct Action {
    pub spawn_lambda_bullets : Vec<SpawnRule>,
    pub spawn_bullets : Vec<SpawnRule>,
    pub spawn_enemies : Vec<SpawnRule>,
    pub spawn_effects : Vec<SpawnRule>,
    pub show_card     : Option<&'static str>,
    pub play_sound    : Option<&'static str>,
    pub explosion     : i32,

    pub position      : Vector2f,

    pub score      : i32,
    pub multiplier : i32
}

impl Action {
    pub fn new() -> ActionBuilder {
        ActionBuilder {
            spawn_lambda_bullets : Vec::new(),
            spawn_bullets : Vec::new(),
            spawn_enemies : Vec::new(),
            spawn_effects : Vec::new(),
            show_card     : None,
            play_sound    : None,
            explosion     : 0,

            score : 0,
            multiplier : 0,
        }
    }

    pub fn empty() -> Action {
        Action {
            spawn_lambda_bullets : Vec::new(),
            spawn_bullets : Vec::new(),
            spawn_enemies : Vec::new(),
            spawn_effects : Vec::new(),
            show_card     : None,
            play_sound    : None,
            explosion     : 0,

            position      : Vector2f::new(0.0, 0.0),

            score      : 0,
            multiplier : 0
        }
    }

    pub fn show_card(card : &'static str) -> Action {
        Action {
            spawn_lambda_bullets : Vec::new(),
            spawn_bullets : Vec::new(),
            spawn_enemies : Vec::new(),
            spawn_effects : Vec::new(),
            show_card     : Some(card),
            play_sound    : None,
            explosion     : 0,

            position      : Vector2f::new(0.0, 0.0),

            score      : 0,
            multiplier : 0
        }
    }
}

pub struct ActionBuilder {
    spawn_lambda_bullets : Vec<SpawnRule>,
    spawn_bullets : Vec<SpawnRule>,
    spawn_enemies : Vec<SpawnRule>,
    spawn_effects : Vec<SpawnRule>,
    show_card     : Option<&'static str>,
    play_sound    : Option<&'static str>,
    explosion     : i32,

    score      : i32,
    multiplier : i32
}

impl ActionBuilder {
    pub fn emit(self, position : Vector2f) -> Action {
        Action {
            spawn_lambda_bullets : self.spawn_lambda_bullets,
            spawn_bullets : self.spawn_bullets,
            spawn_enemies : self.spawn_enemies,
            spawn_effects : self.spawn_effects,
            show_card     : self.show_card,
            play_sound    : self.play_sound,
            explosion     : self.explosion,

            position      : position,

            score      : self.score,
            multiplier : self.multiplier
        }
    }

    pub fn spawn_lambda_bullets(self, rules : Vec<SpawnRule>)
            -> ActionBuilder {
        ActionBuilder {
            spawn_lambda_bullets : rules,
            .. self
        }
    }

    pub fn spawn_bullets(self, rules : Vec<SpawnRule>)
            -> ActionBuilder {
        ActionBuilder {
            spawn_bullets : rules,
            .. self
        }
    }

    pub fn spawn_enemies(self, rules : Vec<SpawnRule>)
            -> ActionBuilder {
        ActionBuilder {
            spawn_enemies : rules,
            .. self
        }
    }

    pub fn spawn_effects(self, rules : Vec<SpawnRule>)
            -> ActionBuilder {
        ActionBuilder {
            spawn_lambda_bullets : rules,
            .. self
        }
    }

    pub fn show_card(self, card : Option<&'static str>)
            -> ActionBuilder {
        ActionBuilder {
            show_card : card,
            .. self
        }
    }

    pub fn play_sound(self, sound : Option<&'static str>)
            -> ActionBuilder {
        ActionBuilder {
            play_sound : sound,
            .. self
        }
    }

    pub fn explosion(self, size : i32)
            -> ActionBuilder {
        ActionBuilder {
            explosion : size,
            .. self
        }
    }

    pub fn score(self, score : i32)
            -> ActionBuilder {
        ActionBuilder {
            score : score,
            .. self
        }
    }

    pub fn multiplier(self, multiplier : i32)
            -> ActionBuilder {
        ActionBuilder {
            multiplier : multiplier,
            .. self
        }
    }
}
