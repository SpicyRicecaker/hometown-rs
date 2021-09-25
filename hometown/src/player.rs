use thomas::cgmath;
use thomas::{context::Context, frontend::camera_controller::CameraController};

pub struct Player {
    x: f32,
    dx: f32,
    y: f32,
    dy: f32,
    z: f32,
    dz: f32,
    velocity: f32,
    direction: cgmath::Vector3<f32>,
    camera_controller: CameraController,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            x: 0.0,
            dx: 0.0,
            y: 0.0,
            dy: 0.0,
            z: 0.0,
            dz: 0.0,
            velocity: 5.0,
            // Looking into rain?
            direction: cgmath::Vector3 {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            },
            camera_controller: CameraController::new(0.5),
        }
    }
}

impl Player {
    fn input(&mut self, ctx: &mut Context) {
        self.dx = 0.0;
        self.dy = 0.0;
        self.dz = 0.0;
        // W moves player forward (+z)
        if ctx.keyboard.w {
            self.dz = self.velocity;
        }
        // S moves player backward (-z)
        if ctx.keyboard.s {
            self.dz = -self.velocity;
        }
        // A moves player left (-x)
        if ctx.keyboard.a {
            self.dx = -self.velocity;
        }
        // D moves player right (+x)
        if ctx.keyboard.d {
            self.dx = self.velocity;
        }
        // Space moves player up (-y)
        if ctx.keyboard.space {
            self.dy = -self.velocity;
        }
        // Shift moves player down
        if ctx.keyboard.shift {
            self.dy = self.velocity;
        }
    }
    pub fn tick(&mut self, _ctx: &mut Context) {
        self.input(_ctx);
        // Apply dx, dy, and dz
        self.x += self.dx;
        self.y += self.dy;
        self.z += self.dz;
    }
    pub fn render(&self, ctx: &mut Context) {
        // Debug player position
        ctx.debugger
            .queue(format!("pos: {}/{}/{}", self.x, self.y, self.z));
    }
}
