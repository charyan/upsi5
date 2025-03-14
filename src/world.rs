
use std::cell::RefCell;

use glam::Vec2;

use crate::entity;

const COLLISION_SMOOTHNESS: f32 = 0.03;

pub struct World {
    pub balls: Vec<RefCell<entity::Ball>>,
    width: f32,
    height: f32,
}

impl World {
    pub fn new(width: f32, height: f32) -> Self{
        Self {
            width,
            height,
            balls: vec![]
        }
    }

    pub fn add_ball(&mut self, position: Vec2, radius:f32, mass:f32, friction_coeff: f32) {
        let new_ball = entity::Ball{
            position,
            radius,
            mass,
            speed: Vec2::ZERO,
            friction_coeff
        };
        self.balls.push(RefCell::new(new_ball));
    }

    pub fn tick(&mut self) {
        for (index, ball_cell) in self.balls.iter().enumerate() {
            let mut ball = ball_cell.borrow_mut();
            ball.position =ball.position + ball.speed;
            ball.speed = ball.speed * ball.friction_coeff;

            for other_ball_index in index+1..self.balls.len() {
                let mut other_ball = self.balls[other_ball_index].borrow_mut();
                Self::collide(&mut ball, &mut other_ball);
                self.check_border(&mut ball);
            }
        }
    }

    fn check_border(&self, ball: &mut entity::Ball) {
        if ball.position.x - ball.radius < 0. {
            ball.position.x = ball.radius;
            ball.speed.x = -ball.speed.x;
        } else if ball.position.x + ball.radius > self.width {
            ball.position.x = self.width - ball.radius;
            ball.speed.x = -ball.speed.x;
        }
    
        if ball.position.y - ball.radius < 0. {
            ball.position.y = ball.radius;
            ball.speed.y = -ball.speed.y;
        } else if ball.position.y + ball.radius > self.height {
            ball.position.y = self.height - ball.radius;
            ball.speed.y = -ball.speed.y;
        }
    }


    pub fn collide(a: &mut entity::Ball, b: &mut entity::Ball) {
        let dist = a.position - b.position;

        let overlap = a.radius + b.radius - dist.length();

        if overlap > 0. {
            let push = dist.normalize_or_zero() * overlap * COLLISION_SMOOTHNESS;
            a.speed += push;
            b.speed -= push;
        }
    }

}