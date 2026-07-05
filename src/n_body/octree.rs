use crate::n_body::vec3::Vec3;

#[derive(Copy, Clone, Debug)]
pub struct Node {
    pub pos: Vec3,
    pub size: f64,
}

impl Node {
    pub fn new(pos: Vec3, size: f64) -> Self {
        Node { pos, size }
    }

    pub fn contains(&self, p_pos: &Vec3) -> bool {
        let half_size = self.size / 2.0;
        p_pos.x() >= self.pos.x() - half_size
            && p_pos.x() < self.pos.x() + half_size
            && p_pos.y() >= self.pos.y() - half_size
            && p_pos.y() < self.pos.y() + half_size
            && p_pos.z() >= self.pos.z() - half_size
            && p_pos.z() < self.pos.z() + half_size
    }

    pub fn intersects(&self, range: &Node) -> bool {
        let half_size = self.size / 2.0;
        let range_half = range.size / 2.0;

        !(range.pos.x() - range_half > self.pos.x() + half_size
            || range.pos.x() + range_half < self.pos.x() - half_size
            || range.pos.y() - range_half > self.pos.y() + half_size
            || range.pos.y() + range_half < self.pos.y() - half_size
            || range.pos.z() - range_half > self.pos.z() + half_size
            || range.pos.z() + range_half < self.pos.z() - half_size)
    }
}

pub struct Octree {
    pub boundary: Node,
    pub capacity: usize,
    pub bodies: Vec<usize>, // Store indices
    pub divided: bool,
    pub children: Option<Box<[Octree; 8]>>,
}

impl Octree {
    pub fn new(boundary: Node, capacity: usize) -> Self {
        Octree {
            boundary,
            capacity,
            bodies: Vec::new(),
            divided: false,
            children: None,
        }
    }

    pub fn subdivide(&mut self) {
        let half_size = self.boundary.size / 2.0;
        let quarter_size = self.boundary.size / 4.0;
        let x = self.boundary.pos.x();
        let y = self.boundary.pos.y();
        let z = self.boundary.pos.z();

        let offsets = [
            (-quarter_size, -quarter_size, -quarter_size),
            ( quarter_size, -quarter_size, -quarter_size),
            (-quarter_size,  quarter_size, -quarter_size),
            ( quarter_size,  quarter_size, -quarter_size),
            (-quarter_size, -quarter_size,  quarter_size),
            ( quarter_size, -quarter_size,  quarter_size),
            (-quarter_size,  quarter_size,  quarter_size),
            ( quarter_size,  quarter_size,  quarter_size),
        ];

        let mut children_vec = Vec::with_capacity(8);
        for &(dx, dy, dz) in &offsets {
            let child_pos = Vec3::new(x + dx, y + dy, z + dz);
            children_vec.push(Octree::new(
                Node::new(child_pos, half_size),
                self.capacity,
            ));
        }

        let boxed_array: Box<[Octree; 8]> = match children_vec.try_into() {
            Ok(arr) => Box::new(arr),
            Err(_) => panic!("Failed to convert children vector to array"),
        };

        self.children = Some(boxed_array);
        self.divided = true;
    }

    pub fn insert(&mut self, index: usize, all_bodies: &[Vec3]) -> bool {
        if !self.boundary.contains(&all_bodies[index]) {
            return false;
        }

        if self.bodies.len() < self.capacity {
            self.bodies.push(index);
            return true;
        }

        if !self.divided {
            self.subdivide();
        }

        let children = self.children.as_mut().unwrap();
        for child in children.iter_mut() {
            if child.insert(index, all_bodies) {
                return true;
            }
        }

        false
    }

    pub fn query(&self, range: &Node, all_bodies: &[Vec3], found: &mut Vec<usize>) {
        if !self.boundary.intersects(range) {
            return;
        }

        for &index in &self.bodies {
            if range.contains(&all_bodies[index]) {
                found.push(index);
            }
        }

        if self.divided {
            let children = self.children.as_ref().unwrap();
            for child in children.iter() {
                child.query(range, all_bodies, found);
            }
        }
    }

    /// Draw every cell boundary as a cube wireframe (for debug visualisation).
    pub fn draw(&self) {
        use macroquad::prelude::*;
        let s = self.boundary.size as f32;
        let pos = vec3(
            self.boundary.pos.x() as f32,
            self.boundary.pos.y() as f32,
            self.boundary.pos.z() as f32,
        );

        let half = s / 2.0;
        let c = Color::from_rgba(0, 200, 100, 100);

        // 8 vertices
        let v = [
            pos + vec3(-half, -half, -half),
            pos + vec3( half, -half, -half),
            pos + vec3( half,  half, -half),
            pos + vec3(-half,  half, -half),
            pos + vec3(-half, -half,  half),
            pos + vec3( half, -half,  half),
            pos + vec3( half,  half,  half),
            pos + vec3(-half,  half,  half),
        ];

        // 12 edges
        let edges = [
            (0, 1), (1, 2), (2, 3), (3, 0), // back face
            (4, 5), (5, 6), (6, 7), (7, 4), // front face
            (0, 4), (1, 5), (2, 6), (3, 7), // connections
        ];

        for &(i, j) in &edges {
            draw_line_3d(v[i], v[j], c);
        }

        if self.divided {
            if let Some(ref children) = self.children {
                for child in children.iter() {
                    child.draw();
                }
            }
        }
    }
}
