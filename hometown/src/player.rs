use thomas::cgmath;
use thomas::{context::Context, frontend::camera_controller::CameraController};

pub struct Player {
    x: f32,
    y: f32,
    z: f32,
    direction: cgmath::Vector3<f32>,
    camera_controller: CameraController,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
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
    fn input(ctx: &mut Context) {
        // W moves player forward (+z)
        if ctx.keyboard.w {

        }
        // A moves player left (-x)
        // S moves player backward (-x)
        // D moves player backward (-x)
    }
    fn tick(_ctx: &mut Context) {
    }
    fn render(ctx: &mut Context) {}
}
