use ggez::error::{GameError, GameResult};
use ggez::event::EventHandler;
use ggez::graphics::Color;
use ggez::*;

pub struct MainState {}

impl EventHandler<GameError> for MainState {
  fn update(&mut self, ctx: &mut ggez::Context) -> GameResult {
    const TARGET_FPS: u32 = 60;
    while timer::check_update_time(ctx, TARGET_FPS) {
      //update objs
    }
    Ok(())
  }

  fn draw(&mut self, ctx: &mut ggez::Context) -> GameResult {
    graphics::clear(ctx, Color::BLACK);
    graphics::present(ctx)?;
    timer::yield_now();
    Ok(())
  }
}
