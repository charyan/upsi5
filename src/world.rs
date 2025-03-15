use std::{cell::RefCell, collections::BTreeMap};

use crate::entity::{self, Ball, BallType, EnemyData};
use glam::Vec2;
use marmalade::{console, rand};

const COLLISION_SMOOTHNESS: f32 = 0.03;
const COIN_RADIUS: f32 = 0.01;
const COIN_PRICE: u32 = 100;
pub const WORLD_DIM: Vec2 = Vec2::new(1.926, 1.01);
const HOLE_RADIUS: f32 = 0.038;
const HOLES: [Vec2; 6] = [
    Vec2::new(HOLE_RADIUS, HOLE_RADIUS),
    Vec2::new(HOLE_RADIUS, WORLD_DIM.y - HOLE_RADIUS),
    Vec2::new(WORLD_DIM.x / 2., HOLE_RADIUS),
    Vec2::new(WORLD_DIM.x / 2., WORLD_DIM.y - HOLE_RADIUS),
    Vec2::new(WORLD_DIM.x - HOLE_RADIUS, HOLE_RADIUS),
    Vec2::new(WORLD_DIM.x - HOLE_RADIUS, WORLD_DIM.y - HOLE_RADIUS),
];
const MAX_POS_TRY: i32 = 100;

const MAX_SPEED_SCALING: [f32; 5] = [0.005, 0.01, 0.02, 0.03, 0.04];
const START_MASS_SCALING: [f32; 5] = [0.1, 0.5, 1., 1.5, 3.];
const PROFITABILITY_SCALING: [u32; 5] = [1, 2, 5, 10, 25];
const SLIDING_SCALING: [f32; 5] = [0.9994, 0.99945, 0.9995, 0.99955, 0.9996];

pub struct World {
    pub balls: Vec<RefCell<entity::Ball>>,
    pub money: u32,
    pub round: u32,
    pub coins: Vec<Vec2>,
    game_over: bool,
    max_speed_level: usize,
    profitability_level: usize,
    sliding_level: usize,
}

impl World {
    pub fn new(
        max_speed_level: usize,
        profitability_level: usize,
        start_mass_level: usize,
        sliding_level: usize,
    ) -> Self {
        let mut new_world = World {
            balls: Vec::new(),
            money: 0,
            round: 1,
            game_over: false,
            max_speed_level,
            profitability_level,
            sliding_level,
            coins: Vec::new(),
        };
        new_world.add_ball(
            Vec2::new(WORLD_DIM.x / 4., WORLD_DIM.y / 2.),
            START_MASS_SCALING[start_mass_level] / 4.,
            START_MASS_SCALING[start_mass_level],
            SLIDING_SCALING[sliding_level],
            entity::BallType::Player,
        );
        new_world
    }

    fn spawn_coins(&mut self) {
        let coin_number: usize = rand::rand_range(1., 5.) as usize;
        for _ in 0..coin_number {
            let coin_pos = self.get_free_pos(COIN_RADIUS);
            self.coins.push(coin_pos);
        }
    }

    fn get_free_pos(&mut self, radius: f32) -> Vec2 {
        let x1 = HOLE_RADIUS + radius;
        let x2 = WORLD_DIM.x - HOLE_RADIUS - radius;
        let y1 = HOLE_RADIUS + radius;
        let y2 = WORLD_DIM.y - HOLE_RADIUS - radius;

        let mut pos_not_ok = true;
        let mut count = 0;
        let mut new_pos = Vec2::ZERO;

        while pos_not_ok {
            if count > MAX_POS_TRY {
                self.game_over = true;
                break;
            }
            new_pos = Vec2::new(
                rand::rand_range(x1 as f64, x2 as f64) as f32,
                rand::rand_range(y1 as f64, y2 as f64) as f32,
            );
            pos_not_ok = false;
            for index in 0..self.balls.len() {
                let ball = self.balls[index].borrow();

                let dist = ball.position - new_pos;

                if ball.radius + radius - dist.length() > 0. {
                    pos_not_ok = true;
                }
            }
            count += 1;
        }
        new_pos
    }

