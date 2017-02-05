// Orbis Tertius

extern crate rush;

extern crate sfml;

use std::time::*;

use sfml::system::*;
use sfml::window::*;
use sfml::graphics::*;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum GameState {
    Start, Running, End
}

fn main() {
    println!("hello");

    let resource = rush::Resource::new();

    let mut window = match RenderWindow::new(
        VideoMode::new_init(640, 480, 32),
        "LAMBDA RUSH",
        window_style::CLOSE,
        &ContextSettings::default()
    ) {
        Some(w) => w,
        None    => panic!("Cannot initialise window.")
    };

    window.set_vertical_sync_enabled(true);

    let mut fps_counter = match Text::new_init(
        "",
        resource.get_font("serif"),
        20
    ) {
        Some(r) => r,
        None    => panic!("Cannot create text.")
    };
    let mut draw_counter = match Text::new_init(
        "",
        resource.get_font("serif"),
        20
    ) {
        Some(r) => r,
        None    => panic!("Cannot create text.")
    };
    let mut hit_counter = match Text::new_init(
        "",
        resource.get_font("serif"),
        20
    ) {
        Some(r) => r,
        None    => panic!("Cannot create text.")
    };

    fps_counter.set_position2f(0.0, 0.0);
    fps_counter.set_string("");
    draw_counter.set_position2f(80.0, 0.0);
    draw_counter.set_string("");
    hit_counter.set_position2f(0.0, 360.0);
    hit_counter.set_string("");

    let mut last = Instant::now();
    let mut frame = 0;

    let mut title = Sprite::new_with_texture(
        resource.get_texture("title")
    ).unwrap();
    let mut dead = Sprite::new_with_texture(
        resource.get_texture("dead")
    ).unwrap();

    let mut rush = rush::Rush::new(&resource);
    let mut render_texture = RenderTexture::new(640, 640, false).unwrap();
    let mut state = GameState::Start;

    while window.is_open() {

        if Key::Escape.is_pressed() {
            window.close();
        }

        for e in window.events() {
            match e {
                event::KeyPressed {..} => {
                    match state {
                        GameState::Start => state = GameState::Running,
                        _ => ()
                    }
                },
                event::Closed => window.close(),
                _             => ()
            }
        }

        let d = {
            if Key::Left.is_pressed() {
                rush::Direction::Left
            } else if Key::Right.is_pressed() {
                rush::Direction::Right
            } else {
                rush::Direction::Front
            }
        };

        let t = {
            if Key::Up.is_pressed() {
                rush::Thrust::Accelerate
            } else if Key::Down.is_pressed() {
                rush::Thrust::Slow
            } else {
                rush::Thrust::Hold
            }
        };

        let cmd = rush::Command {
            direction : d,
            thrust    : t,
            z         : Key::Z.is_pressed(),
            x         : Key::X.is_pressed(),
            shift     : Key::LShift.is_pressed()
        };

        if let GameState::Running = state {
            rush.update(cmd);
        }
        if rush.is_dead() {
            state = GameState::End;
        }
        rush.clear();

        rush.tick();

        // Finally set the frame counter
        let dt = Instant::now().duration_since(last);
        if dt >= Duration::new(1, 0) {
            fps_counter.set_string(&(frame.to_string()));
            frame = 0;
            last = Instant::now();
        }
        frame += 1;

        // And then draw everything
        window.clear(&Color::new_rgb(32, 32, 32));
        render_texture.clear(&Color::new_rgb(32, 32, 32));

        render_texture.set_active(true);
        let mut d : i32 = 0;
        for p in rush.get_phantoms().iter() {
            render_texture.draw(*p);
            d += 1;
        }
        for p in rush.get_shields().iter() {
            render_texture.draw(p);
            d += 1;
        }
        render_texture.set_active(false);

        draw_counter.set_string(&(d.to_string()));

        // Render the world
        let view = render_texture.get_texture().unwrap();
        let mut slice = Sprite::new().unwrap();
        slice.set_texture(&view, false);
        slice.set_scale2f(1.0, -1.0);
        slice.set_position2f(0.0, 640.0);
        window.draw(&slice);

        if let Some(c) = rush.get_card() {
            window.draw(c);
        }

        match state {
            GameState::Start   => window.draw(&title),
            GameState::Running => (),
            GameState::End     => window.draw(&dead)
        }
        for t in rush.get_text().iter() {
            window.draw(*t);
        }

        window.draw(&fps_counter);

        window.display();
    }
}
