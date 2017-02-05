// An agent is something that moves.

use *;

extern crate rand;
extern crate sfml;

use self::rand::{Rng, XorShiftRng};

use self::sfml::system::*;
use self::sfml::window::*;
use self::sfml::graphics::*;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum AgentType {
    Renko,
    RenkoBullet,
    Bullet,
    Enemy,
    Effect
}

pub struct Agent<'a> {
    ticks  : i32,
    life   : i32,
    damage : i32,
    dead   : bool,
    shield : i32,

    position      : Vector2f,
    position_last : Vector2f,
    velocity      : Vector2f,
    angular_velocity : f32,

    phantoms : Vec<Sprite<'a>>,
    current  : i32,

    behaviour : Behaviour,

    body      : &'a IdealBody,

    state     : State
}

impl<'a> Agent<'a> {
    pub fn new(
            mut phantoms  : Vec<Sprite<'a>>,
            behaviour : Behaviour,
            position  : Vector2f,
            offset    : Vector2f,
            velocity  : Vector2f,
            angular   : f32,
            damage    : i32,
            life      : i32,
            body      : &'a IdealBody,
            state     : State) -> Agent<'a> {

        for p in phantoms.iter_mut() {
            p.set_position(&(position + DISPLAY_OFFSET));
            p.set_origin(&offset);
        }

