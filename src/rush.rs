// Main game stuff of Orbis Tertius

use *;

extern crate rand;
extern crate sfml;

use std::collections::BTreeMap;

use self::rand::*;

use self::sfml::system::*;
use self::sfml::graphics::*;
use self::sfml::audio::*;

static OT_FIRE_INTERVAL    : i32 = 4;
static OT_MISSILE_INTERVAL : i32 = 10;
static OT_LAMBDA_ACC_NATURAL : f32 = 0.08;
static OT_LAMBDA_ACC_HIGH  : f32 = 0.6;
static OT_LAMBDA_ACC_LOW   : f32 = 0.4;
static OT_LAMBDA_MAX_SPEED : f32 = 7.2;
static OT_LAMBDA_MIN_SPEED : f32 = -2.4;

static OT_RANK_STEP : i32 = 2;

#[derive(Clone, PartialEq)]
pub struct State {
    pub ot_ticks       : i32,
    pub lambda_position : Vector2f,
    pub lock_position  : Option<Vector2f>
}

pub struct Rush<'ot> {
    generator : XorShiftRng,

    ticks        : i32,
    last_fire    : i32,
    last_missile : i32,

    resource  : &'ot Resource<'ot>,

    lambda_bullets : Vec<Agent<'ot>>,
    bullets       : Vec<Agent<'ot>>,
    enemies       : Vec<Agent<'ot>>,
    effects       : Vec<Agent<'ot>>,
    sounds        : Vec<Sound<'ot>>,
    lambda         : Agent<'ot>,

    facing : f32,
    speed  : f32,

    backgrounds   : BTreeMap<i32, Background<'ot>>,
    overlays      : BTreeMap<i32, Background<'ot>>,

    score_display : Text<'ot>,
    rank_display  : Text<'ot>,
    multiplier_display  : Text<'ot>,

    card      : Option<Sprite<'ot>>,
    card_time : i32,

    score      : i32,
    multiplier : i32,

    rank : i32,

    unsafePerformIO : bool,
    last_rank : i32,
    hyper_start : i32,

    dead : bool
}

impl<'ot> Rush<'ot> {
    pub fn new(resource : &'ot Resource<'ot>)
            -> Rush<'ot> {

        let state = State {
            ot_ticks       : 0,
            lambda_position : Vector2f::new(240.0, 420.0),
            lock_position  : None
        };

        // First make lambda
        let lambda_idea = resource.get_idea("lambda");
        let ts = lambda_idea.texture_names.iter()
                    .map(|n| resource.get_texture(n))
                    .collect();
        let lambda = Agent::new_from_textures(
            &ts,
            resource.get_behaviour("lambda"),
            Vector2f::new(320.0, 240.0),
            lambda_idea.offset,
            Vector2f::new(0.0, 0.0),
            0.0,
            0.0,
            999999,
            0,
            &lambda_idea.body,
            state
        );

        let bg0 = Background::new_from_textures(
            &vec![resource.get_texture("background-sky")],
            12.0,
            true
        );

        let bg1 = Background::new_from_textures(
            &vec![resource.get_texture("background-stars-000")],
            3.0,
            true
        );

        let bg2 = Background::new_from_textures(
            &vec![resource.get_texture("background-stars-00")],
            2.4,
            true
        );

        let bg3 = Background::new_from_textures(
            &vec![resource.get_texture("background-stars-0")],
            1.8,
            true
        );

        let bg4 = Background::new_from_textures(
            &vec![resource.get_texture("background-stars-1")],
            1.6,
            false
        );

        let bg5 = Background::new_from_textures(
            &vec![resource.get_texture("background-stars-2")],
            1.2,
            true
        );

        let bg6 = Background::new_from_textures(
            &vec![resource.get_texture("background-cloud")],
            10.0,
            true
        );

        let mut bgs = BTreeMap::new();
        bgs.insert(0, bg0);
        bgs.insert(1, bg1);
        bgs.insert(2, bg2);
        bgs.insert(3, bg3);
        bgs.insert(4, bg4);
        bgs.insert(6, bg6);

        let mut sd = Text::new_init(
            "00",
            resource.get_font("serif"),
            24
        ).unwrap();
        sd.set_position2f(0.0, 450.0);

        let mut rd = Text::new_init(
            "10",
            resource.get_font("serif"),
            24
        ).unwrap();
        rd.set_position2f(0.0, 420.0);

        let mut md = Text::new_init(
            "1",
            resource.get_font("serif"),
            24
        ).unwrap();
        md.set_position2f(0.0, 390.0);

