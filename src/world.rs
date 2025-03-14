
use std::vec;

use crate::entity;


pub struct World {
    pub balls: Vec<entity::Ball>,
}

impl World {
    pub fn tick(&mut self) -> Self {
        let mut new_balls = vec![];

        for mut ball in self.balls.iter().copied() {
            ball.position += ball.speed;
            ball.speed *= ball.friction_coeff;
            new_balls.push(ball);
        }
        World{balls: vec![]}
    }




}