// Resource loading and managing

use *;

extern crate rand;
extern crate sfml;

use std::collections::*;
use std::fs::*;
use std::io::Read;

use self::rand::*;

use self::sfml::system::*;
use self::sfml::window::*;
use self::sfml::graphics::*;
use self::sfml::audio::*;

#[derive(Copy, Clone, PartialEq, Eq)]
enum ResourceFileType {
    Font,
    Texture,
    Sound
}

pub struct Resource<'r> {
    fonts         : HashMap<&'r str, Font>,
    textures      : HashMap<&'r str, Texture>,
    sound_buffers : HashMap<&'r str, SoundBuffer>,

    behaviours : HashMap<&'r str, Behaviour>,

    ideas : HashMap<&'r str, Idea>
}

impl<'r> Resource<'r> {
    pub fn new() -> Resource<'r> {
        //Resource::new_from_archive("resource.tar")
        Resource::new_from_archive("resource/")
    }

    pub fn new_from_archive(path : &str) -> Resource<'r> {
        let directory = path;

        let mut fm = HashMap::new();
        let mut tm = HashMap::new();
        let mut sm = HashMap::new();
        let mut im = HashMap::new();
        //let mut bm = HaskMap::new();
        let mut bm : HashMap<&'r str, Behaviour> = HashMap::new();

        macro_rules! extract {
            ($p:expr) => {

            }
        };

        macro_rules! load {
            (font < $n:expr, $f:expr) => {
                match Font::new_from_file($f) {
                    Some(r) => fm.insert($n, r),
                    None    => panic!("Cannot load font {}.", $n)
                };
            };
            (texture < $n:expr, $f:expr) => {
                match Texture::new_from_file(
                    &format!("{}{}", directory, $f)) {
                        Some(r) => tm.insert($n, r),
                        None    => panic!("Cannot load texture {}.", $n)
                };
            };
            (repeat < texture < $n:expr, $f:expr) => {
                match Texture::new_from_file(
                    &format!("{}{}", directory, $f)) {
                        Some(mut r) => {
                            r.set_repeated(true);
                            tm.insert($n, r);
                        },
                        None    => panic!("Cannot load texture {}.", $n)
                };
            };
            (sound < $n:expr, $f:expr) => {
                match SoundBuffer::new(
                    &format!("{}{}", directory, $f)) {
                        Some(r) => sm.insert($n, r),
                        None    => panic!("Cannot load sound {}.", $n)
                };
            }
        }

        //

        load!(font < "serif", "C:/Windows/Fonts/GARA.TTF");

        //

        load!(texture < "lambda-bullet-wave", "self_bullet_bind.png");

        load!(texture < "bullet-lisp", "bullet_lisp.png");

        //

        load!(texture < "enemy-lisp-1", "enemy_lisp_1.png");
        load!(texture < "enemy-lisp-2", "enemy_lisp_2.png");
        load!(texture < "enemy-lisp-3", "enemy_lisp_3.png");
        load!(texture < "enemy-lisp-eye-1", "enemy_lisp_eye_1.png");
        load!(texture < "enemy-lisp-eye-2", "enemy_lisp_eye_2.png");
        load!(texture < "enemy-lisp-eye-3", "enemy_lisp_eye_3.png");
        load!(texture < "enemy-lisp-eye-4", "enemy_lisp_eye_4.png");
        load!(texture < "enemy-lisp-eye-5", "enemy_lisp_eye_5.png");
        // The 'Lisp alien' appearing here is derived from the Lisp Mascot made by Conrad Barski
        // http://lisperati.com/logo.html

        load!(texture < "particle", "particle_lambda.png");

        load!(texture < "hitbox", "self.png");
        load!(texture < "shield", "shield.png");

        load!(repeat < texture < "background-sky",
            "background_sky.png");
        load!(repeat < texture < "background-stars-000",
            "background_stars_000.png");
        load!(repeat < texture < "background-stars-00",
            "background_stars_00.png");
        load!(repeat < texture < "background-stars-0",
            "background_stars_0.png");
        load!(repeat < texture < "background-stars-1",
            "background_stars_1.png");
        load!(repeat < texture < "background-stars-2",
            "background_stars_2.png");
        load!(repeat < texture < "background-cloud",
            "background_cloud.png");

        load!(repeat < texture < "overlay-unsafePerformIO",
            "overlay_unsafePerformIO.png");

        load!(texture < "title", "title.png");
        load!(texture < "dead", "dead.png");

        //

        bm.insert("nothing", behaviour::no_action);
        bm.insert("lambda", behaviour::lambda);

        bm.insert("lambda-bullet", behaviour::lambda_bullet);

        bm.insert("enemy-lisp", behaviour::enemy_lisp);
        bm.insert("enemy-lisp-eye", behaviour::enemy_lisp_eye);
        bm.insert("effect-lisp", behaviour::effect_lisp);

        bm.insert("velocity-drop-after", behaviour::velocity_drop_after);

        bm.insert("error", behaviour::error);
        bm.insert("particle", behaviour::particle);

        //

        im.insert(
            "explosion",
            Idea {
                texture_names : vec!["error-big"],
                body : IdealBody::Nothing,
                offset : Vector2f::new(0.0, 0.0)
            }
        );

        im.insert(
            "error",
            Idea {
                texture_names : vec![
                    "error-1",
                    "error-2",
                    "error-big"
                ],
                body : IdealBody::Nothing,
                offset : Vector2f::new(0.0, 0.0)
            }
        );

        im.insert(
            "particle",
            Idea {
                texture_names : vec!["particle"],
                body : IdealBody::Nothing,
                offset : Vector2f::new(0.0, 0.0)
            }
        );

        im.insert(
            "lambda",
            Idea {
                texture_names : vec!["hitbox"],
                body : IdealBody::Point(3.0),
                offset : Vector2f::new(0.0, 0.0)
            }
        );

        //

        im.insert(
            "lambda-bullet-wave",
            Idea {
                texture_names : vec!["lambda-bullet-wave"],
                body : IdealBody::Point(5.0),
                offset : Vector2f::new(0.0, 0.0)
            }
        );

        //

        im.insert(
            "bullet-lisp",
            Idea {
                texture_names : vec!["bullet-lisp"],
                body : IdealBody::Point(4.0),
                offset : Vector2f::new(0.0, 0.0)
            }
        );

        //

        im.insert(
            "enemy-lisp",
            Idea {
                texture_names : vec![
                    "enemy-lisp-1",
                    "enemy-lisp-2",
                    "enemy-lisp-3"
                ],
                body : IdealBody::Point(80.0),
                offset : Vector2f::new(0.0, 0.0)
            }
        );

        im.insert(
            "enemy-lisp-eye",
            Idea {
                texture_names : vec![
                    "enemy-lisp-eye-1",
                    "enemy-lisp-eye-2",
                    "enemy-lisp-eye-3",
                    "enemy-lisp-eye-4",
                    "enemy-lisp-eye-5"
                ],
                body : IdealBody::Point(28.0),
                offset : Vector2f::new(0.0, 0.0)
            }
        );

        //

        Resource {
            fonts         : fm,
            textures      : tm,
            sound_buffers : sm,

            behaviours    : bm,

            ideas : im
        }
    }

    pub fn get_font(&'r self, name : &str) -> &'r Font {
        match self.fonts.get(name) {
            Some(r) => r,
            None    => panic!("Cannot load font: {}.", name)
        }
    }

    pub fn get_texture(&'r self, name : &str) -> &'r Texture {
        match self.textures.get(name) {
            Some(r) => r,
            None    => panic!("Cannot load texture: {}.", name)
        }
    }

    pub fn get_sound_buffer(&'r self, name : &str) -> &'r SoundBuffer {
        match self.sound_buffers.get(name) {
            Some(r) => r,
            None    => panic!("Cannot load sound: {}.", name)
        }
    }

    pub fn get_behaviour(&'r self, name : &str) -> Behaviour {
        match self.behaviours.get(name) {
            Some(r) => *r,
            None    => panic!("Cannot load behaviour: {}.", name)
        }
    }

    pub fn get_idea(&'r self, name : &str) -> &'r Idea {
        match self.ideas.get(name) {
            Some(r) => r,
            None    => panic!("Cannot load idea: {}.", name)
        }
    }
}
