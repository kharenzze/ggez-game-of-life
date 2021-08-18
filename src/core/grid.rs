use super::utils::inner_size;
use ggez::event::*;
use ggez::graphics::{Color, DrawMode, Rect, DrawParam};
use ggez::{graphics, Context, GameResult};
use glam::*;
use log::*;
use std::ops::RangeInclusive;

type Matrix = Vec<Vec<Cell>>;
type Pt = IVec2;
pub struct Grid {
  dim: Pt,
  matrix: Matrix,
  show_grid: bool,
}

impl Grid {
  pub fn new(dim: Pt) -> Self {
    let get_row = |x: i32| (0..dim.y).map(|y| Cell::new(Pt::from([x, y]))).collect();
    let matrix: Matrix = (0..dim.x).map(get_row).collect();
    Self {
      dim,
      matrix,
      show_grid: true,
    }
  }

  pub fn cell_at(&self, pt: Pt) -> Option<&Cell> {
    if self.in_range(pt) {
      return Some(&self.matrix[pt.x as usize][pt.y as usize]);
    }
    None
  }

  pub fn in_range(&self, pt: Pt) -> bool {
    Pt::ZERO.cmple(pt).all() && pt.cmplt(self.dim).all()
  }

  pub fn points_arround<'a>(&'a self, pt: Pt) -> impl Iterator<Item = Pt> + 'a {
    const _RNG: RangeInclusive<i32> = -1..=1;
    _RNG
      .flat_map(move |i| _RNG.map(move |j| Pt::from([i, j]) + pt))
      .filter(move |p| self.in_range(*p) && p.ne(&pt))
  }

  fn map_screen_to_grid(&self, win_size: Vec2, pos: Vec2) -> Pt {
    let cell_size = self.cell_size(win_size);
    let relative_pos = pos / cell_size;
    relative_pos.as_i32()
  }

  fn cell_size(&self, win_size: Vec2) -> Vec2 {
    let grid_size = self.dim.as_f32();
    win_size / grid_size
  }

  pub fn draw(&self, ctx: &mut Context) -> GameResult {
    let win_size = inner_size(ctx);
    let cell_size = self.cell_size(win_size);
    let mut mb = graphics::MeshBuilder::new();

    for col in &self.matrix {
      for cell in col {
        let rect = cell.to_rect(cell_size);
        let color = match cell.state {
          CellState::Alive => Color::GREEN,
          CellState::Dead => Color::BLACK,
        };
        mb.rectangle(DrawMode::fill(), rect, color)?;
        if self.show_grid {
          mb.rectangle(DrawMode::stroke(1.0), rect, Color::WHITE)?;
        }
      }
    }
    let mesh = mb.build(ctx)?;
    graphics::draw(ctx, &mesh, DrawParam::default())?;
    Ok(())
  }

  fn toggle_show_grid(&mut self) {
    self.show_grid = !self.show_grid;
  }

  pub fn handle_key_press(
    &mut self,
    _ctx: &mut Context,
    keycode: KeyCode,
    _keymod: KeyMods,
    _repeat: bool,
  ) {
    match keycode {
      KeyCode::G => {
        self.toggle_show_grid();
      }
      _ => (())
    }
  }

  pub fn handle_mouse_click(&mut self, ctx: &mut Context, button: MouseButton, pos: Vec2) {
    if button != MouseButton::Left {
      return
    }
    let win_size = inner_size(ctx);
    let cell_pt = self.map_screen_to_grid(win_size, pos);
    if let Some(cell) = self.cell_at(cell_pt) {
      cell.
    }
  }
}

#[derive(Debug)]
pub struct Cell {
  state: CellState,
  pos: Pt,
}

impl Cell {
  pub fn pos(&self) -> Pt {
    self.pos
  }
}

impl Cell {
  fn new(pos: Pt) -> Self {
    Self {
      state: CellState::Dead,
      pos,
    }
  }

  fn to_rect(&self, size: Vec2) -> Rect {
    Rect {
      x: size.x * self.pos.x as f32,
      y: size.y * self.pos.y as f32,
      h: size.x,
      w: size.y,
    }
  }

  fn toggle(&mut self) {
    self.state = match self.state {
      CellState::Alive => CellState::Dead,
      CellState::Dead => CellState::Alive,
    }
  }
}

#[derive(Debug)]
enum CellState {
  Dead,
  Alive,
}

#[cfg(test)]
mod tests {
  use super::*;
  fn grid() -> Grid {
    let dim = Pt::from([10, 10]);
    let grid = Grid::new(dim);
    return grid;
  }
  #[test]
  fn creation() {
    let grid = grid();
    let cell = grid.cell_at(Pt::from([0, 0])).unwrap();
    assert_eq!(cell.pos(), Pt::from([0, 0]));
  }

  #[test]
  fn range() {
    let grid = grid();
    assert_eq!(grid.in_range(Pt::from([-1, 0])), false);
    assert_eq!(grid.in_range(Pt::from([0, 0])), true);
    assert_eq!(grid.in_range(Pt::from([5, 5])), true);
    assert_eq!(grid.in_range(Pt::from([10, 9])), false);
  }

  #[test]
  fn points_arround() {
    let grid = grid();
    let points: Vec<Pt> = grid.points_arround(Pt::from([0, 0])).collect();
    assert_eq!(points.len(), 3);
    let points: Vec<Pt> = grid.points_arround(Pt::from([5, 5])).collect();
    assert_eq!(points.len(), 8);
    let points: Vec<Pt> = grid.points_arround(Pt::from([0, 1])).collect();
    assert_eq!(points.len(), 5);
  }

  #[test]
  fn map_screen_to_grid() {
    let grid = grid();
    let win_size = Vec2::from([100.0, 100.0]);
    let pos = Vec2::from([1.0, 1.0]);
    let x = grid.map_screen_to_grid(win_size, pos);
    assert_eq!(x, Pt::from([0, 0]));

    let pos = Vec2::from([99.0, 99.0]);
    let x = grid.map_screen_to_grid(win_size, pos);
    assert_eq!(x, Pt::from([9, 9]));
  }
}
