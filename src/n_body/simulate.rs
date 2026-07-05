use crate::n_body::octree::{Node, Octree};
use crate::n_body::vec3::Vec3;

pub struct Simulation {
    pub bodies: Vec<Vec3>,
    pub paths: Vec<Vec<[f32; 3]>>,
}

const G: f64 = 5.0; // Stronger attraction

impl Simulation {
    pub fn new() -> Self {
        Self {
            bodies: Vec::new(),
            paths: Vec::new(),
        }
    }

    pub fn add_body(&mut self, body: Vec3) {
        self.bodies.push(body);
        self.paths.push(Vec::new());
    }

    pub fn remove_body(&mut self, index: usize) {
        if index < self.bodies.len() {
            self.bodies.remove(index);
            self.paths.remove(index);
        }
    }

    pub fn clear(&mut self) {
        self.bodies.clear();
        self.paths.clear();
    }

    pub fn update_physics(&mut self, bounded: bool, half_box: f64) {
        self.gravitational_attration();
        self.octree_collision(half_box * 2.0);

        for p in &mut self.bodies {
            p.update();
            if bounded {
                p.keep_in_bounds(half_box);
            }
        }

        // Record trails
        if self.paths.len() != self.bodies.len() {
            self.paths.resize(self.bodies.len(), Vec::new());
        }
        for (i, p) in self.bodies.iter().enumerate() {
            self.paths[i].push([p.x() as f32, p.y() as f32, p.z() as f32]);
            if self.paths[i].len() > 150 {
                self.paths[i].remove(0);
            }
        }
    }

    pub fn gravitational_attration(&mut self) {
        for i in 0..self.bodies.len() {
            self.bodies[i].acc = [0.0, 0.0, 0.0];

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
                let dz = self.bodies[j].particle[2] - self.bodies[i].particle[2];

                self.bodies[i].acc[0] += (force_mag * dx / r) / self.bodies[i].mass;
                self.bodies[i].acc[1] += (force_mag * dy / r) / self.bodies[i].mass;
                self.bodies[i].acc[2] += (force_mag * dz / r) / self.bodies[i].mass;
            }
        }
    }

    pub fn octree_collision(&mut self, size: f64) {
        let boundary = Node::new(
            Vec3::new(0.0, 0.0, 0.0),
            size,
        );
        let mut ot = Octree::new(boundary, 4);

        // Build the tree with indices
        for i in 0..self.bodies.len() {
            ot.insert(i, &self.bodies);
        }

        // Query and resolve for each particle
        for i in 0..self.bodies.len() {
            let range = Node::new(self.bodies[i], 80.0);
            let mut neighbors = Vec::new();
            ot.query(&range, &self.bodies, &mut neighbors);

            for &j in &neighbors {
                if i < j {
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
                let (left, right) = self.bodies.split_at_mut(j);
                left[i].resolve_collision(&mut right[0]);
            }
        }
    }
}
