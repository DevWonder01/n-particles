use n_particles::n_body::octree;
use n_particles::n_body::simulate::Simulation;
use n_particles::n_body::vec3::Vec3;

use macroquad::prelude::*;

/// Fixed width of the egui sidebar (pixels).
const PANEL_W: f32 = 280.0;

fn window_conf() -> Conf {
    Conf {
        window_title: "N-Particles 3D Simulation".to_owned(),
        window_width: PANEL_W as i32 + 700, // sidebar + 700×700 sim area
        window_height: 800,
        fullscreen: false,
        ..Default::default()
    }
}

// ── Helpers ──────────────────────────────────────────────────────────────

/// Spawn `count` random particles.
fn spawn_particles(sim: &mut Simulation, count: usize, mass_min: f64, mass_max: f64, box_size: f64, has_sun: bool) {
    let half_box = box_size / 2.0;
    for _ in 0..count {
        if has_sun && !sim.bodies.is_empty() {
            let sun_mass = sim.bodies[0].mass;
            let dist = fastrand::f64() * (half_box * 0.7) + 20.0;
            let theta = fastrand::f64() * std::f64::consts::TAU;
            let phi = (fastrand::f64() - 0.5) * std::f64::consts::PI;

            let p_x = dist * phi.cos() * theta.cos();
            let p_y = dist * phi.sin();
            let p_z = dist * phi.cos() * theta.sin();

            let mut p = Vec3::new(p_x, p_y, p_z);
            p.mass = fastrand::f64() * (mass_max - mass_min) + mass_min;

            // Randomized orbit normal
            let nx = fastrand::f64() - 0.5;
            let ny = fastrand::f64() - 0.5;
            let nz = fastrand::f64() - 0.5;
            let n_len = (nx*nx + ny*ny + nz*nz).sqrt();
            let (nx, ny, nz) = (nx/n_len, ny/n_len, nz/n_len);

            // Velocity direction = normal x position
            let mut vx = ny * p_z - nz * p_y;
            let mut vy = nz * p_x - nx * p_z;
            let mut vz = nx * p_y - ny * p_x;
            let v_len = (vx*vx + vy*vy + vz*vz).sqrt();
            if v_len > 0.0 {
                vx /= v_len;
                vy /= v_len;
                vz /= v_len;
            }

            let orb_speed = (5.0 * sun_mass / dist).sqrt(); // G = 5.0
            p.vel = [vx * orb_speed, vy * orb_speed, vz * orb_speed];
            sim.add_body(p);
        } else {
            let mut p = Vec3::new(
                (fastrand::f64() - 0.5) * box_size,
                (fastrand::f64() - 0.5) * box_size,
                (fastrand::f64() - 0.5) * box_size,
            );
            p.mass = fastrand::f64() * (mass_max - mass_min) + mass_min;
            p.vel = [
                (fastrand::f64() - 0.5) * 4.0,
                (fastrand::f64() - 0.5) * 4.0,
                (fastrand::f64() - 0.5) * 4.0,
            ];
            sim.add_body(p);
        }
    }
}

/// Create a fixed central body (sun) at the origin.
fn spawn_sun(sim: &mut Simulation, sun_mass: f64) {
    let mut sun = Vec3::new(0.0, 0.0, 0.0);
    sun.mass = sun_mass;
    sun.fixed = true;
    sim.bodies.insert(0, sun);
    sim.paths.insert(0, Vec::new());
}



// ── Main ─────────────────────────────────────────────────────────────────

