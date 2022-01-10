use crate::draw::draw_block;
use piston_window::{Context, G2d, types::Color};

const BALL_COLOR: Color = [1.0, 1.0, 1.0, 1.0];

#[derive(Copy, Clone, Debug)]
pub struct Ball {
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
    direction: f64,
}

impl Ball {
    pub fn new(x: f64, y: f64, vx: f64, vy: f64, direction: f64) -> Self {
        Self { x, y, vx, vy, direction }
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        draw_block(BALL_COLOR, self.x, self.y, con, g);
    }

    pub fn set_position(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
    }

    pub fn get_next_location(&self, delta_time: f64) -> (f64, f64) {
        let distance_x = (self.vx * delta_time) * self.direction;
        let distance_y = self.vy * delta_time;
        let new_x = self.x + distance_x;
        let new_y = self.y + distance_y;
        (new_x, new_y)
    }

    pub fn flip_velocity_y(&mut self) {
        self.vy *= -1.0;
    }

    pub fn flip_velocity_x(&mut self, direction: f64) {
        self.direction = direction;
    }

    pub fn increase_y(&mut self, factor: f64) {
        self.vy += factor;
    }

    pub fn set_velocity(&mut self, vx: f64, vy: f64) {
        self.vx = vx;
        self.vy = vy;
    }

    pub fn set_direction(&mut self, direction: f64) {
        self.direction = direction;
    }

    pub fn get_direction(&self) -> f64 {
        self.direction
    }
}