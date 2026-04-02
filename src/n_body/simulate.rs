use crate::n_body::quadtree::{Node, QuadTree};
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

    pub fn gravitational_attration(&mut self) {
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

                let dx = self.bodies[j].particle[0] - self.bodies[i].particle[0];
                let dy = self.bodies[j].particle[1] - self.bodies[i].particle[1];

                self.bodies[i].acc[0] += (force_mag * dx / r) / self.bodies[i].mass;
                self.bodies[i].acc[1] += (force_mag * dy / r) / self.bodies[i].mass;
            }
        }
    }

    pub fn quad_tree_collision(&mut self, width: f32, height: f32) {
        let boundary = Node::new(
            Vec2::new(width as f64 / 2.0, height as f64 / 2.0),
            width.max(height) as f64,
        );
        let mut qt = QuadTree::new(boundary, 4);

        // Build the tree with indices
        for i in 0..self.bodies.len() {
            qt.insert(i, &self.bodies);
        }

        // Query and resolve for each particle
        for i in 0..self.bodies.len() {
            // Define search area (roughly 4x max particle radius to be safe)
            let range = Node::new(self.bodies[i], 80.0);
            let mut neighbors = Vec::new();
            qt.query(&range, &self.bodies, &mut neighbors);

            for &j in &neighbors {
                if i < j {
                    // Safe split_at_mut to get two different indices
                    let (left, right) = self.bodies.split_at_mut(j);
                    left[i].resolve_collision(&mut right[0]);
                }
            }
        }
    }

    pub fn handle_collisions(&mut self) {
        let len = self.bodies.len();
        for i in 0..len {
            for j in i + 1..len {
                // Safely get two mutable references to different elements
                let (left, right) = self.bodies.split_at_mut(j);
                left[i].resolve_collision(&mut right[0]);
            }
        }
    }
}
