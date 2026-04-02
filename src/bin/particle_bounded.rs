use n_particles::n_body::simulate::Simulation;
use n_particles::n_body::vec2::Vec2;

use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "N-Particles Simulation".to_owned(),
        fullscreen: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let background_color = Color::from_rgba(18, 18, 18, 255);

    let mut sim = Simulation::new();
    // Create random particles
    for _ in 0..10 {
        let mut p = Vec2::new(
            fastrand::f64() * screen_width() as f64,
            fastrand::f64() * screen_height() as f64,
        );
        p.mass = fastrand::f64() * 20.0 + 1.0;
        // Add random initial velocity for more dynamic orbits
        p.vel = [(fastrand::f64() - 0.5) * 2.0, (fastrand::f64() - 0.5) * 2.0];
        sim.bodies.push(p);
    }

    loop {
        clear_background(background_color);

        // 1. Calculate gravity
        sim.gravitational_attration();

        // 2. Resolve collisions between particles (Optimized with QuadTree)
        sim.quad_tree_collision(screen_width(), screen_height());

        for p in &mut sim.bodies {
            // 3. Movement
            p.update();

            // 4. Stay inside screen bounds
            // p.keep_in_bounds(screen_width() as f64, screen_height() as f64);

            // 5. Render
            p.draw();
        }

        next_frame().await;
    }
}
