use wasm_bindgen::prelude::*;
use js_sys::Float64Array;

const GRAVITY: f64 = 0.5;

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

                if index == other_index {
                    continue;
                }

                let distance = ((ball.x - other_ball.x).powf(2.) + (ball.y - other_ball.y).powf(2.)).sqrt();
                let radii = ball.radius + other_ball.radius;

                if distance < radii {

                    let diff = (radii - distance) / 2.;

                    let ball_x = (ball.x - other_ball.x).clamp(-diff, diff);
                    let ball_y = (ball.y - other_ball.y).clamp(-diff, diff);
                    let other_x = (other_ball.x - ball.x).clamp(-diff, diff);
                    let other_y = (other_ball.y - ball.y).clamp(-diff, diff);

                    // let ball_x = (ball.x - other_ball.x);
                    // let ball_y = (ball.y - other_ball.y);
                    // let other_x = (other_ball.x - ball.x);
                    // let other_y = (other_ball.y - ball.y);

                    corrections.push(Correction {ball_index: index, x: ball_x, y: ball_y});
                    corrections.push(Correction {ball_index: other_index, x: other_x, y: other_y});
                }
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
        self.solve();
        self.gravity();
        self.apply();
    }

    pub fn step(&mut self, miliseconds: u32) {

    }

    pub fn new() -> Universe {
        let width = 64.;
        let height = 64.;

        Universe {
            width,
            height,
            balls: vec![
                Ball {radius: 50., x: 100., y: 100., vel_x: 0., vel_y: 0.},
                Ball {radius: 50., x: 150., y: -100., vel_x: -20., vel_y: 0.},
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