use crate::{context::Context, graphics::backend::State};

const FONT_SIZE: f32 = 20.0;
const COLOR: wgpu::Color = wgpu::Color::GREEN;

// struct DebugInfo {
//     text: String,
// }

pub struct Debugger {
    queue: Vec<String>,
}

impl Debugger {
    pub fn new() -> Self {
        Debugger { queue: Vec::new() }
    }
    pub fn queue(&mut self, text: String) {
        self.queue.push(text)
    }
    pub fn render(&mut self, gfx: &mut State) {
        // Store current row's width
        let mut end_x = 0.0;
        let mut y = 0.0;
        let max_width = gfx.size.width as f32;

        self.queue.iter().for_each(|d| {
            // multiply d.len by font size for size of the sentence
            let debug_len = d.len() as f32 * FONT_SIZE;
            // add to current row len for a tentative size
            let predicted_len = debug_len + end_x;

            match predicted_len.partial_cmp(&max_width) {
                // if the tentative size is less than or equal to max width
                Some(std::cmp::Ordering::Less) | Some(std::cmp::Ordering::Equal) => {
                    // draw it at current x
                    gfx.draw_text(d, end_x, y, COLOR, FONT_SIZE);
                    // and set end x to the tentative len
                    end_x = predicted_len
                }
                // Otherwise
                _ => {
                    // increment current y
                    y += FONT_SIZE;
                    // draw at x=0, y=current_y
                    gfx.draw_text(d, 0.0, y, COLOR, FONT_SIZE);
                    // then set current row len to the debug_len
                    end_x = debug_len
                }
            }
        });
    }
}

impl Default for Debugger {
    fn default() -> Self {
        Self::new()
    }
}