        Agent {
            ticks  : 0,
            life   : life,
            damage : damage,
            dead   : false,
            shield : 0,

            position      : position,
            position_last : position,
            velocity      : velocity,
            angular_velocity : angular,

            phantoms : phantoms,
            current  : 0,

            behaviour : behaviour,

            body : body,

            state     : state
        }
    }

    pub fn new_from_texture(
            texture   : &'a Texture,
            behaviour : Behaviour,
            position  : Vector2f,
            offset    : Vector2f,
            velocity  : Vector2f,
            angular   : f32,
            rotation  : f32,
            damage    : i32,
            life      : i32,
            body      : &'a IdealBody,
            state     : State) -> Agent<'a> {
        Agent::new_from_textures(
            &vec![texture],
            behaviour,
            position,
            offset,
            velocity,
            angular,
            rotation,
            damage,
            life,
            body,
            state
        )
    }

    pub fn new_from_textures(
            textures  : &Vec<&'a Texture>,
            behaviour : Behaviour,
            position  : Vector2f,
            offset    : Vector2f,
            velocity  : Vector2f,
            angular   : f32,
            rotation  : f32,
            damage    : i32,
            life      : i32,
            body      : &'a IdealBody,
            state     : State) -> Agent<'a> {
        let mut ps = Vec::new();
        for t in textures.iter() {
            let dim = t.get_size();
            let mut p = match Sprite::new_with_texture(t) {
                Some(r) => r,
                None    => panic!("Cannot create phantom.")
            };
            let origin = Vector2f::new(
                dim.x as f32 / 2.0,
                dim.y as f32 / 2.0
            );
            p.set_origin(&(origin + offset));
            p.set_position(&(position + DISPLAY_OFFSET));
            p.set_rotation(-rotation);
            ps.push(p);
        }

        Agent {
            ticks  : 0,
            life   : life,
            damage : damage,
            dead   : false,
            shield : 0,

            position      : position,
            position_last : position,
            velocity      : velocity,
            angular_velocity : angular,
            //acceleration : acceleration,

            phantoms : ps,
            current  : 0,

            behaviour : behaviour,
            //size      : size,
            //fast      : fast,

            body : body,

            state     : state
        }
    }

    pub fn terminate(&mut self) {
        self.dead = true;
    }

    pub fn is_dead(&self) -> bool {
        self.dead
    }

    pub fn tick(&mut self) {
        self.ticks += 1;
    }

    pub fn get_ticks(&self) -> i32 {
        self.ticks
    }

    pub fn get_phantom(&self) -> &Sprite {
        &self.phantoms[self.current as usize]
    }

    pub fn get_current(&self) -> i32 {
        self.current
    }

    pub fn get_position(&self) -> Vector2f {
        self.position
    }

    pub fn get_position_last(&self) -> Vector2f {
        self.position_last
    }

    pub fn get_velocity(&self) -> Vector2f {
        self.velocity
    }

    pub fn get_angular_velocity(&self) -> f32 {
        self.angular_velocity
    }

    pub fn get_rotation(&self) -> f32 {
        -self.phantoms[0].get_rotation()
    }

    pub fn get_size(&self) -> f32 {
        match self.body {
            &IdealBody::Nothing    => 0.0,
            &IdealBody::Point(r)   => r
        }
    }

    pub fn get_scale(&self) -> Vector2f {
        self.phantoms[0].get_scale()
    }

    pub fn get_behaviour(&self) -> Behaviour {
        self.behaviour
    }

    pub fn get_damage(&self) -> i32 {
        self.damage
    }

    pub fn get_life(&self) -> i32 {
        self.life
    }

    pub fn get_shield(&self) -> i32 {
        self.shield
    }

    pub fn get_state(&self) -> State {
        self.state.clone()
    }

    pub fn get_body(&self) -> Body {
        self.body.to_body(self.get_position(), self.get_rotation())
    }

    pub fn set_current(&mut self, index : i32) {
        self.current = index;
    }

    pub fn set_position(&mut self, position : Vector2f) {
        self.position_last = self.get_position();
        self.position = position;
        for p in self.phantoms.iter_mut() {
            p.set_position(&(position + DISPLAY_OFFSET));
        }
    }

    pub fn set_velocity(&mut self, velocity : Vector2f) {
        self.velocity = velocity;
    }

    pub fn set_angular_velocity(&mut self, angular : f32) {
        self.angular_velocity = angular;
    }

    pub fn set_rotation(&mut self, rotation : f32) {
        for p in self.phantoms.iter_mut() {
            p.set_rotation(-rotation);
        }
    }

    pub fn set_scale(&mut self, scale : Vector2f) {
        for p in self.phantoms.iter_mut() {
            p.set_scale(&scale);
        }
    }

    pub fn scale(&mut self, scale : Vector2f) {
        for p in self.phantoms.iter_mut() {
            p.scale(&scale);
        }
    }

    pub fn set_damage(&mut self, damage : i32) {
        self.damage = damage;
    }

    pub fn set_life(&mut self, life : i32) {
        self.life = life;
        if self.get_life() <= 0 {
            self.terminate();
        }
    }

    pub fn set_shield(&mut self, shield : i32) {
        self.shield = shield;
    }

    pub fn set_state(&mut self, state : State) {
        self.state = state;
    }

    pub fn move_amount(&mut self, amount : Vector2f) {
        let p = self.get_position();
        self.set_position(amount + p);
    }

    pub fn update(&mut self) {
        let v = self.get_velocity();
        self.move_amount(v);
        let a = self.get_rotation() + self.get_angular_velocity();
        self.set_rotation(a);
        if self.shield > 0 {
            self.shield -= 1;
        }
    }

    pub fn act(&mut self, state : State, rng : &mut XorShiftRng)
            -> Action {
        (self.behaviour)(self, state, rng)
    }

    pub fn take_force(&mut self, force : &Force, rng : &mut XorShiftRng)
            -> Action {
        match force {
            &Force::Damage(ref d) => {
                let mut es = Vec::new();
                let mut e = 0;
                if self.get_shield() <= 0 {
                    let l = self.get_life();
                    self.set_life(l - d);
                    e = 1;
                } else {
                    for _ in 0..rng.gen_range(8, 12) {
                        let r = SpawnRule {
                            idea_name : "dot",
                            behaviour_name : Some("explosion-particle"),
                            position       : self.get_position(),
                            velocity       : polar_to_vector((
                                rng.gen_range(8.0, 40.0),
                                rng.gen_range(0.0, 360.0)
                            )),
                            angular  : rng.gen_range(-8.0, 8.0),
                            rotation : rng.gen_range(0.0, 360.0),
                            damage   : 999999,
                            life     : 15
                        };
                        es.push(r);
                    }
                }
                if self.is_dead() {
                    Action::new()
                        .spawn_effects(es)
                        .explosion(2)
                        .score(1)
                        .emit(self.get_position())
                } else {
                    Action::new()
                        .spawn_effects(es)
                        .explosion(e)
                        .emit(self.get_position())
                }
            }
        }
    }
}
