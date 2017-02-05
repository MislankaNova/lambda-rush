use *;

extern crate rand;
extern crate sfml;

use self::rand::*;

use self::sfml::system::*;
use self::sfml::window::*;
use self::sfml::graphics::*;

pub fn no_action(_ : &mut Agent, _ : State, _ : &mut XorShiftRng)
        -> Action {
    Action::empty()
}

pub fn velocity_drop_after(agent : &mut Agent, _ : State, _ : &mut XorShiftRng)
        -> Action {
    let l = agent.get_life();
    if l > 1 {
        if l < 10 {
            let v = agent.get_velocity();
            agent.set_velocity(times(v, 0.8));
        }
        agent.set_life(l - 1);
    }
    Action::empty()
}

pub fn lambda(_ : &mut Agent, _ : State, _ : &mut XorShiftRng)
        -> Action {
    Action::empty()
}

pub fn lambda_bullet(agent : &mut Agent, _ : State, _ : &mut XorShiftRng)
        -> Action {
    let p = agent.get_position();
    if p.x < -20.0
            || p.x > DISPLAY_WIDTH + 20.0
            || p.y < -20.0
            || p.y > DISPLAY_HEIGHT + 20.0 {
        agent.terminate();
    }
    Action::empty()
}

pub fn enemy_lisp(agent : &mut Agent, state : State, generator : &mut XorShiftRng)
        -> Action {
    let t = agent.get_ticks();
    agent.set_current((t / 40) % 3);
    if agent.is_dead() {
        let mut rs = Vec::new();
        for i in 0..5 {
            let a = i as f32 * 72.0 + generator.gen_range(-2.0, 2.0);
            let dp = polar_to_vector((60.0, a));
            let r = SpawnRule {
                idea_name : "enemy-lisp-eye",
                behaviour_name : Some("enemy-lisp-eye"),
                position       : agent.get_position() + dp,
                velocity       : polar_to_vector((generator.gen_range(3.0, 5.0), a)),
                angular        : generator.gen_range(-1.6, 1.6),
                rotation       : a,
                damage         : 0,
                life           : 1
            };
            rs.push(r);
        }
        Action::new()
            .multiplier(2)
            .spawn_enemies(rs)
            .emit(agent.get_position())
    } else {
        Action::empty()
    }
}

pub fn enemy_lisp_eye(agent : &mut Agent, state : State, generator : &mut XorShiftRng)
        -> Action {
    if agent.get_ticks() % 40 == 0 {
        agent.set_current(generator.gen_range(0, 5))
    }
    if agent.is_dead() {
        let mut rs = Vec::new();
        for i in 0..12 {
            let a = i as f32 * 30.0 + generator.gen_range(-3.0, 3.0);
            let r = SpawnRule {
                idea_name : "bullet-lisp",
                behaviour_name : Some("effect-lisp"),
                position       : agent.get_position(),
                velocity       : polar_to_vector((generator.gen_range(8.0, 12.0), a)),
                angular        : generator.gen_range(-2.0, 2.0),
                rotation       : a,
                damage         : 0,
                life           : 999999
            };
            rs.push(r);
        }
        Action::new()
            .spawn_effects(rs)
            .emit(agent.get_position())
    } else {
        Action::empty()
    }
}

pub fn effect_lisp(agent : &mut Agent, _ : State, _ : &mut XorShiftRng)
        -> Action {
    if agent.get_ticks() == 30 {
        agent.terminate();
    } else {
        let v = agent.get_velocity();
        agent.set_velocity(times(v, 0.95));
    }
    Action::empty()
}

//

pub fn error(agent : &mut Agent, state : State, _ : &mut XorShiftRng)
        -> Action {
    agent.scale(Vector2f::new(1.03, 1.03));
    let v = agent.get_velocity();
    agent.set_velocity(v + Vector2f::new(0.0, 0.3));
    Action::empty()
}

pub fn particle(agent : &mut Agent, _ : State, _ : &mut XorShiftRng)
        -> Action {
    let v = agent.get_velocity();
    let l = agent.get_life();
    agent.set_velocity(times(v, 0.8));
    agent.set_life(l - 1);
    Action::empty()
}
