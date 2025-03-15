use std::{cell::RefCell, collections::BTreeMap};

use glam::Vec2;
use marmalade::console;

use crate::entity::{self, Ball, BallType};

const COLLISION_SMOOTHNESS: f32 = 0.03;

pub const WORLD_DIM: Vec2 = Vec2::new(1.926, 1.01);
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
    round: u32,
}

impl World {
    pub fn new() -> Self {
        Self {
            balls: vec![],
            money: 0,
            round: 1,
        }
    }

    pub fn spawn_round(&mut self) {}

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
        let mut trash = Vec::new();
        let mut new_balls = Vec::new();
        for (index, ball_cell) in self.balls.iter().enumerate() {
            {
                let mut ball = ball_cell.borrow_mut();
                ball.position = ball.position + ball.speed;
                ball.speed = ball.speed * ball.friction_coeff;
                Self::check_border(&mut ball);
                if self.in_hole(&ball) {
                    trash.push(index);
                    if let BallType::Enemy(enemy) = ball.letypedelaboule {
                        self.money += enemy.price;
                    }
                }
            }
            for other_ball_index in index + 1..self.balls.len() {
                if let Some((a, b, new_ball)) = self.collide(index, other_ball_index) {
                    trash.push(a);
                    trash.push(b);
                    new_balls.push(RefCell::new(new_ball));
                }
            }
        }

        trash.sort();

        for bye_bye in trash.into_iter().rev() {
            self.balls.remove(bye_bye);
        }

        self.balls.extend(new_balls);

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
                ball.mass / 2.,
                ball.position + velocity.normalize_or_zero() * ball.radius,
                *velocity,
                ball.friction_coeff,
                ball.radius / 2f32.sqrt(),
                ball.letypedelaboule,
            );
            let ball2 = Ball::new(
                ball.mass / 2.,
                ball.position - velocity.normalize_or_zero() * ball.radius,
                *velocity * -1.,
                ball.friction_coeff,
                ball.radius / 2f32.sqrt(),
                ball.letypedelaboule,
            );
            new_balls.push(RefCell::new(ball1));
            new_balls.push(RefCell::new(ball2));
        }

        for (index, _) in velocities.iter().rev() {
            self.balls.remove(*index);
        }

        self.balls.extend(new_balls);
        self.round += 1
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

    pub fn collide(&self, a: usize, b: usize) -> Option<(usize, usize, Ball)> {
        let mut ball_a = self.balls[a].borrow_mut();
        let mut ball_b = self.balls[b].borrow_mut();

        let dist = ball_a.position - ball_b.position;

        let overlap = ball_a.radius + ball_b.radius - dist.length();

        if overlap > 0. {
            if ball_a.letypedelaboule == BallType::Player
                && ball_b.letypedelaboule == BallType::Player
            {
                console::log("oui");
                let tot_mass = ball_a.mass + ball_b.mass;
                let new_ball = entity::Ball {
                    mass: tot_mass,
                    position: (ball_a.position + ball_b.position) / (2.),
                    speed: Vec2::new(
                        (ball_a.speed.x * ball_a.mass + ball_b.speed.x * ball_b.mass) / (tot_mass),
                        (ball_a.speed.y * ball_a.mass + ball_b.speed.y * ball_b.mass)
                            / (2. * tot_mass),
                    ),
                    friction_coeff: ball_a.friction_coeff,
                    radius: (ball_a.radius.powf(2.) + ball_b.radius.powf(2.)).sqrt(),
                    letypedelaboule: BallType::Player,
                };

                let smaller = a.min(b);
                let bigger = a.max(b);
                return Some((smaller, bigger, new_ball));
            } else {
                let push = dist.normalize_or_zero() * overlap * COLLISION_SMOOTHNESS;
                ball_a.speed = ball_a.speed + push * ball_a.mass / ball_b.mass;
                ball_b.speed = ball_b.speed - push * ball_b.mass / ball_a.mass;
            }
        }
        None
    }
}
