use ggez::graphics::window;
use ggez::Context;
use glam::*;

fn inner_size(ctx: &Context) -> Vec2 {
  let s = window(ctx).inner_size();
  Vec2::new(s.width as f32, s.height as f32)
}
