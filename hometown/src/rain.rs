use rand::{distributions::Uniform, prelude::ThreadRng};
use thomas::{context::Context, frontend::color::Color};

#[derive(Debug)]
pub enum RainDropMachineWrapper {
    Falling(RainDropMachine<Falling>),
    Colliding(RainDropMachine<Colliding>),
    Spreading(RainDropMachine<Spreading>),
    Dead,
}

impl RainDropMachineWrapper {
    pub fn tick(&mut self, ctx: &mut Context) {
        match self {
            RainDropMachineWrapper::Falling(r) => r.tick(ctx),
            RainDropMachineWrapper::Colliding(r) => r.tick(ctx),
            RainDropMachineWrapper::Spreading(r) => r.tick(ctx),
            RainDropMachineWrapper::Dead => todo!(),
        }
    }
    pub fn render(&self, ctx: &mut Context) {
        match self {
            RainDropMachineWrapper::Falling(r) => r.render(ctx),
            RainDropMachineWrapper::Colliding(r) => r.render(ctx),
            RainDropMachineWrapper::Spreading(r) => r.render(ctx),
            RainDropMachineWrapper::Dead => todo!(),
        }
    }
}

#[derive(Debug)]
pub struct RainDropMachine<T> {
    pub state: T,
}

impl RainDropMachine<Falling> {
    /// A choice between having a starting and stopping coord, and mutating them both, or just having a starting coord and having to double calculate stuff...
    /// Also calculating cos 2 blows. We should cache it and update it when it changes
    pub fn new(
        x: f32,
        y: f32,
        z: f32,
        endy: f32,
        length: f32,
        thickness: f32,
        x_rad: f32,
        z_rad: f32,
        velocity: f32,
    ) -> Self {
        Self {
            state: Falling {
                x,
                y,
                z,
                endy,
                length,
                thickness,
                x_rad,
                z_rad,
                velocity,
            },
        }
    }
    pub fn tick(&mut self, ctx: &mut Context) {
        let cos = self.state.x_rad.cos();
        let sin = self.state.z_rad.sin();
        // Generate unit vector
        let x = self.state.velocity * cos / 2.0;
        let y = self.state.velocity * (cos + sin) / 2.0;
        let z = self.state.velocity * sin / 2.0;
        self.state.x += x;
        self.state.y += y;
        self.state.z += z;
    }
    pub fn render(&self, ctx: &mut Context) {
        let cos = self.state.x_rad.cos();
        let sin = self.state.z_rad.sin();
        // Generate unit vector
        let x = self.state.length * cos / 2.0;
        let y = self.state.length * (cos + sin) / 2.0;
        let z = self.state.length * sin / 2.0;

        // dbg!((self.state.x, self.state.y, self.state.z, x, y,z, self.state.thickness, Color::from_hex("FFFFFF").unwrap()));

        // dbg!(self.state.x, self.state.y, self.state.z, self.state.x + x, self.state.y + y, self.state.z + z, self.state.thickness, Color::from_hex("FFFFFF").unwrap());

        ctx.graphics.draw_line_3d(
            self.state.x,
            2000.0 - self.state.y,
            self.state.z,
            self.state.x + x,
            2000.0 - (self.state.y + y),
            self.state.z + z,
            self.state.thickness,
            Color::from_hex("FFFFFF").unwrap(),
        )
    }
}

#[derive(Debug)]
pub struct Falling {
    // Basically the spawn location of the object in the frustrum
    // There should be some ranges for x, y and z when we spawn
    pub x: f32,
    pub y: f32,
    // The z covers the bottom of the raindrop
    z: f32,
    // The point at which the drop goes into puddle
    endy: f32,
    // The length of the drop gets modified as it goes into the puddle
    length: f32,
    thickness: f32,
    // We need all of these because the raindrop could be falling at an angle
    x_rad: f32,
    z_rad: f32,
    velocity: f32, // Could add some bezier curve to make it more natural later (acceleration)
}
// impl Falling {
//     pub fn tick(&mut self, ctx: &mut Context) {}
//     pub fn render(&self, ctx: &mut Context) {}
// }

#[derive(Debug)]
pub struct Colliding {
    // Length, should get modified as its being inserted into the puddle
    length: u32,
    x: f32,
    y: f32,
    // The z is fixed at this point
    z: f32,
    dx: f32,
    dy: f32,
    dz: f32,
    // We no know z declares the tip of the water
    thickness: u32,
    // At this point we should also be holding a custom spreading struct, maybe with a high
    // velocity
    spreading: Spreading,
}

impl RainDropMachine<Colliding> {
    pub fn tick(&mut self, ctx: &mut Context) {}
    pub fn render(&self, ctx: &mut Context) {}
}

#[derive(Debug)]
pub struct Spreading {
    // The location of the puddle
    x: f32,
    y: f32,
    z: f32,
    // The thickness of the edges of the water
    thickness: u32,
    // The radius
    r: f32,
    // The rate at which the radius is changing
    dr: u32,
    // Might even add a ddr for fun later
    // Pretty important
    lifetime: u32,
}

impl RainDropMachine<Spreading> {
    pub fn tick(&mut self, ctx: &mut Context) {}
    pub fn render(&self, ctx: &mut Context) {}
}
