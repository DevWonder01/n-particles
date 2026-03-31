use crate::n_body::vec2::Vec2;

pub struct Simulation {
    pub bodies: Vec<Vec2>,
}

const G: f64 = 5.0; // Increased to 5.0 for much stronger attraction and faster movement

impl Simulation {
    pub fn new() -> Self {
        Self { bodies: Vec::new() }
    }

    // f = g * m1 * m2 / r^2

    pub fn gravitation_attration(&mut self) {
        for i in 0..self.bodies.len() {
            self.bodies[i].acc = [0.0, 0.0];

            for j in 0..self.bodies.len() {
                if i == j {
                    continue;
                }

                let dist_sq = self.bodies[i].dist_sq(&self.bodies[j]);
                if dist_sq < 1.0 {
                    continue;
                }

                let r = dist_sq.sqrt();
                let force_mag = G * self.bodies[i].mass * self.bodies[j].mass / dist_sq;

                let dx = self.bodies[j].e[0] - self.bodies[i].e[0];
                let dy = self.bodies[j].e[1] - self.bodies[i].e[1];

                self.bodies[i].acc[0] += (force_mag * dx / r) / self.bodies[i].mass;
                self.bodies[i].acc[1] += (force_mag * dy / r) / self.bodies[i].mass;
            }
        }
    }
}
