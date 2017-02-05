extern crate sfml;

use *;

use self::sfml::system::*;
use self::sfml::graphics::*;

pub struct Background<'bg> {
    repeat   : bool,

    phantoms : Vec<Sprite<'bg>>,
    current  : i32,

    distance : f32,
    position : Vector2f
}

impl<'bg> Background<'bg> {
    pub fn new_from_textures(
            textures : &Vec<&'bg Texture>,
            distance : f32,
            repeat   : bool) -> Background<'bg> {
        let mut ps = Vec::new();
        for t in textures.iter() {
            let mut p = match Sprite::new_with_texture(t) {
                Some(r) => r,
                None    => panic!("Cannot create phantom.")
            };
            p.set_position(&DISPLAY_OFFSET);
            ps.push(p);
        }

        Background {
            repeat   : repeat,

            phantoms : ps,
            current  : 0,

            distance : distance,
            position : Vector2f::new(0.0, 0.0)
        }
    }

    pub fn get_distance(&self) -> f32 {
        self.distance
    }

    pub fn get_current(&self) -> i32 {
        self.current
    }

    pub fn get_phantom(&self) -> &Sprite {
        &self.phantoms[self.get_current() as usize]
    }

    pub fn set_distance(&mut self, distance : f32) {
        self.distance = distance;
    }

    pub fn set_current(&mut self, index : i32) {
        self.current = index
    }

    pub fn set_position(&mut self, position : Vector2f) {
        //self.position = position;
        for p in self.phantoms.iter_mut() {
            p.set_position(&position);
        }
    }

    pub fn update(&mut self, displacement : Vector2f) {
        let dp = times(displacement, 1.0 / self.distance);
        if self.repeat {
            self.position = self.position - dp;
            for p in self.phantoms.iter_mut() {
                let r = p.get_texture_rect();
                p.set_texture_rect(&IntRect::new(
                    self.position.x as i32,
                    self.position.y as i32,
                    r.width,
                    r.height
                ));
            }
        } else {
            self.position = self.position + dp;
            for p in self.phantoms.iter_mut() {
                p.set_position(&self.position);
            }
        }
    }
}
