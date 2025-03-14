use glam::Vec2;

#[derive(Clone, Copy)]
pub struct Ball{
    pub mass: f32,
    pub position: Vec2,
    pub speed: Vec2,
    pub bouncing_coefficient: f32,
    pub friction_coeff: f32,
}

impl Ball {
    fn new(mass:f32, position: Vec2, speed:Vec2, bouncing_coefficient: f32, speed_decrease: f32, acceleration: Vec2) -> Ball {
        Ball {
            mass: mass,
            position: position,
            speed: speed,
            bouncing_coefficient: bouncing_coefficient,
            friction_coeff: speed_decrease,
        }
    }


    }   

