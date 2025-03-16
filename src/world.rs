use std::{
    cell::RefCell,
    collections::{BTreeMap, BTreeSet},
};

use crate::entity::{self, Ball, BallType, EnemyData};
use glam::Vec2;
use marmalade::{console, rand};

const COLLISION_SMOOTHNESS: f32 = 0.03;
pub const COIN_RADIUS: f32 = 0.01;
const COIN_PRICE: u64 = 100;
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

const MAX_SPEED_SCALING: [f32; 5] = [0.01, 0.01, 0.02, 0.03, 0.04];
const START_MASS_SCALING: [f32; 5] = [0.3, 0.5, 1., 1.5, 3.];
const PROFITABILITY_SCALING: [u64; 5] = [1, 2, 5, 10, 25];
const SLIDING_SCALING: [f32; 5] = [0.9994, 0.99945, 0.9995, 0.99955, 0.9996];

pub const ENEMY_ROUND: [usize; 20] = [1, 0, 1, 0, 1, 2, 1, 0, 3, 1, 0, 2, 0, 1, 2, 0, 1, 2, 3, 1];

pub const PLAYER_START_SIZE: f32 = 0.05;
pub const ENEMY_BALL_SIZE: f32 = 0.035;
const ENEMY_MASS: f32 = 0.15;

#[derive(PartialEq, PartialOrd, Eq, Ord)]
pub enum Sounds {
    SlimeSlime,
    Coin,
    SlimeEnemy,
    EnemyEnemy,
    Border,
    Falling,
}

pub struct World {
    pub balls: Vec<RefCell<entity::Ball>>,
    pub money: u64,
    pub round: usize,
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
            round: 0,
            game_over: false,
            max_speed_level,
            profitability_level,
            sliding_level,
            coins: Vec::new(),
        };
        new_world.add_ball(
            Vec2::new(WORLD_DIM.x / 2., WORLD_DIM.y / 2.),
            PLAYER_START_SIZE,
            START_MASS_SCALING[start_mass_level],
            SLIDING_SCALING[sliding_level],
            entity::BallType::Player,
        );

        new_world.add_ball(
            Vec2::new(WORLD_DIM.x, WORLD_DIM.y) - Vec2::splat(2. * HOLE_RADIUS),
            ENEMY_BALL_SIZE,
            ENEMY_MASS,
            SLIDING_SCALING[sliding_level],
            entity::BallType::Enemy(EnemyData { timer: 5 }),
        );

        new_world.add_ball(
            Vec2::ZERO + Vec2::splat(2. * HOLE_RADIUS),
            ENEMY_BALL_SIZE,
            ENEMY_MASS,
            SLIDING_SCALING[sliding_level],
            entity::BallType::Enemy(EnemyData { timer: 5 }),
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

    fn spawn_enemies(&mut self) {
        console::log(&format!("{}", ENEMY_ROUND[self.round]));
        for _ in 0..ENEMY_ROUND[self.round] {
            let new_friction_coeff = SLIDING_SCALING[self.sliding_level];
            let new_pos = self.get_free_pos(ENEMY_BALL_SIZE);

            self.add_ball(
                new_pos,
                ENEMY_BALL_SIZE,
                ENEMY_MASS,
                new_friction_coeff,
                BallType::Enemy(EnemyData { timer: 5 }),
            );
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
        self.spawn_coins();

        for ball in &mut self.balls {
            if let BallType::Enemy(enemy_data) = &mut ball.borrow_mut().letypedelaboule {
                enemy_data.timer -= 1;
                if enemy_data.timer < 1 {
                    self.game_over = true
                }
            }
            ball.borrow_mut().speed = Vec2::ZERO;
        }
        self.spawn_enemies();
        self.round += 1;
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

    pub fn tick(&mut self) -> (bool, BTreeSet<Sounds>) {
        let mut trash = Vec::new();
        let mut new_balls = Vec::new();
        let mut coin_trash: Vec<usize> = Vec::new();
        let mut sounds: BTreeSet<Sounds> = BTreeSet::new();
        for (index, ball_cell) in self.balls.iter().enumerate() {
            {
                let mut ball = ball_cell.borrow_mut();
                ball.position = ball.position + ball.speed;
                ball.speed = ball.speed * ball.friction_coeff;
                Self::check_border(&mut ball, &mut sounds);
                if self.in_hole(&ball) {
                    trash.push(index);
                    sounds.insert(Sounds::Falling);
                }
            }
            for other_ball_index in index + 1..self.balls.len() {
                if let Some((a, b, new_ball)) = self.collide(index, other_ball_index, &mut sounds) {
                    trash.push(a);
                    trash.push(b);
                    new_balls.push(RefCell::new(new_ball));
                }
            }

            for (coin_index, coin) in self.coins.iter().enumerate() {
                let ball = ball_cell.borrow();
                if ball.letypedelaboule == BallType::Player {
                    if ball.radius + COIN_RADIUS - (ball.position - coin).length() > 0. {
                        coin_trash.push(coin_index);
                        sounds.insert(Sounds::Coin);
                        self.money += COIN_PRICE * PROFITABILITY_SCALING[self.profitability_level];
                    }
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
            if ball.speed.length() > 0.00003 {
                return (true, sounds);
            }
        }

        let mut player_ball = false;

        for ball in &self.balls {
            if ball.borrow().letypedelaboule == BallType::Player {
                player_ball = true;
            }
        }
        self.game_over |= !player_ball;

        (false, sounds)
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
    }

    fn check_border(ball: &mut entity::Ball, sounds: &mut BTreeSet<Sounds>) {
        if ball.position.x - ball.radius < 0. {
            ball.position.x = ball.radius;
            ball.speed.x = -ball.speed.x;
            sounds.insert(Sounds::Border);
        } else if ball.position.x + ball.radius > WORLD_DIM.x {
            ball.position.x = WORLD_DIM.x - ball.radius;
            ball.speed.x = -ball.speed.x;
            sounds.insert(Sounds::Border);
        }

        if ball.position.y - ball.radius < 0. {
            ball.position.y = ball.radius;
            ball.speed.y = -ball.speed.y;
            sounds.insert(Sounds::Border);
        } else if ball.position.y + ball.radius > WORLD_DIM.y {
            ball.position.y = WORLD_DIM.y - ball.radius;
            ball.speed.y = -ball.speed.y;
            sounds.insert(Sounds::Border);
        }
    }

    pub fn collide(
        &self,
        a: usize,
        b: usize,
        sounds: &mut BTreeSet<Sounds>,
    ) -> Option<(usize, usize, Ball)> {
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
                sounds.insert(Sounds::SlimeSlime);

                let smaller = a.min(b);
                let bigger = a.max(b);
                return Some((smaller, bigger, new_ball));
            } else {
                let push = dist.normalize_or_zero() * overlap * COLLISION_SMOOTHNESS;
                ball_a.speed = ball_a.speed + push * ball_b.mass / ball_a.mass;
                ball_b.speed = ball_b.speed - push * ball_a.mass / ball_b.mass;
                if ball_a.letypedelaboule == ball_b.letypedelaboule {
                    sounds.insert(Sounds::EnemyEnemy);
                } else {
                    sounds.insert(Sounds::SlimeEnemy);
                }
            }
        }
        None
    }
}
