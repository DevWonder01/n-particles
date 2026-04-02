mod n_body;

use n_body::simulate::Simulation;
use n_body::vec2::Vec2;

use macroquad::prelude::*;

#[macroquad::main("N-Particles Simulation")]
async fn main() {
    let background_color = Color::from_rgba(18, 18, 18, 255);

    let mut sim = Simulation::new();
    let center_x = screen_width() as f64 / 2.0;
    let center_y = screen_height() as f64 / 2.0;
    let g_from_sim = 5.0; // Matching G from simulate.rs

    // 1. Solid Central Body (Sun)
    let mut sun = Vec2::new(center_x, center_y);
    sun.mass = 100.0;
    sim.bodies.push(sun);

    // 2. Orbits (Planets)
    for _ in 0..100 {
        let angle = fastrand::f64() * std::f64::consts::TAU;
        let dist = fastrand::f64() * 300.0 + 100.0; // Distance from center

        let p_x = center_x + angle.cos() * dist;
        let p_y = center_y + angle.sin() * dist;

        let mut p = Vec2::new(p_x, p_y);
        p.mass = fastrand::f64() * 5.0 + 1.0;

        // Tangential velocity formula: sqrt(G * M / R)
        let orb_speed = (g_from_sim * sun.mass / dist).sqrt();
        p.vel = [-angle.sin() * orb_speed, angle.cos() * orb_speed];

        sim.bodies.push(p);
    }

    loop {
        clear_background(background_color);

        // 1. Calculate gravity
        sim.gravitational_attration();

        // 2. Resolve collisions between particles
        sim.quad_tree_collision(screen_width(), screen_height());

        for p in &mut sim.bodies {
            // 3. Movement
            p.update();

            // 4. Stay inside screen bounds
            p.keep_in_bounds(screen_width() as f64, screen_height() as f64);

            // 5. Render
            p.draw();
        }

        next_frame().await;
    }
}