        let ol1 = Background::new_from_textures(
            &vec![resource.get_texture("overlay-unsafePerformIO")],
            1.0,
            true
        );

        let mut ol2 = Background::new_from_textures(
            &vec![resource.get_texture("overlay-unsafePerformIO")],
            -1.0,
            true
        );
        ol2.set_position(Vector2f::new(0.0, 416.0));

        let mut ols = BTreeMap::new();
        ols.insert(0, ol1);
        ols.insert(1, ol2);

        Rush {
            generator : XorShiftRng::from_seed(
                [7, 17, 27, 37]
            ),

            ticks : 0,
            last_fire : -666,
            last_missile : -666,

            resource  : resource,

            lambda_bullets : Vec::with_capacity(3000),
            bullets : Vec::with_capacity(3000),
            enemies : Vec::new(),
            effects : Vec::new(),
            sounds  : Vec::new(),
            lambda   : lambda,

            facing : 0.0,
            speed  : 1.0,

            backgrounds   : bgs,
            overlays      : ols,

            score_display : sd,
            rank_display : rd,
            multiplier_display : md,

            card      : None,
            card_time : 0,

            score      : 0,
            multiplier : 1,
            rank       : 10,

            unsafePerformIO : false,
            last_rank : 10,
            hyper_start : -666,

            dead : false
        }
    }

    pub fn is_dead(&self) -> bool {
        self.dead
    }

    pub fn tick(&mut self) {
        self.ticks += 1;
        if self.card_time > 0 {
            self.card_time -= 1;
        }
        self.lambda.tick();
        for b in self.lambda_bullets.iter_mut() {
            b.tick();
        }
        for b in self.bullets.iter_mut() {
            b.tick();
        }
        for e in self.enemies.iter_mut() {
            e.tick();
        }
        for e in self.effects.iter_mut() {
            e.tick();
        }
    }

    pub fn get_ticks(&self) -> i32 {
        self.ticks
    }

    pub fn get_hit(&self) -> i32 {
        self.lambda.get_life()
    }

    pub fn get_phantoms(&self) -> Vec<&Sprite> {
        let mut phantoms = Vec::new();
        for (_, bg) in self.backgrounds.iter() {
            phantoms.push(bg.get_phantom());
        }
        for b in self.lambda_bullets.iter() {
            phantoms.push(b.get_phantom());
        }
        for e in self.effects.iter() {
            phantoms.push(e.get_phantom());
        }
        for e in self.enemies.iter() {
            phantoms.push(e.get_phantom());
        }
        for b in self.bullets.iter() {
            phantoms.push(b.get_phantom());
        }
        phantoms.push(self.lambda.get_phantom());
        if self.unsafePerformIO {
            for (_, ol) in self.overlays.iter() {
                phantoms.push(ol.get_phantom());
            }
        }
        phantoms
    }

    pub fn get_shields(&self) -> Vec<CircleShape> {
        let mut phantoms = Vec::new();
        for e in self.enemies.iter() {
            if let Some(r) = self.spawn_shield(e) {
                phantoms.push(r);
            }
        }

        if let Some(r) = self.spawn_shield(&self.lambda) {
            phantoms.push(r);
        }

        phantoms
    }

    pub fn get_card(&self) -> Option<&Sprite> {
        if let Some(ref c) = self.card {
            Some(c)
        } else {
            None
        }
    }

    pub fn get_text(&self) -> Vec<&Text> {
        vec![&self.score_display, &self.rank_display, &self.multiplier_display]
    }

    pub fn take_action(&mut self, action : &Action) {
        self.spawn(
            &action.spawn_lambda_bullets, AgentType::RenkoBullet
        );
        self.spawn(
            &action.spawn_bullets, AgentType::Bullet
        );
        self.spawn(
            &action.spawn_enemies, AgentType::Enemy
        );
        self.spawn(
            &action.spawn_effects, AgentType::Effect
        );

        if let Some(n) = action.show_card {
            let t = self.resource.get_texture(n);
            let mut p = match Sprite::new_with_texture(t) {
                Some(r) => r,
                None    => panic!("Cannot create card.")
            };
            p.set_position(&DISPLAY_OFFSET);
            p.set_color(&Color::new_rgba(255, 255, 255, 0));
            self.card = Some(p);
            self.card_time = 360;
        }

        if action.explosion > 0 {
            for _ in 0..action.explosion {
                self.spawn_particle(action.position);
            }
        }

        self.score += self.multiplier * action.score;

        if self.multiplier > 0 {
            self.multiplier += action.multiplier;
        }
    }

    pub fn update(&mut self, command : Command) {
        // First update all agents
        let s = self.get_state();

        let mut actions = Vec::new();

        actions.append(&mut self.check_lambda_bullets());
        self.check_bullets();
        self.check_enemies();

        actions.append(
            &mut Rush::update_agents(
                &mut self.lambda_bullets,
                s.clone(),
                &mut self.generator
            )
        );
        actions.append(
            &mut Rush::update_agents(
                &mut self.bullets,
                s.clone(),
                &mut self.generator
            )
        );
        actions.append(
            &mut Rush::update_agents(
                &mut self.enemies,
                s.clone(),
                &mut self.generator
            )
        );
        actions.append(
            &mut Rush::update_agents(
                &mut self.effects,
                s.clone(),
                &mut self.generator
            )
        );

        self.lambda.update();
        actions.push(
            (self.lambda.get_behaviour())
                (&mut self.lambda, s.clone(), &mut self.generator)
        );

        for a in actions.iter() {
            self.take_action(a);
        }

        // Then fire bullets
        if command.z {
            self.fire_bullet();
        }
        if command.x {
            self.hyper();
        }

        if self.unsafePerformIO {
            self.fire_missile();
            if self.rank - self.last_rank > self.last_rank {
                self.rank = (self.last_rank / 4) * 3;
                self.unsafePerformIO = false;
                self.multiplier = 1;
            }
        }

        // Then move lambda
        self.move_lambda(&command);

        self.generate_enemies();

        self.update_card();

        self.update_background();
        self.update_overlay();

        self.update_score_display();
        self.update_rank_display();
        self.update_multiplier_display();

        if self.unsafePerformIO {
            self.rank += OT_RANK_STEP;
        } else if self.get_ticks() % 60 == 0 && self.rank > 10 {
            self.rank -= 1;
        }
    }

    pub fn clear(&mut self) {
        self.clear_agents();
        self.clear_sounds();

        self.clear_cards();
    }

    fn get_state(&self) -> State {
        let out = |p : &Vector2f| {
            p.x < 0.0 || p.y < 0.0 ||
            p.x > DISPLAY_WIDTH || p.y > DISPLAY_HEIGHT
        };
        let lp = self.enemies.iter()
            .map(Agent::get_position)
            .skip_while(out)
            .nth(0);
        State {
            ot_ticks       : self.ticks,
            lambda_position : self.lambda.get_position(),
            lock_position  : lp
        }
    }

    fn clear_agents(&mut self) {
        let cond = |a : &Agent| {
            let p = a.get_position();
               p.x > -MARGIN_WIDTH
            && p.x < DISPLAY_WIDTH + MARGIN_WIDTH
            && p.y > -MARGIN_WIDTH
            && p.y < DISPLAY_HEIGHT + MARGIN_WIDTH
            && !a.is_dead()
        };
        self.lambda_bullets.retain(&cond);
        self.bullets.retain(&cond);
        self.enemies.retain(&cond);
        self.effects.retain(&cond);
    }

    fn clear_sounds(&mut self) {
        self.sounds.retain(
            |s| s.get_status() == SoundStatus::Playing
        );
    }

    fn clear_cards(&mut self) {
        if self.card_time == 0 {
            self.card = None;
            self.card_time = -1;
        }
    }

    fn spawn(&mut self,
            rules       : &Vec<SpawnRule>,
            destination : AgentType) {

        let state = self.get_state();

        for r in (*rules).iter() {
            let beh = match r.behaviour_name {
                Some(n) => self.resource.get_behaviour(n),
                None    => behaviour::no_action
            };
            let idea = self.resource.get_idea(r.idea_name);
            let ts = idea.texture_names.iter()
                        .map(|n| self.resource.get_texture(n))
                        .collect();
            let b = Agent::new_from_textures(
                &ts,
                beh,
                r.position,
                idea.offset,
                r.velocity,
                r.angular,
                r.rotation,
                r.damage,
                r.life,
                &idea.body,
                state.clone()
            );
            match destination {
                AgentType::Renko       => self.enemies.push(b),
                AgentType::RenkoBullet => self.lambda_bullets.push(b),
                AgentType::Bullet      => self.bullets.push(b),
                AgentType::Enemy       => self.enemies.push(b),
                AgentType::Effect      => self.effects.push(b)
            };
        }
    }

    fn update_score_display(&mut self) {
        self.score_display.set_string(
            &(format!("SCORE: {:010}", self.score))
        );
    }

    fn update_rank_display(&mut self) {
        self.rank_display.set_string(
            &format!("RANK: {}", self.rank)
        );
    }

    fn update_multiplier_display(&mut self) {
        self.multiplier_display.set_string(
            &format!("MULT.: {}", self.multiplier)
        );
    }

    fn update_overlay(&mut self) {
        let mut x = false;
        for (_, ol)
                in self.overlays.iter_mut() {
            ol.update(Vector2f::new(4.0, 0.0));
        }
        let t = self.get_ticks();
    }

    fn update_background(&mut self) {
        let dp = polar_to_vector((self.speed, self.facing));
        for (_, bg)
                in self.backgrounds.iter_mut() {
            bg.update(dp);
        }
    }

    fn update_card(&mut self) {
        if let Some(ref mut c) = self.card {
            let a = (self.card_time as f32 / 2.0).to_radians().sin() * 255.0;
            c.set_color(&Color::new_rgba(255, 255, 255, a as u8));
        }
    }

    fn update_agents(agents : &mut Vec<Agent>, state : State, rng : &mut XorShiftRng)
            -> Vec<Action> {
        agents.iter_mut()
            .map(|a| { a.update(); a.act(state.clone(), rng) })
            .collect()
    }

    fn check_lambda_bullets(&mut self) -> Vec<Action> {
        let mut hit = false;
        let mut actions = Vec::new();
        for b in self.lambda_bullets.iter_mut() {
            for e in self.enemies.iter_mut() {
                if e.get_body().collide(b.get_body()) {
                    let f = Force::Damage(b.get_damage());
                    actions.push(e.take_force(&f, &mut self.generator));
                    b.terminate();
                    hit = true;
                }
            }
        }

        if hit {
            self.rank += 1;
        }

        actions
    }

    fn check_bullets(&mut self) {
        let mut hit = false;
        let body = self.lambda.get_body();
        for b in self.bullets.iter() {
            if b.get_body().collide(body) {
                self.lambda.take_force(
                    &Force::Damage(1),
                    &mut self.generator
                );
                hit = true;
            }
        }

        if hit && self.lambda.get_shield() == 0 {
            self.lambda.set_shield(120);
            self.dead = true;
        }
    }

    fn check_enemies(&mut self) {
        let mut hit = false;
        let body = self.lambda.get_body();
        for e in self.enemies.iter() {
            if e.get_body().collide(body) {
                self.lambda.take_force(
                    &Force::Damage(1),
                    &mut self.generator
                );
                hit = true;
            }
        }

        if hit && self.lambda.get_shield() == 0 {
            self.lambda.set_shield(120);
            self.dead = true;
        }
    }

    fn hyper(&mut self) {
        if !self.unsafePerformIO && self.rank > 80 {
            self.unsafePerformIO = true;
            self.last_rank = self.rank;
            self.multiplier = 0;
            self.hyper_start = self.get_ticks();
        }
    }

    fn fire_bullet(&mut self) {
        let state = self.get_state();

        let t = self.get_ticks();

        if t - self.last_fire > OT_FIRE_INTERVAL {
            let idea = self.resource.get_idea("lambda-bullet-wave");
            let ts = idea.texture_names.iter()
                        .map(|n| self.resource.get_texture(n))
                        .collect();
            let dy = Vector2f::new(0.0, 20.0);
            let b = Agent::new_from_textures(
                &ts,
                self.resource.get_behaviour("lambda-bullet"),
                self.lambda.get_position(),
                idea.offset,
                polar_to_vector((12.0 + self.speed, self.facing + 180.0)),
                0.0,
                self.facing,
                10,
                9999999,
                &idea.body,
                state.clone()
            );
            self.lambda_bullets.push(b);
            self.last_fire = self.get_ticks();
        }
    }

    fn fire_missile(&mut self) {

        let state = self.get_state();

        let t = self.get_ticks();

        if t - self.last_missile > OT_MISSILE_INTERVAL {
            let idea = self.resource.get_idea("lambda-bullet-wave");
            let ts = idea.texture_names.iter()
                        .map(|n| self.resource.get_texture(n))
                        .collect();
            let dy = Vector2f::new(0.0, 20.0);
            for i in 0..36 {
                let a = i as f32 * 10.0;
                let b = Agent::new_from_textures(
                    &ts,
                    self.resource.get_behaviour("lambda-bullet"),
                    self.lambda.get_position(),
                    idea.offset,
                    polar_to_vector((12.0 + self.speed, self.facing + a)),
                    0.0,
                    180.0 + self.facing + a,
                    10,
                    9999999,
                    &idea.body,
                    state.clone()
                );
                self.lambda_bullets.push(b);
            }
            self.last_missile = self.get_ticks();
        }
    }

    fn move_lambda(&mut self, command : &Command) {
        let (acc, ang) = if command.shift {
            (OT_LAMBDA_ACC_LOW, 1.8)
        } else {
            (OT_LAMBDA_ACC_HIGH, 4.5)
        };

        let v = self.lambda.get_velocity();

        let da = match command.direction {
            Direction::Left  => ang,
            Direction::Right => -ang,
            Direction::Front => 0.0
        };

        let ds = match command.thrust {
            Thrust::Accelerate => acc,
            Thrust::Slow       => -acc,
            Thrust::Hold       => 0.0
        };

        self.facing += da;
        self.speed += ds;
        if self.speed > OT_LAMBDA_MAX_SPEED {
            self.speed = OT_LAMBDA_MAX_SPEED;
        } else if self.speed < OT_LAMBDA_MIN_SPEED {
            self.speed = OT_LAMBDA_MIN_SPEED;
        }
        self.lambda.set_rotation(self.facing);

        let dp = polar_to_vector((self.speed, self.facing));

        for a in self.lambda_bullets.iter_mut() {
            a.move_amount(dp);
        }
        for a in self.bullets.iter_mut() {
            a.move_amount(dp);
        }
        for a in self.enemies.iter_mut() {
            a.move_amount(dp);
        }
        for a in self.effects.iter_mut() {
            a.move_amount(dp);
        }

        self.speed *= 0.95;
    }

    fn spawn_particle(&mut self, position : Vector2f) {
        let state = self.get_state();

        let idea = self.resource.get_idea("particle");
        for _ in 0..self.generator.gen_range(6, 12) {
            let t = self.resource.get_texture(idea.texture_names[0]);
            let mut e = Agent::new_from_texture(
                t,
                self.resource.get_behaviour("particle"),
                position,
                idea.offset,
                polar_to_vector((
                    self.generator.gen_range(8.0, 32.0),
                    self.generator.gen_range(0.0, 360.0)
                )),
                self.generator.gen_range(-4.0, 4.0),
                self.generator.gen_range(0.0, 360.0),
                0,
                self.generator.gen_range(12, 24),
                &idea.body,
                state.clone()
            );
            let s = self.generator.gen_range(0.75, 1.0);
            e.set_scale(Vector2f::new(s, s));
            self.effects.push(e);
        }
    }

    fn spawn_shield(&self, agent : &Agent) -> Option<CircleShape> {
        let s = agent.get_shield();
        if s > 60 || (s / 10) % 2 == 0 && s > 0 {
            let mut p = CircleShape::new_with_texture(
                self.resource.get_texture("shield")
            ).unwrap();
            let r = agent.get_size().ln() * 24.0;
            p.set_radius(r);
            p.set_origin2f(r, r);
            p.set_position(
                &(agent.get_position() + DISPLAY_OFFSET)
            );
            Some(p)
        } else {
            None
        }
    }

    fn generate_enemies(&mut self) {

        let state = self.get_state();

        let position = match self.generator.gen_range(0.0, 1.0) {
            x if x < 0.25 => Vector2f::new(
                -60.0,
                self.generator.gen_range(0.0, DISPLAY_HEIGHT)
            ),
            x if x < 0.50 => Vector2f::new(
                DISPLAY_WIDTH + 60.0,
                self.generator.gen_range(0.0, DISPLAY_HEIGHT)
            ),
            x if x < 0.75 => Vector2f::new(
                self.generator.gen_range(0.0, DISPLAY_WIDTH),
                -60.0
            ),
            _ => Vector2f::new(
                self.generator.gen_range(0.0, DISPLAY_WIDTH),
                DISPLAY_HEIGHT + 60.0
            ),
        };

        if self.generator.gen_range(0, 910 + self.rank / OT_RANK_STEP) > 900
            && self.enemies.len() < 40 {
            let idea = self.resource.get_idea("enemy-lisp");
            let ts = idea.texture_names.iter()
                .map(|n| self.resource.get_texture(n))
                .collect();

            let mut e = Agent::new_from_textures(
                &ts,
                self.resource.get_behaviour("enemy-lisp"),
                position,
                idea.offset,
                polar_to_vector((
                    0.4,
                    self.generator.gen_range(0.0, 360.0)
                )),
                self.generator.gen_range(-0.8, 0.8),
                self.generator.gen_range(0.0, 360.0),
                0,
                200,
                &idea.body,
                state.clone()
            );
            self.enemies.push(e);
        }
    }
}
