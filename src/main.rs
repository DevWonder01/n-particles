mod n_body;

use n_body::simulate::Simulation;
use n_body::vec2::Vec2;


use macroquad::prelude::*;



#[macroquad::main("N-Particles Simulation")]
async fn main() {
    let background_color = Color::from_rgba(18, 18, 18, 255);

    let mut sim = Simulation::new();
    // Create random particles
    for _ in 0..3 {
        let mut p = Vec2::new(
            fastrand::f64() * screen_width() as f64,
            fastrand::f64() * screen_height() as f64,
        );
        p.mass = fastrand::f64() * 20.0 + 5.0;
        // Add random initial velocity for more dynamic orbits
        p.vel = [
            (fastrand::f64() - 0.5) * 2.0,
            (fastrand::f64() - 0.5) * 2.0,
        ];
        sim.bodies.push(p);
    }

    loop {
        clear_background(background_color);

        sim.gravitation_attration();

        for p in &mut sim.bodies {
            p.update();

            // Render
            p.draw();
        }

        next_frame().await;
    }
}
