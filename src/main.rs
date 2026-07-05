mod n_body;

use n_body::simulate::Simulation;
use n_body::vec3::Vec3;

use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "N-Particles 3D Simulation".to_owned(),
        window_width: 800,
        window_height: 800,
        fullscreen: false,
        ..Default::default()
    }
}



#[macroquad::main(window_conf)]
async fn main() {
    let background_color = Color::from_rgba(10, 10, 15, 255);

    let mut sim = Simulation::new();
    let g_from_sim = 5.0; // Matching G from simulate.rs
    let box_size = 500.0;
    let half_box = box_size / 2.0;

    // 1. Solid Central Body (Sun)
    let mut sun = Vec3::new(0.0, 0.0, 0.0);
    sun.mass = 200.0;
    sun.fixed = true;
    sim.add_body(sun);

    // 2. Orbits (Planets) in 3D
    for _ in 0..15 {
        // Random distance from central sun
        let dist = fastrand::f64() * 150.0 + 50.0;
        // Random point on sphere of radius `dist`
        let theta = fastrand::f64() * std::f64::consts::TAU;
        let phi = (fastrand::f64() - 0.5) * std::f64::consts::PI;

        let p_x = dist * phi.cos() * theta.cos();
        let p_y = dist * phi.sin();
        let p_z = dist * phi.cos() * theta.sin();

        let mut p = Vec3::new(p_x, p_y, p_z);
        p.mass = fastrand::f64() * 8.0 + 1.0;

        // Tangential velocity in 3D:
        let nx = fastrand::f64() - 0.5;
        let ny = fastrand::f64() - 0.5;
        let nz = fastrand::f64() - 0.5;
        let n_len = (nx*nx + ny*ny + nz*nz).sqrt();
        let (nx, ny, nz) = (nx/n_len, ny/n_len, nz/n_len);

        let mut vx = ny * p_z - nz * p_y;
        let mut vy = nz * p_x - nx * p_z;
        let mut vz = nx * p_y - ny * p_x;
        let v_len = (vx*vx + vy*vy + vz*vz).sqrt();
        if v_len > 0.0 {
            vx /= v_len;
            vy /= v_len;
            vz /= v_len;
        }

        let orb_speed = (g_from_sim * sun.mass / dist).sqrt();
        p.vel = [vx * orb_speed, vy * orb_speed, vz * orb_speed];

        sim.add_body(p);
    }

    // Camera variables
    let mut camera_yaw: f32 = 0.0;
    let mut camera_pitch: f32 = 0.2;
    let mut camera_dist: f32 = 600.0;
    let mut last_mouse_pos = mouse_position();

    loop {
        clear_background(background_color);

        // --- Camera Controls ---
        let current_mouse_pos = mouse_position();
        if is_mouse_button_down(MouseButton::Left) {
            let dx = current_mouse_pos.0 - last_mouse_pos.0;
            let dy = current_mouse_pos.1 - last_mouse_pos.1;
            camera_yaw += dx * 0.005;
            camera_pitch = (camera_pitch + dy * 0.005).clamp(-1.4, 1.4);
        }
        last_mouse_pos = current_mouse_pos;

        // Zoom with scroll wheel
        let wheel = mouse_wheel();
        camera_dist = (camera_dist - wheel.1 * 30.0).clamp(100.0, 1500.0);

        // Rotate yaw slowly over time if not dragging
        if !is_mouse_button_down(MouseButton::Left) {
            camera_yaw += 0.002;
        }

        // Calculate camera position
        let cam_x = camera_dist * camera_pitch.cos() * camera_yaw.sin();
        let cam_y = camera_dist * camera_pitch.sin();
        let cam_z = camera_dist * camera_pitch.cos() * camera_yaw.cos();

        let camera = Camera3D {
            position: vec3(cam_x, cam_y, cam_z),
            target: vec3(0.0, 0.0, 0.0),
            up: vec3(0.0, 1.0, 0.0),
            fovy: 45.0,
            ..Default::default()
        };

        // --- Physics Step ---
        sim.update_physics(true, half_box);

        // --- Render 3D Scene ---
        set_camera(&camera);

        // Draw Bounding Cube Wireframe
        let c_box = Color::from_rgba(60, 60, 80, 255);
        let h_box = half_box as f32;
        let corners = [
            vec3(-h_box, -h_box, -h_box),
            vec3( h_box, -h_box, -h_box),
            vec3( h_box,  h_box, -h_box),
            vec3(-h_box,  h_box, -h_box),
            vec3(-h_box, -h_box,  h_box),
            vec3( h_box, -h_box,  h_box),
            vec3( h_box,  h_box,  h_box),
            vec3(-h_box,  h_box,  h_box),
        ];
        let edges = [
            (0, 1), (1, 2), (2, 3), (3, 0),
            (4, 5), (5, 6), (6, 7), (7, 4),
            (0, 4), (1, 5), (2, 6), (3, 7),
        ];
        for &(i, j) in &edges {
            draw_line_3d(corners[i], corners[j], c_box);
        }

        // Draw Grid at the bottom of the box for perspective
        let grid_y = -h_box;
        let grid_step = box_size as f32 / 10.0;
        for i in 0..=10 {
            let offset = -h_box + i as f32 * grid_step;
            draw_line_3d(vec3(offset, grid_y, -h_box), vec3(offset, grid_y, h_box), Color::from_rgba(40, 40, 50, 150));
            draw_line_3d(vec3(-h_box, grid_y, offset), vec3(h_box, grid_y, offset), Color::from_rgba(40, 40, 50, 150));
        }

        // Draw Trails
        for (i, path) in sim.paths.iter().enumerate() {
            if i == 0 {
                continue; // Sun remains stationary
            }
            if path.len() < 2 {
                continue;
            }
            for w in 0..path.len() - 1 {
                let p1 = vec3(path[w][0], path[w][1], path[w][2]);
                let p2 = vec3(path[w+1][0], path[w+1][1], path[w+1][2]);

                let alpha = (w as f32 / path.len() as f32) * 0.65;
                let mut c = Color::from_rgba(100, 200, 255, 255);
                c.a = alpha;

                draw_line_3d(p1, p2, c);
            }
        }

        // Draw Bodies
        for (i, p) in sim.bodies.iter().enumerate() {
            if i == 0 {
                // Sun
                draw_sphere(
                    vec3(p.x() as f32, p.y() as f32, p.z() as f32),
                    p.radius() as f32,
                    None,
                    Color::from_rgba(255, 200, 50, 255),
                );
            } else {
                p.draw();
            }
        }

        set_default_camera();

        // Draw Overlay Info
        draw_text("N-Particles 3D Simulation", 20.0, 30.0, 24.0, WHITE);
        draw_text("Drag with Left Mouse Button to Rotate", 20.0, 55.0, 18.0, GRAY);
        draw_text("Scroll to Zoom", 20.0, 75.0, 18.0, GRAY);
        draw_text(&format!("Bodies: {}", sim.bodies.len()), 20.0, 105.0, 18.0, WHITE);
        draw_text(&format!("FPS: {:.0}", get_fps()), 20.0, 125.0, 18.0, WHITE);

        next_frame().await;
    }
}
