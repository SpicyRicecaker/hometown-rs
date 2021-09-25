use thomas::frontend::color::Color;
use thomas::{context::Context, Runnable};

use super::Game;

const BLACK: Color = Color {
    r: 0,
    g: 0,
    b: 0,
    a: 256,
};

impl Runnable for Game {
    fn tick(&mut self, ctx: &mut Context) {
        self.raindrops.iter_mut().for_each(|f| f.tick(ctx));

        // tick player
        self.player.tick(ctx);
    }
    fn render(&self, ctx: &mut Context) {
        self.raindrops.iter().for_each(|f| f.render(ctx));

        // Draw a debug bounding box for the world
        ctx.graphics.draw_square(0.0, 0.0, 2000.0, BLACK.fade(0.2));

        // render player
        self.player.render(ctx);

        ctx.graphics.clear_background(BLACK);
    }
}
