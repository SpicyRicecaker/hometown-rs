use std::f32::consts::PI;
use hometown::rain::{self, Falling, RainDropMachine, RainDropMachineWrapper};
use rand::{
    distributions::Uniform,
    prelude::{Distribution, ThreadRng},
};
use thomas::context::Context;
use thomas::Runnable;

// We'll have world be a 2000 x 2000 x 2000 box
const WORLD_X: f32 = 2000.0;
const WORLD_Y: f32 = 2000.0;
const WORLD_Z: f32 = 2000.0;

struct CoordRange {
    x_range: Uniform<f32>,
    y_range: Uniform<f32>,
    z: f32,
    rng: ThreadRng,
}

impl CoordRange {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            x_range: Uniform::new_inclusive(0.0, x),
            y_range: Uniform::new_inclusive(0.0, y),
            z,
            rng: rand::thread_rng(),
        }
    }

    pub fn x(&mut self) -> f32 {
        self.x_range.sample(&mut self.rng)
    }

    pub fn y(&mut self) -> f32 {
        self.y_range.sample(&mut self.rng)
    }
}

struct AngleRange {
    x_range: Uniform<f32>,
    y_range: Uniform<f32>,
    rng: ThreadRng,
}

impl AngleRange {
    fn new(x_deg: f32, y_deg: f32) -> Self {
        // Remember that it's pointing down
        // Convert to radians, cause
        let down = 3.0 * PI / 2.0;
        let x_rad = x_deg.to_radians();
        let y_rad = y_deg.to_radians();
        Self {
            x_range: Uniform::new_inclusive(down - x_rad, down + x_rad),
            y_range: Uniform::new_inclusive(down - y_rad, down + y_rad),
            rng: rand::thread_rng(),
        }
    }
    pub fn x_deg(&mut self) -> f32 {
        self.x_range.sample(&mut self.rng)
    }
    pub fn y_deg(&mut self) -> f32 {
        self.y_range.sample(&mut self.rng)
    }
}

struct Game {
    raindrops: Vec<rain::RainDropMachineWrapper>,

    thickness: f32,
    velocity: f32,
    length: f32,

    x: f32,
    y: f32,
    z: f32,

    coord_range: CoordRange,

    x_angle: f32,
    y_angle: f32,

    angle_range: AngleRange,
}

impl Game {
    fn new() -> Self {
        let thickness = 2;
        let thickness = 4.0;
        let velocity = 10.0;
        let length = 50.0;

        let x = WORLD_X;
        let y = WORLD_Y;
        let z = WORLD_Z;

        let coord_range = CoordRange::new(WORLD_X, WORLD_Y, WORLD_Z);
        // Range at which the rain will initially spawn from
        // In angles from perpendicular down
        // Should change in accordance with wind
        // Based on degrees for now
        let x_angle = 10.0;
        let y_angle = 1.0;

        let angle_range = AngleRange::new(x_angle, y_angle);

        Self {
            raindrops: Vec::new(),
            thickness,
            velocity,
            length,
            x,
            y,
            z,
            coord_range,
            x_angle,
            y_angle,
            angle_range,
        }
    }
    fn gen_raindrop(&mut self) {
        let raindropmachine = RainDropMachine::<Falling>::new(
            self.coord_range.x(),
            self.coord_range.y(),
            self.z,
            0.0,
            self.length,
            self.thickness,
            self.angle_range.x_deg(),
            self.angle_range.y_deg(),
            self.velocity,
        );
        let raindropmachinewrapper = RainDropMachineWrapper::Falling(raindropmachine);
        self.raindrops.push(raindropmachinewrapper);
    }
}

impl Runnable for Game {
    fn tick(&mut self, ctx: &mut Context) {
        self.raindrops.iter_mut().for_each(|f| f.tick(ctx));
    }
    fn render(&self, ctx: &mut Context) {
        self.raindrops.iter().for_each(|f| f.render(ctx));
    }
}

fn main() {
    let mut game = Game::new();
    (0..10).into_iter().for_each(|_| game.gen_raindrop());

    let (ctx, cb) = thomas::ContextBuilder::new().build();

    thomas::main::run(ctx, cb, game);
}
