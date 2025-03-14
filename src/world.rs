use std::{cell::RefCell, collections::BTreeMap};

use glam::Vec2;

use crate::entity::{self, Ball, BallType};

const COLLISION_SMOOTHNESS: f32 = 0.03;

pub const WORLD_DIM: Vec2 = Vec2::new(1.850, 0.925);
const HOLE_RADIUS: f32 = 0.038;
const HOLES: [Vec2; 6] = [
    Vec2::ZERO,
    Vec2::new(0., WORLD_DIM.y),
    Vec2::new(WORLD_DIM.x / 2., 0.),
    Vec2::new(WORLD_DIM.x / 2., WORLD_DIM.y),
    Vec2::new(WORLD_DIM.x, 0.),
    Vec2::new(WORLD_DIM.x, WORLD_DIM.y),
];

pub struct World {
    pub balls: Vec<RefCell<entity::Ball>>,
    money: u32,
}

impl World {
    pub fn new() -> Self {
        Self {
            balls: vec![],
            money: 0,
        }
    }

    pub fn add_ball(&mut self, position: Vec2, radius: f32, mass: f32, friction_coeff: f32) {
        let new_ball = entity::Ball {
            position,
            radius,
            mass,
            speed: Vec2::ZERO,
            friction_coeff,
            letypedelaboule: entity::BallType::Player,
        };
        self.balls.push(RefCell::new(new_ball));
    }

    pub fn tick(&mut self) -> bool {
        let mut trash = vec![];
        for (index, ball_cell) in self.balls.iter().enumerate() {
            let mut ball = ball_cell.borrow_mut();
            ball.position = ball.position + ball.speed;
            ball.speed = ball.speed * ball.friction_coeff;
            Self::check_border(&mut ball);

            for other_ball_index in index + 1..self.balls.len() {
                let mut other_ball = self.balls[other_ball_index].borrow_mut();
                Self::collide(&mut ball, &mut other_ball);
            }
            if self.in_hole(&ball) {
                trash.push(index);
                if let BallType::Enemy(enemy) = ball.letypedelaboule {
                    self.money += enemy.price;
                }
            }
        }

        for bye_bye in trash.into_iter().rev() {
            self.balls.remove(bye_bye);
        }

        for ball in &self.balls {
            let ball = ball.borrow();
            if ball.speed.length() > 0.00001 {
                return true;
            }
        }

        false
    }

    fn in_hole(&self, ball: &entity::Ball) -> bool {
        for hole in &HOLES {
            if ball.radius + HOLE_RADIUS > ball.position.distance(*hole) {
                return true;
            }
        }
        false
    }

    pub fn launch_round(&mut self, velocities: BTreeMap<usize, Vec2>) {
        let mut new_balls = vec![];
        for (index, velocity) in &velocities {
            let ball = self.balls[*index].borrow();
            let ball1 = Ball::new(
                ball.mass,
                ball.position + velocity.normalize_or_zero() * ball.radius,
                *velocity,
                ball.friction_coeff,
                ball.radius,
                ball.letypedelaboule,
            );
            let ball2 = Ball::new(
                ball.mass,
                ball.position - velocity.normalize_or_zero() * ball.radius,
                *velocity * -1.,
                ball.friction_coeff,
                ball.radius,
                ball.letypedelaboule,
            );
            new_balls.push(RefCell::new(ball1));
            new_balls.push(RefCell::new(ball2));
        }

        for (index, _) in velocities.iter().rev() {
            self.balls.remove(*index);
        }

        self.balls.extend(new_balls);
    }

    fn check_border(ball: &mut entity::Ball) {
        if ball.position.x - ball.radius < 0. {
            ball.position.x = ball.radius;
            ball.speed.x = -ball.speed.x;
        } else if ball.position.x + ball.radius > WORLD_DIM.x {
            ball.position.x = WORLD_DIM.x - ball.radius;
            ball.speed.x = -ball.speed.x;
        }

        if ball.position.y - ball.radius < 0. {
            ball.position.y = ball.radius;
            ball.speed.y = -ball.speed.y;
        } else if ball.position.y + ball.radius > WORLD_DIM.y {
            ball.position.y = WORLD_DIM.y - ball.radius;
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
