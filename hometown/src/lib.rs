use std::f32::consts::PI;

use rand::{distributions::Uniform, prelude::{Distribution, ThreadRng}};

pub mod rain;
pub mod game;
pub mod player;

/// There are a couple of ways to deal with multiple dimensions... One would be to define our own
/// type inside of thomas and export it
struct Frustrum {
    x: f32,
    y: f32,
    z: f32
}

/// Another wuold be to simply use cgmath
struct Any {
    any: thomas::cgmath::Vector3<f32>
}

// We'll have world be a 2000 x 2000 x 2000 box
const WORLD_X: f32 = 2000.0;
const WORLD_Y: f32 = 2000.0;
const WORLD_Z: f32 = 2000.0;

struct CoordRange {
    x_range: Uniform<f32>,
    y: f32,
    z_range: Uniform<f32>,
    rng: ThreadRng,
}

impl CoordRange {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            x_range: Uniform::new_inclusive(0.0, x),
            y,
            z_range: Uniform::new_inclusive(0.0, z),
            rng: rand::thread_rng(),
        }
    }

    pub fn x(&mut self) -> f32 {
        self.x_range.sample(&mut self.rng)
    }

    pub fn z(&mut self) -> f32 {
        self.z_range.sample(&mut self.rng)
    }
}

struct AngleRange {
    x_range: Uniform<f32>,
    z_range: Uniform<f32>,
    rng: ThreadRng,
}

impl AngleRange {
    fn new(x_deg: f32, z_deg: f32) -> Self {
        // Remember that it's pointing down
        // Convert to radians, cause
        let down = 3.0 * PI / 2.0;
        let x_rad = x_deg.to_radians();
        let z_rad = z_deg.to_radians();
        Self {
            x_range: Uniform::new_inclusive(down - x_rad, down + x_rad),
            z_range: Uniform::new_inclusive(down - z_rad, down + z_rad),
            rng: rand::thread_rng(),
        }
    }
    pub fn x_deg(&mut self) -> f32 {
        self.x_range.sample(&mut self.rng)
    }
    pub fn z_deg(&mut self) -> f32 {
        self.z_range.sample(&mut self.rng)
    }
}
