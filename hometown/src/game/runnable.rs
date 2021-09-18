use thomas::{Runnable, context::Context};

use super::Game;

impl Runnable for Game {
    fn tick(&mut self, ctx: &mut Context) {
        self.raindrops.iter_mut().for_each(|f| f.tick(ctx));
    }
    fn render(&self, ctx: &mut Context) {
        self.raindrops.iter().for_each(|f| f.render(ctx));

        // Draw a debug bounding box for the world
        ctx.graphics.draw_square(
            0.0,
            0.0,
            2000.0,
            thomas::frontend::color::Color {
                r: 256,
                g: 256,
                b: 256,
                a: 20,
            },
        );
    }
}