#[macroquad::main(window_conf)]
async fn main() {
    let bg = Color::from_rgba(10, 10, 15, 255);

    // ── Simulation state ──────────────────────────────────────────────
    let mut sim = Simulation::new();
    let initial_count: usize = 3;
    let mut spawn_count: f32 = initial_count as f32;
    let mut mass_min: f32 = 1.0;
    let mut mass_max: f32 = 15.0;
    let mut paused = false;
    let mut show_octree = false;
    let mut show_paths = true;
    let mut auto_rotate = true;
    let mut bounded = false;
    let box_size = 500.0;
    let half_box = box_size / 2.0;

    // ── Central body (sun) ────────────────────────────────────────────
    let mut has_sun = true;
    let mut sun_mass: f32 = 500.0;
    spawn_sun(&mut sim, sun_mass as f64);

    // Spawn initial orbiting particles
    spawn_particles(&mut sim, initial_count, mass_min as f64, mass_max as f64, box_size, has_sun);

    // ── Energy history for the live plot ───────────────────────────────
    let mut ke_history: Vec<f64> = Vec::new();
    let max_history: usize = 600;
    let mut frame_counter: usize = 0;

    // Camera variables
    let mut camera_yaw: f32 = 0.0;
    let mut camera_pitch: f32 = 0.2;
    let mut camera_dist: f32 = 650.0;
    let mut last_mouse_pos = mouse_position();

    loop {
        clear_background(bg);

        // --- Camera Orbit Logic ---
        let current_mouse_pos = mouse_position();
        if is_mouse_button_down(MouseButton::Left) && current_mouse_pos.0 > PANEL_W {
            let dx = current_mouse_pos.0 - last_mouse_pos.0;
            let dy = current_mouse_pos.1 - last_mouse_pos.1;
            camera_yaw += dx * 0.005;
            camera_pitch = (camera_pitch + dy * 0.005).clamp(-1.4, 1.4);
        }
        last_mouse_pos = current_mouse_pos;

        // Zoom with scroll wheel if mouse is in the simulation area
        if current_mouse_pos.0 > PANEL_W {
            let wheel = mouse_wheel();
            camera_dist = (camera_dist - wheel.1 * 30.0).clamp(100.0, 1500.0);
        }

        // Slowly rotate yaw if not dragging and auto-rotate is enabled
        if auto_rotate && !is_mouse_button_down(MouseButton::Left) && !paused {
            camera_yaw += 0.002;
        }

        // Calculate camera position
        let cam_x = camera_dist * camera_pitch.cos() * camera_yaw.sin();
        let cam_y = camera_dist * camera_pitch.sin();
        let cam_z = camera_dist * camera_pitch.cos() * camera_yaw.cos();

        // Viewport to the right of the egui panel
        let view_w = (screen_width() - PANEL_W).max(10.0);
        let view_h = screen_height().max(10.0);

        let camera = Camera3D {
            position: vec3(cam_x, cam_y, cam_z),
            target: vec3(0.0, 0.0, 0.0),
            up: vec3(0.0, 1.0, 0.0),
            fovy: 45.0,
            aspect: Some(view_w / view_h),
            viewport: Some((PANEL_W as i32, 0, view_w as i32, view_h as i32)),
            ..Default::default()
        };

        // ── Physics step ──────────────────────────────────────────────
        if !paused {
            sim.update_physics(bounded, half_box);
        }

        // ── Kinetic energy ────────────────────────────────────────────
        let total_ke: f64 = sim
            .bodies
            .iter()
            .map(|p| 0.5 * p.mass * (p.vel[0].powi(2) + p.vel[1].powi(2) + p.vel[2].powi(2)))
            .sum();
        ke_history.push(total_ke);
        if ke_history.len() > max_history {
            ke_history.remove(0);
        }
        frame_counter += 1;

        // ── Render 3D Scene ───────────────────────────────────────────
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

        // ── Octree visualisation ──────────────────────────────────────
        if show_octree {
            let boundary = octree::Node::new(Vec3::new(0.0, 0.0, 0.0), box_size);
            let mut ot = octree::Octree::new(boundary, 2);
            for i in 0..sim.bodies.len() {
                ot.insert(i, &sim.bodies);
            }
            ot.draw();
        }

        // ── Render Trails/Paths ───────────────────────────────────────
        if show_paths {
            for (i, path) in sim.paths.iter().enumerate() {
                if i == 0 && has_sun {
                    continue; // Sun remains stationary
                }
                if path.len() < 2 {
                    continue;
                }
                for w in 0..path.len() - 1 {
                    let p1 = vec3(path[w][0], path[w][1], path[w][2]);
                    let p2 = vec3(path[w+1][0], path[w+1][1], path[w+1][2]);

                    let alpha = (w as f32 / path.len() as f32) * 0.65;
                    let mut c = Color::from_rgba(100, 200, 255, 255); // Cyan trail
                    c.a = alpha;

                    draw_line_3d(p1, p2, c);
                }
            }
        }

        // ── Render particles ──────────────────────────────────────────
        for (i, p) in sim.bodies.iter().enumerate() {
            if i == 0 && has_sun {
                let r = p.radius();
                draw_sphere(
                    vec3(p.x() as f32, p.y() as f32, p.z() as f32),
                    r as f32,
                    None,
                    Color::from_rgba(255, 200, 50, 255),
                );
            } else {
                p.draw();
            }
        }

        set_default_camera();

        // ── Draw a subtle border between panel and sim area ───────────
        draw_line(
            PANEL_W,
            0.0,
            PANEL_W,
            screen_height(),
            1.0,
            Color::from_rgba(60, 60, 80, 255),
        );

        // ── egui sidebar ──────────────────────────────────────────────
        egui_macroquad::ui(|egui_ctx| {
            let mut visuals = egui::Visuals::dark();
            visuals.panel_fill = egui::Color32::from_rgb(10, 10, 14);
            visuals.window_fill = egui::Color32::from_rgb(10, 10, 14);
            egui_ctx.set_visuals(visuals);

            egui::SidePanel::left("control_panel")
                .exact_width(PANEL_W)
                .resizable(false)
                .show(egui_ctx, |ui| {
                    ui.heading("⚛  3D N-Particles");
                    ui.separator();

                    // ── Info ───────────────────────────────────────────
                    ui.label(format!("Bodies: {}", sim.bodies.len()));
                    ui.label(format!("FPS: {:.0}", get_fps()));
                    ui.label(format!("KE: {:.1}", total_ke));
                    ui.separator();

                    // ── Central Body (Sun) ─────────────────────────────
                    ui.collapsing("Central Body", |ui| {
                        let prev_has_sun = has_sun;
                        ui.checkbox(&mut has_sun, "Enable Sun");

                        if has_sun {
                            ui.add(
                                egui::Slider::new(&mut sun_mass, 10.0..=10000.0)
                                    .text("Sun Mass")
                                    .logarithmic(true),
                            );

                            // Live-update the sun
                            if !sim.bodies.is_empty() && sim.bodies[0].fixed {
                                sim.bodies[0].mass = sun_mass as f64;
                                sim.bodies[0].particle = [0.0, 0.0, 0.0];
                            }

                            // Sun was just toggled ON
                            if !prev_has_sun {
                                spawn_sun(&mut sim, sun_mass as f64);
                            }
                        } else if prev_has_sun {
                            if !sim.bodies.is_empty() && sim.bodies[0].fixed {
                                sim.remove_body(0);
                            }
                        }
                    });
                    ui.separator();

                    // ── Spawn Controls ─────────────────────────────────
                    ui.collapsing("Spawn Settings", |ui| {
                        ui.add(
                            egui::Slider::new(&mut spawn_count, 1.0..=200.0)
                                .text("Count")
                                .logarithmic(true),
                        );
                        ui.add(egui::Slider::new(&mut mass_min, 0.1..=50.0).text("Mass min"));
                        ui.add(egui::Slider::new(&mut mass_max, 1.0..=100.0).text("Mass max"));
                    });
                    ui.separator();

                    // ── Bounded Space ──────────────────────────────────
                    ui.checkbox(&mut bounded, "Bounded Space");
                    ui.separator();

                    // ── Actions ────────────────────────────────────────
                    ui.horizontal(|ui| {
                        if ui
                            .button(if paused { "▶ Resume" } else { "⏸ Pause" })
                            .clicked()
                        {
                            paused = !paused;
                        }
                        if ui.button("🔄 Reset").clicked() {
                            sim.clear();
                            ke_history.clear();
                            frame_counter = 0;
                            if has_sun {
                                spawn_sun(&mut sim, sun_mass as f64);
                            }
                            spawn_particles(
                                &mut sim,
                                spawn_count as usize,
                                mass_min as f64,
                                mass_max as f64,
                                box_size,
                                has_sun,
                            );
                        }
                    });

                    if ui.button("➕ Add Particles").clicked() {
                        spawn_particles(
                            &mut sim,
                            spawn_count as usize,
                            mass_min as f64,
                            mass_max as f64,
                            box_size,
                            has_sun,
                        );
                    }

                    if ui.button("💥 Clear All").clicked() {
                        sim.clear();
                        ke_history.clear();
                        frame_counter = 0;
                        if has_sun {
                            spawn_sun(&mut sim, sun_mass as f64);
                        }
                    }

                    ui.separator();
                    ui.checkbox(&mut show_octree, "Show Octree");
                    ui.checkbox(&mut show_paths, "Show Trails");
                    ui.checkbox(&mut auto_rotate, "Auto Rotate Camera");

                    ui.separator();

                    // ── Live Kinetic Energy Plot ───────────────────────
                    ui.label("Kinetic Energy");
                    let points: egui_plot::PlotPoints = ke_history
                        .iter()
                        .enumerate()
                        .map(|(i, &ke)| {
                            let x = (frame_counter as f64 - ke_history.len() as f64) + i as f64;
                            [x, ke]
                        })
                        .collect();

                    let line = egui_plot::Line::new("KE", points)
                        .color(egui::Color32::from_rgb(100, 200, 255));

                    egui_plot::Plot::new("ke_plot")
                        .height(150.0)
                        .show_axes(true)
                        .allow_drag(false)
                        .allow_zoom(false)
                        .allow_scroll(false)
                        .show(ui, |plot_ui| {
                            plot_ui.line(line);
                        });
                });
        });

        egui_macroquad::draw();

        next_frame().await;
    }
}
