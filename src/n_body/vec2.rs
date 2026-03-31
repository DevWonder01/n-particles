#[derive(Copy, Clone, Default)]
pub struct Vec2 {
    pub e: [f64; 2],
    pub vel: [f64; 2],
    pub acc: [f64; 2],
    pub mass: f64,
}

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Vec2 {
        Vec2 {
            e: [x, y],
            vel: [0.0, 0.0],
            acc: [0.0, 0.0],
            mass: 1.0,
        }
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    


    pub fn dist_sq(&self, other: &Vec2) -> f64 {
        (self.e[0] - other.e[0]).powi(2) + (self.e[1] - other.e[1]).powi(2)
    }

    pub fn update(&mut self) {
        self.vel[0] += self.acc[0];
        self.vel[1] += self.acc[1];
        self.e[0] += self.vel[0];
        self.e[1] += self.vel[1];
    }

    pub fn draw(&self) {
        use macroquad::prelude::*;
        // Calculate radius based on mass (area = PI * r^2)
        let radius = (self.mass / std::f64::consts::PI).sqrt() * 10.0;

        draw_circle(
            self.x() as f32,
            self.y() as f32,
            radius as f32,
            Color::from_rgba(100, 150, 255, 255), // A nice soft blue
        );
    }
}
