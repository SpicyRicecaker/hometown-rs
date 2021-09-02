use hometown::rain::{self, Falling, RainDropMachine, RainDropMachineWrapper};
use rand::{
    distributions::Uniform,
    prelude::{Distribution, ThreadRng},
};
use std::f32::consts::PI;
use thomas::context::Context;
use thomas::Runnable;

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
    z_angle: f32,

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
        // let x_angle = 10.0;
        let x_angle = 0.0;
        // I'm thinking that rain moving front and back would be cool, but it's not a big deal if
        // the angle deg would be 0. In short, let user config lol
        // let x_angle = 1.0;
        let z_angle = 0.0;

        let angle_range = AngleRange::new(x_angle, z_angle);

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
            z_angle,
            angle_range,
        }
    }
    fn gen_raindrop(&mut self) {
        let raindropmachine = RainDropMachine::<Falling>::new(
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
    (0..2).into_iter().for_each(|_| game.gen_raindrop());
    game.raindrops.iter().for_each(|f| match f {
        RainDropMachineWrapper::Falling(e) => {
            dbg!(e.state.x, e.state.y);
        }
        _ => {}
    });

    let mut g = 10.0;

    let (ctx, cb) = thomas::ContextBuilder::new()
        .with_world_dimension((game.x, game.y, game.z))
        .build();

    thomas::main::run(ctx, cb, game);
}
