use wasm_bindgen::prelude::*;
use js_sys::Float64Array;

const GRAVITY: f64 = 0.1;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ball {
    radius: f64,
    x: f64,
    y: f64,
    vel_x: f64,
    vel_y: f64,
}

#[wasm_bindgen]
pub struct Universe {
    width: f64,
    height: f64,
    balls: Vec<Ball>,
}

struct Correction {
    ball_index: usize,
    x: f64,
    y: f64,
}

impl Universe {
    fn solve(&mut self) {

        let mut corrections: Vec<Correction> = Vec::new();

        for (index, ball) in self.balls.iter().enumerate() {
            for (other_index, other_ball) in self.balls.iter().enumerate() {
                let x = ball.x - other_ball.x;
                let y = ball.y - other_ball.y;

                let mut diff_x = 0.;
                let mut diff_y = 0.;

                if (x.abs() < ball.radius && y.abs() < ball.radius) {
                    diff_x = x / 2.;
                    diff_y = y / 2.;
                }

                corrections.push(Correction {ball_index: index, x: diff_x, y: diff_y});
                }
        }
        
        for (index, ball) in self.balls.iter_mut().enumerate() {
            for correction in corrections.iter() {
                if correction.ball_index == index {
                    ball.x += correction.x;
                    ball.y += correction.y;
                }
            }
        }
    }

    fn gravity(&mut self) {
        for ball in self.balls.iter_mut() {
            let grav_vector = (ball.x.powf(2.) + ball.y.powf(2.)).sqrt();
            let ratio_x = ball.x / grav_vector;
            let ratio_y = ball.y / grav_vector;

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

    pub fn step(&mut self, miliseconds: u32) {
        for i in 0..=miliseconds {
            self.tick();
        }
    }

    pub fn new() -> Universe {
        let width = 64.;
        let height = 64.;

        Universe {
            width,
            height,
            balls: vec![
                Ball {radius: 50., x: 30., y: 30., vel_x: 0., vel_y: 0.},
                Ball {radius: 40., x: -40., y: -20., vel_x: -0., vel_y: 0.},
                Ball {radius: 30., x: 40., y: 0., vel_x: -0., vel_y: 0.},
                Ball {radius: 20., x: 30., y: -30., vel_x: -0., vel_y: 0.},
                Ball {radius: 10., x: 20., y: 30., vel_x: -0., vel_y: 0.},
            ],
        }
    }

    //TODO this is kind of ugly, but were fighting with types between js and wasm here
    pub fn get_x(&self) -> Float64Array {
        Float64Array::from(&self.balls.iter().map(|ball| ball.x).collect::<Vec<f64>>()[..])
    }

    pub fn get_y(&self) -> Float64Array {
        Float64Array::from(&self.balls.iter().map(|ball| ball.y).collect::<Vec<f64>>()[..])
    }

    pub fn get_radius(&self) -> Float64Array {
        Float64Array::from(&self.balls.iter().map(|ball| ball.radius).collect::<Vec<f64>>()[..])
    }
}
