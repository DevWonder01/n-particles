#[derive(Copy, Clone, Default)]
pub struct Vec2 {
    pub particle: [f64; 2],
    pub vel: [f64; 2],
    pub acc: [f64; 2],
    pub mass: f64,
}

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Vec2 {
        Vec2 {
            particle: [x, y],
            vel: [0.0, 0.0],
            acc: [0.0, 0.0],
            mass: 1.0,
        }
    }

    pub fn x(&self) -> f64 {
        self.particle[0]
    }

    pub fn y(&self) -> f64 {
        self.particle[1]
    }

    pub fn radius(&self) -> f64 {
        // Unified radius calculation: (Area = PI * r^2) * scale
        (self.mass / std::f64::consts::PI).sqrt() * 10.0
    }

    pub fn collision(&self, other: &Vec2) -> bool {
        let dist = self.dist_sq(other).sqrt();

        let r1 = self.radius();
        let r2 = other.radius();

        if dist < r1 + r2 {
            return true;
        }
        return false;
    }

    pub fn resolve_collision(&mut self, other: &mut Vec2) {
        let dx = other.particle[0] - self.particle[0];
        let dy = other.particle[1] - self.particle[1];
        let dist_sq = self.dist_sq(other);
        let dist = dist_sq.sqrt();

        let r1 = self.radius();
        let r2 = other.radius();

        if dist < r1 + r2 && dist > 0.0 {
            // --- 1. Static resolution (push apart) ---
            let overlap = (r1 + r2) - dist;
            let nx = dx / dist; // Collision Normal X
            let ny = dy / dist; // Collision Normal Y

            let total_mass = self.mass + other.mass;
            let m1_ratio = other.mass / total_mass;
            let m2_ratio = self.mass / total_mass;

            self.particle[0] -= nx * overlap * m1_ratio;
            self.particle[1] -= ny * overlap * m1_ratio;
            other.particle[0] += nx * overlap * m2_ratio;
            other.particle[1] += ny * overlap * m2_ratio;

            // --- 2. Dynamic resolution (Elastic Collision) ---
            // Relative velocity
            let r_vel_x = self.vel[0] - other.vel[0];
            let r_vel_y = self.vel[1] - other.vel[1];

            // Velocity along the normal
            let vel_along_normal = r_vel_x * nx + r_vel_y * ny;

            // Only resolve if they are moving towards each other
            if vel_along_normal > 0.0 {
                let j = 2.0 * vel_along_normal / total_mass;

                self.vel[0] -= j * other.mass * nx;
                self.vel[1] -= j * other.mass * ny;
                other.vel[0] += j * self.mass * nx;
                other.vel[1] += j * self.mass * ny;
            }
        }
    }

    pub fn dist_sq(&self, other: &Vec2) -> f64 {
        (self.particle[0] - other.particle[0]).powi(2)
            + (self.particle[1] - other.particle[1]).powi(2)
    }

    pub fn update(&mut self) {
        self.vel[0] += self.acc[0];
        self.vel[1] += self.acc[1];
        self.particle[0] += self.vel[0];
        self.particle[1] += self.vel[1];
    }

    pub fn keep_in_bounds(&mut self, width: f64, height: f64) {
        let radius = (self.mass / std::f64::consts::PI).sqrt();

        // Horizontal bounds
        if self.particle[0] - radius < 0.0 {
            self.particle[0] = radius;
            self.vel[0] *= -0.8; // Lose 20% energy on bounce
        } else if self.particle[0] + radius > width {
            self.particle[0] = width - radius;
            self.vel[0] *= -0.8;
        }

        // Vertical bounds
        if self.particle[1] - radius < 0.0 {
            self.particle[1] = radius;
            self.vel[1] *= -0.8;
        } else if self.particle[1] + radius > height {
            self.particle[1] = height - radius;
            self.vel[1] *= -0.8;
        }
    }

    pub fn draw(&self) {
        use macroquad::prelude::*;
        let r = self.radius();

        draw_circle(
            self.x() as f32,
            self.y() as f32,
            r as f32,
            Color::from_rgba(255, 255, 255, 255),
        );
    }
}
