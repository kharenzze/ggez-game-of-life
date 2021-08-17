use ggez::error::{GameError, GameResult};
use ggez::event::EventHandler;
use ggez::graphics::Color;
use ggez::*;
use super::grid::Grid;
use glam::IVec2;

pub struct MainState {
  grid: Grid
}

impl MainState {
  pub fn new() -> Self {
    Self {
      grid: Grid::new(IVec2::from([160, 90]))
    }
  }
}

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