    pub fn spawn_round(&mut self) {
        for ball in &mut self.balls {
            if let BallType::Enemy(enemy_data) = &mut ball.borrow_mut().letypedelaboule {
                enemy_data.timer -= 1;
            }
            ball.borrow_mut().speed = Vec2::ZERO;
        }

        let new_mass = self.round as f32 * 0.05;
        let new_radius = new_mass / 4.;

        let new_friction_coeff = SLIDING_SCALING[self.sliding_level];
        let new_pos = self.get_free_pos(new_radius);

        self.add_ball(
            new_pos,
            new_radius,
            new_mass,
            new_friction_coeff,
            BallType::Enemy(EnemyData { timer: 5 }),
        );
    }

    pub fn is_game_over(&self) -> bool {
        self.game_over
    }

    fn add_ball(
        &mut self,
        position: Vec2,
        radius: f32,
        mass: f32,
        friction_coeff: f32,
        letypedelaboule: entity::BallType,
    ) {
        let new_ball = Ball {
            position,
            radius,
            mass,
            speed: Vec2::ZERO,
            friction_coeff,
            letypedelaboule,
        };
        self.balls.push(RefCell::new(new_ball));
    }

    pub fn tick(&mut self) -> bool {
        let mut trash = Vec::new();
        let mut new_balls = Vec::new();
        let mut coin_trash: Vec<usize> = Vec::new();

        for (index, ball_cell) in self.balls.iter().enumerate() {
            {
                let mut ball = ball_cell.borrow_mut();
                ball.position = ball.position + ball.speed;
                ball.speed = ball.speed * ball.friction_coeff;
                Self::check_border(&mut ball);
                if self.in_hole(&ball) {
                    trash.push(index);
                }
            }
            for other_ball_index in index + 1..self.balls.len() {
                if let Some((a, b, new_ball)) = self.collide(index, other_ball_index) {
                    trash.push(a);
                    trash.push(b);
                    new_balls.push(RefCell::new(new_ball));
                }
            }

            for (coin_index, coin) in self.coins.iter().enumerate() {
                let ball = ball_cell.borrow();
                if ball.radius + COIN_RADIUS - (ball.position - coin).length() > 0. {
                    coin_trash.push(coin_index);
                    self.money += COIN_PRICE * PROFITABILITY_SCALING[self.profitability_level];
                }
            }
        }

        trash.sort();
        coin_trash.sort();

        for bye_bye in coin_trash.into_iter().rev() {
            self.coins.remove(bye_bye);
        }
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

        let mut player_ball = false;
        let mut counter_ball_low = false;

        for ball in &self.balls {
            if let BallType::Enemy(enemy_data) = ball.borrow().letypedelaboule {
                if enemy_data.timer < 1 {
                    counter_ball_low = true
                }
            } else {
                player_ball = true;
            }
        }
        self.game_over |= !player_ball | counter_ball_low;

        false
    }

    fn in_hole(&self, ball: &entity::Ball) -> bool {
        for hole in &HOLES {
            if HOLE_RADIUS > ball.position.distance(*hole) {
                return true;
            }
        }
        false
    }

    pub fn launch_round(&mut self, velocities: BTreeMap<usize, Vec2>) {
        let mut new_balls = vec![];
        for (index, velocity) in &velocities {
            let ball = self.balls[*index].borrow();
            let velocity = *velocity * MAX_SPEED_SCALING[self.max_speed_level];
            let ball1 = Ball::new(
                ball.mass / 2.,
                ball.position + velocity.normalize_or_zero() * ball.radius,
                velocity,
                ball.friction_coeff,
                ball.radius / 2f32.sqrt(),
                ball.letypedelaboule,
            );
            let ball2 = Ball::new(
                ball.mass / 2.,
                ball.position - velocity.normalize_or_zero() * ball.radius,
                velocity * -1.,
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
                let tot_mass = ball_a.mass + ball_b.mass;
                let new_ball = Ball {
                    mass: tot_mass,
                    position: (ball_a.position * ball_a.mass + ball_b.position * ball_b.mass)
                        / (tot_mass),
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
                ball_a.speed = ball_a.speed + push * ball_b.mass / ball_a.mass;
                ball_b.speed = ball_b.speed - push * ball_a.mass / ball_b.mass;
            }
        }
        None
    }
}
