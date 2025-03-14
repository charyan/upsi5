use glam::Vec2;

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum BallType {
    Player,
    Enemy(EnemyData)
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct EnemyData {
    pub price: u32
}

#[derive(Clone, Copy)]
pub struct Ball{
    pub mass: f32,
    pub position: Vec2,
    pub speed: Vec2,
    pub friction_coeff: f32,
    pub radius: f32,
    pub letypedelaboule: BallType,
}

impl Ball {
    fn new(mass:f32, position: Vec2, speed:Vec2, friction_coeff: f32, radius: f32, letypedelaboule: BallType) -> Ball {
        Ball {
            mass,
            position,
            speed,
            friction_coeff,
            radius,
            letypedelaboule
        }
    }
}   

