pub mod runnable;
use crate::{player::Player, rain};

pub struct Game {
    pub raindrops: Vec<rain::RainDropMachineWrapper>,

    player: Player,

    thickness: f32,
    velocity: f32,
    length: f32,

    pub x: f32,
    pub y: f32,
    pub z: f32,

    coord_range: crate::CoordRange,

    x_angle: f32,
    z_angle: f32,

    angle_range: crate::AngleRange,
}

impl Game {
    pub fn new() -> Self {
        let thickness = 2;
        let thickness = 4.0;
        let velocity = 10.0;
        let length = 50.0;

        let x = crate::WORLD_X;
        let y = crate::WORLD_Y;
        let z = crate::WORLD_Z;

        // Range at which the rain will initially spawn from
        // In angles from perpendicular down
        // Should change in accordance with wind
        // Based on degrees for now
        // let x_angle = 10.0;
        let x_angle = 0.0;
        // I'm thinking that rain moving front and back would be cool, but it's not a big deal if
        // the angle deg would be 0. In short, let user config lol
        // let x_angle = 1.0;
        let z_angle = 0.0;

        let angle_range = crate::AngleRange::new(x_angle, z_angle);

        let coord_range = crate::CoordRange::new(crate::WORLD_X, crate::WORLD_Y, crate::WORLD_Z);

        let player = crate::player::Player::default();

        Self {
            raindrops: Vec::new(),
            thickness,
            velocity,
            length,
            x,
            y,
            z,
            player,
            coord_range,
            x_angle,
            z_angle,
            angle_range,
        }
    }
    pub fn gen_raindrop(&mut self) {
        let raindropmachine = rain::RainDropMachine::<rain::Falling>::new(
            self.coord_range.x(),
            self.y,
            self.coord_range.z(),
            0.0,
            self.length,
            self.thickness,
            self.angle_range.x_deg(),
            self.angle_range.z_deg(),
            self.velocity,
        );
        let raindropmachinewrapper = rain::RainDropMachineWrapper::Falling(raindropmachine);
        self.raindrops.push(raindropmachinewrapper);
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
