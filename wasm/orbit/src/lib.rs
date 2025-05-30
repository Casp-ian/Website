use js_sys::Float64Array;
use wasm_bindgen::prelude::*;

const GRAVITY: f64 = 0.1;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ball {
    pub radius: f64,
    pub x: f64,
    pub y: f64,
    vel_x: f64,
    vel_y: f64,
}

impl Ball {
    fn new(radius: f64, x: f64, y: f64) -> Ball {
        return Ball {
            radius,
            x,
            y,
            vel_x: 0.,
            vel_y: 0.,
        };
    }
}

#[wasm_bindgen]
pub struct Universe {
    center_x: f64,
    center_y: f64,
    balls: Vec<Ball>,
}

impl Universe {
    fn solve(&mut self) {
        // Do nothing
    }

    fn gravity(&mut self) {
        for ball in self.balls.iter_mut() {
            let diff_x = ball.x - self.center_x;
            let diff_y = ball.y - self.center_y;

            let distance = (diff_x.powf(2.) + diff_y.powf(2.)).sqrt();

            let ratio_x = diff_x / distance;
            let ratio_y = diff_y / distance;

            ball.vel_x -= ratio_x * GRAVITY;
            ball.vel_y -= ratio_y * GRAVITY;
        }
    }

    fn apply(&mut self) {
        for ball in self.balls.iter_mut() {
            ball.x += ball.vel_x;
            ball.y += ball.vel_y;
        }
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn tick(&mut self) {
        self.gravity();
        self.apply();
        self.solve();
    }

    pub fn step(&mut self, milliseconds: u32) {
        for i in 0..=milliseconds {
            self.tick();
        }
    }

    pub fn new() -> Universe {
        let center_x = 100.;
        let center_y = 100.;

        Universe {
            center_x,
            center_y,
            balls: vec![
                Ball::new(50., 30., 30.),
                Ball::new(40., 400., 20.),
                Ball::new(30., 10., 300.),
                Ball::new(20., 320., 130.),
                Ball::new(10., 130., 320.),
            ],
        }
    }

    // TODO make js use this
    pub fn get_ball(&self, i: usize) -> Ball {
        return *self.balls.get(i).unwrap();
    }

    pub fn get_ball_count(&self) -> usize {
        return self.balls.len();
    }

    pub fn set_center(&mut self, x: f64, y: f64) {
        self.center_x = x;
        self.center_y = y;
    }
}
