#[derive(Copy, Clone, Default, Debug)]
pub struct Vec3 {
    pub particle: [f64; 3],
    pub vel: [f64; 3],
    pub acc: [f64; 3],
    pub mass: f64,
    pub fixed: bool,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 {
            particle: [x, y, z],
            vel: [0.0, 0.0, 0.0],
            acc: [0.0, 0.0, 0.0],
            mass: 1.0,
            fixed: false,
        }
    }

    pub fn x(&self) -> f64 {
        self.particle[0]
    }

    pub fn y(&self) -> f64 {
        self.particle[1]
    }

    pub fn z(&self) -> f64 {
        self.particle[2]
    }

    pub fn radius(&self) -> f64 {
        // In 3D, Volume = 4/3 * PI * r^3.
        // Let's make the radius proportional to cube root of mass.
        // radius = (mass / (4/3 * PI))^(1/3) * scale
        let volume_factor = 3.0 / (4.0 * std::f64::consts::PI);
        (self.mass * volume_factor).cbrt() * 4.0
    }

    pub fn collision(&self, other: &Vec3) -> bool {
        let dist = self.dist_sq(other).sqrt();
        let r1 = self.radius();
        let r2 = other.radius();
        dist < r1 + r2
    }

    pub fn resolve_collision(&mut self, other: &mut Vec3) {
        let dx = other.particle[0] - self.particle[0];
        let dy = other.particle[1] - self.particle[1];
        let dz = other.particle[2] - self.particle[2];
        let dist_sq = self.dist_sq(other);
        let dist = dist_sq.sqrt();

        let r1 = self.radius();
        let r2 = other.radius();

        if dist < r1 + r2 && dist > 0.0 {
            // --- 1. Static resolution (push apart) ---
            let overlap = (r1 + r2) - dist;
            let nx = dx / dist; // Collision Normal X
            let ny = dy / dist; // Collision Normal Y
            let nz = dz / dist; // Collision Normal Z

            let total_mass = self.mass + other.mass;
            let m1_ratio = other.mass / total_mass;
            let m2_ratio = self.mass / total_mass;

            self.particle[0] -= nx * overlap * m1_ratio;
            self.particle[1] -= ny * overlap * m1_ratio;
            self.particle[2] -= nz * overlap * m1_ratio;
            other.particle[0] += nx * overlap * m2_ratio;
            other.particle[1] += ny * overlap * m2_ratio;
            other.particle[2] += nz * overlap * m2_ratio;

            // --- 2. Dynamic resolution (Elastic Collision) ---
            // Relative velocity
            let r_vel_x = self.vel[0] - other.vel[0];
            let r_vel_y = self.vel[1] - other.vel[1];
            let r_vel_z = self.vel[2] - other.vel[2];

            // Velocity along the normal
            let vel_along_normal = r_vel_x * nx + r_vel_y * ny + r_vel_z * nz;

            // Only resolve if they are moving towards each other
            if vel_along_normal > 0.0 {
                let j = 2.0 * vel_along_normal / total_mass;

                self.vel[0] -= j * other.mass * nx;
                self.vel[1] -= j * other.mass * ny;
                self.vel[2] -= j * other.mass * nz;
                other.vel[0] += j * self.mass * nx;
                other.vel[1] += j * self.mass * ny;
                other.vel[2] += j * self.mass * nz;
            }
        }
    }

    pub fn dist_sq(&self, other: &Vec3) -> f64 {
        (self.particle[0] - other.particle[0]).powi(2)
            + (self.particle[1] - other.particle[1]).powi(2)
            + (self.particle[2] - other.particle[2]).powi(2)
    }

    pub fn update(&mut self) {
        if self.fixed {
            return;
        }
        self.vel[0] += self.acc[0];
        self.vel[1] += self.acc[1];
        self.vel[2] += self.acc[2];
        self.particle[0] += self.vel[0];
        self.particle[1] += self.vel[1];
        self.particle[2] += self.vel[2];
    }

    pub fn keep_in_bounds(&mut self, half_size: f64) {
        if self.fixed {
            return;
        }
        let radius = self.radius();

        // X bounds: [-half_size, half_size]
        if self.particle[0] - radius < -half_size {
            self.particle[0] = -half_size + radius;
            self.vel[0] *= -0.8;
        } else if self.particle[0] + radius > half_size {
            self.particle[0] = half_size - radius;
            self.vel[0] *= -0.8;
        }

        // Y bounds: [-half_size, half_size]
        if self.particle[1] - radius < -half_size {
            self.particle[1] = -half_size + radius;
            self.vel[1] *= -0.8;
        } else if self.particle[1] + radius > half_size {
            self.particle[1] = half_size - radius;
            self.vel[1] *= -0.8;
        }

        // Z bounds: [-half_size, half_size]
        if self.particle[2] - radius < -half_size {
            self.particle[2] = -half_size + radius;
            self.vel[2] *= -0.8;
        } else if self.particle[2] + radius > half_size {
            self.particle[2] = half_size - radius;
            self.vel[2] *= -0.8;
        }
    }

    pub fn draw(&self) {
        use macroquad::prelude::*;
        let r = self.radius();

        draw_sphere(
            vec3(self.x() as f32, self.y() as f32, self.z() as f32),
            r as f32,
            None,
            Color::from_rgba(255, 255, 255, 255),
        );
    }
}
