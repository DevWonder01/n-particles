use crate::n_body::vec2::Vec2;

#[derive(Copy, Clone)]
pub struct Node {
    pub pos: Vec2,
    pub size: f64,
}

impl Node {
    pub fn new(pos: Vec2, size: f64) -> Self {
        Node { pos, size }
    }

    pub fn contains(&self, body: &Vec2) -> bool {
        let half_size = self.size / 2.0;
        body.x() >= self.pos.x() - half_size
            && body.x() < self.pos.x() + half_size
            && body.y() >= self.pos.y() - half_size
            && body.y() < self.pos.y() + half_size
    }
}

pub struct QuadTree {
    pub boundary: Node,
    pub capacity: usize,
    pub bodies: Vec<Vec2>,
    pub divided: bool,
    pub nw: Option<Box<QuadTree>>,
    pub ne: Option<Box<QuadTree>>,
    pub sw: Option<Box<QuadTree>>,
    pub se: Option<Box<QuadTree>>,
}

impl QuadTree {
    pub fn new(boundary: Node, capacity: usize) -> Self {
        QuadTree {
            boundary,
            capacity,
            bodies: Vec::new(),
            divided: false,
            nw: None,
            ne: None,
            sw: None,
            se: None,
        }
    }

    pub fn subdivide(&mut self) {
        let half_size = self.boundary.size / 2.0;
        let quarter_size = self.boundary.size / 4.0;
        let x = self.boundary.pos.x();
        let y = self.boundary.pos.y();

        // Northwest
        let nw_pos = Vec2::new(x - quarter_size, y - quarter_size);
        self.nw = Some(Box::new(QuadTree::new(
            Node::new(nw_pos, half_size),
            self.capacity,
        )));

        // Northeast
        let ne_pos = Vec2::new(x + quarter_size, y - quarter_size);
        self.ne = Some(Box::new(QuadTree::new(
            Node::new(ne_pos, half_size),
            self.capacity,
        )));

        // Southwest
        let sw_pos = Vec2::new(x - quarter_size, y + quarter_size);
        self.sw = Some(Box::new(QuadTree::new(
            Node::new(sw_pos, half_size),
            self.capacity,
        )));

        // Southeast
        let se_pos = Vec2::new(x + quarter_size, y + quarter_size);
        self.se = Some(Box::new(QuadTree::new(
            Node::new(se_pos, half_size),
            self.capacity,
        )));

        self.divided = true;
    }

    pub fn insert(&mut self, body: Vec2) -> bool {
        if !self.boundary.contains(&body) {
            return false;
        }

        if self.bodies.len() < self.capacity {
            self.bodies.push(body);
            return true;
        }

        if !self.divided {
            self.subdivide();
        }

        self.nw.as_mut().unwrap().insert(body)
            || self.ne.as_mut().unwrap().insert(body)
            || self.sw.as_mut().unwrap().insert(body)
            || self.se.as_mut().unwrap().insert(body)
    }

    pub fn query(&self, range: &Node, found: &mut Vec<Vec2>) {
        if !self.intersects(range) {
            return;
        }

        for body in &self.bodies {
            if range.contains(body) {
                found.push(*body);
            }
        }

        if self.divided {
            self.nw.as_ref().unwrap().query(range, found);
            self.ne.as_ref().unwrap().query(range, found);
            self.sw.as_ref().unwrap().query(range, found);
            self.se.as_ref().unwrap().query(range, found);
        }
    }

    pub fn intersects(&self, range: &Node) -> bool {
        let half_size = self.boundary.size / 2.0;
        let range_half = range.size / 2.0;

        !(range.pos.x() - range_half > self.boundary.pos.x() + half_size
            || range.pos.x() + range_half < self.boundary.pos.x() - half_size
            || range.pos.y() - range_half > self.boundary.pos.y() + half_size
            || range.pos.y() + range_half < self.boundary.pos.y() - half_size)
    }


    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_contains() {
        let node = Node::new(Vec2::new(0.0, 0.0), 100.0);
        let body_inside = Vec2::new(25.0, 25.0);
        let body_outside = Vec2::new(60.0, 60.0);

        assert!(node.contains(&body_inside));
        assert!(!node.contains(&body_outside));
    }

    #[test]
    fn test_quadtree_insert() {
        let boundary = Node::new(Vec2::new(0.0, 0.0), 100.0);
        let mut qt = QuadTree::new(boundary, 4);

        let body1 = Vec2::new(10.0, 10.0);
        let body2 = Vec2::new(20.0, 20.0);

        assert!(qt.insert(body1));
        assert!(qt.insert(body2));
        assert_eq!(qt.bodies.len(), 2);
    }

    #[test]
    fn test_quadtree_subdivide() {
        let boundary = Node::new(Vec2::new(0.0, 0.0), 100.0);
        let mut qt = QuadTree::new(boundary, 2);

        // Insert 3 bodies to trigger subdivision
        qt.insert(Vec2::new(10.0, 10.0));
        qt.insert(Vec2::new(20.0, 20.0));
        qt.insert(Vec2::new(-10.0, -10.0));

        assert!(qt.divided);
        assert!(qt.nw.is_some());
        assert!(qt.ne.is_some());
        assert!(qt.sw.is_some());
        assert!(qt.se.is_some());
    }

    #[test]
    fn test_quadtree_query() {
        let boundary = Node::new(Vec2::new(0.0, 0.0), 100.0);
        let mut qt = QuadTree::new(boundary, 4);

        qt.insert(Vec2::new(10.0, 10.0));
        qt.insert(Vec2::new(20.0, 20.0));
        qt.insert(Vec2::new(-30.0, -30.0));

        let query_range = Node::new(Vec2::new(0.0, 0.0), 40.0);
        let mut found = Vec::new();
        qt.query(&query_range, &mut found);

        assert_eq!(found.len(), 2);
    }

    #[test]
    fn test_quadtree_intersects() {
        let boundary = Node::new(Vec2::new(0.0, 0.0), 100.0);
        let qt = QuadTree::new(boundary, 4);

        let overlapping_range = Node::new(Vec2::new(30.0, 30.0), 40.0);
        let non_overlapping_range = Node::new(Vec2::new(100.0, 100.0), 20.0);

        assert!(qt.intersects(&overlapping_range));
        assert!(!qt.intersects(&non_overlapping_range));
    }

    #[test]
    fn test_insert_out_of_bounds() {
        let boundary = Node::new(Vec2::new(0.0, 0.0), 100.0);
        let mut qt = QuadTree::new(boundary, 4);

        let body_outside = Vec2::new(100.0, 100.0);
        assert!(!qt.insert(body_outside));
    }
}