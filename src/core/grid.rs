use super::utils::inner_size;
use ggez::event::*;
use ggez::graphics::{Color, DrawMode, DrawParam, Rect};
use ggez::{graphics, Context, GameResult};
use glam::*;
use std::ops::RangeInclusive;
use std::time::Instant;

#[allow(unused_imports)]
use log::*;

type Matrix = Vec<Vec<Cell>>;
type Pt = IVec2;
pub struct Grid {
  dim: Pt,
  matrix: Matrix,
  matrix_next: Matrix,
  show_grid: bool,
  mode: GameMode,
}

#[derive(Debug, PartialEq, Eq)]
pub enum GameMode {
  Initialization,
  Playing,
}

impl Grid {
  fn get_empty_matrix(dim: Pt) -> Matrix {
    let get_row = |x: i32| (0..dim.y).map(|y| Cell::new(Pt::from([x, y]))).collect();
    let matrix: Matrix = (0..dim.x).map(get_row).collect();
    matrix
  }
  pub fn new(dim: Pt) -> Self {
    let matrix = Self::get_empty_matrix(dim);
    Self {
      dim,
      matrix_next: matrix.to_vec(),
      matrix,
      show_grid: true,
      mode: GameMode::Initialization,
    }
  }

  pub fn cell_at(&self, pt: Pt) -> Option<&Cell> {
    if self.in_range(pt) {
      return Some(&self.matrix[pt.x as usize][pt.y as usize]);
    }
    None
  }

  pub fn mut_cell_at(&mut self, pt: Pt) -> Option<&mut Cell> {
    if self.in_range(pt) {
      return Some(&mut self.matrix[pt.x as usize][pt.y as usize]);
    }
    None
  }

  pub fn in_range(&self, pt: Pt) -> bool {
    Pt::ZERO.cmple(pt).all() && pt.cmplt(self.dim).all()
  }

  pub fn points_around<'a>(&'a self, pt: Pt) -> impl Iterator<Item = Pt> + 'a {
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

  fn toggle_matrices(&mut self) {
    std::mem::swap(&mut self.matrix_next, &mut self.matrix);
  }

  pub fn update(&mut self, _ctx: &mut Context) -> GameResult {
    let now = Instant::now();
    if self.mode != GameMode::Playing {
      return Ok(());
    }
    self.compute_next();
    self.toggle_matrices();
    info!("Update:\t{}us", now.elapsed().as_micros());
    Ok(())
  }

  pub fn draw(&self, ctx: &mut Context) -> GameResult {
    let now = Instant::now();
    let win_size = inner_size(ctx);
    let cell_size = self.cell_size(win_size);
    let mut mb = graphics::MeshBuilder::new();

    for col in &self.matrix {
      for cell in col {
        let rect = cell.to_rect(cell_size);
        if cell.state == CellState::Alive {
          mb.rectangle(DrawMode::fill(), rect, Color::GREEN)?;
        }
        if self.show_grid {
          mb.rectangle(DrawMode::stroke(1.0), rect, Color::WHITE)?;
        }
      }
    }
    let mesh = mb.build(ctx)?;
    graphics::draw(ctx, &mesh, DrawParam::default())?;
    info!("Draw:\t{}us", now.elapsed().as_micros());
    Ok(())
  }

  fn toggle_show_grid(&mut self) {
    self.show_grid = !self.show_grid;
  }

  fn compute_next(&mut self) {
    let mut next: Matrix = vec![vec![]];
    std::mem::swap(&mut next, &mut self.matrix_next);
    for col in &mut next {
      for cell in col {
        let around: i32 = self
          .points_around(cell.pos)
          .map(|p| {
            let current = self.cell_at(p).unwrap();
            assert_eq!(current.pos, p);
            let value: i32 = current.state.into();
            value
          })
          .sum();
        cell.state = self.cell_at(cell.pos).unwrap().state.calc_next(around);
      }
    }
    std::mem::swap(&mut next, &mut self.matrix_next);
  }

  fn toggle_play(&mut self) {
    self.mode = match self.mode {
      GameMode::Initialization => GameMode::Playing,
      GameMode::Playing => GameMode::Initialization,
    }
  }

  fn reset(&mut self) {
    if self.mode != GameMode::Playing {
      self.matrix = Self::get_empty_matrix(self.dim);
    }
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
      KeyCode::P => {
        self.toggle_play();
      }
      KeyCode::R => {
        self.reset();
      }
      _ => (()),
    }
  }

  pub fn handle_mouse_click(&mut self, ctx: &mut Context, button: MouseButton, pos: Vec2) {
    if button != MouseButton::Left || self.mode != GameMode::Initialization {
      return;
    }
    let win_size = inner_size(ctx);
    let cell_pt = self.map_screen_to_grid(win_size, pos);
    if let Some(cell) = self.mut_cell_at(cell_pt) {
      cell.toggle();
    }
  }
}

#[derive(Debug, Clone)]
pub struct Cell {
  state: CellState,
  pos: Pt,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CellState {
  Dead,
  Alive,
}

impl CellState {
  fn calc_next(&self, around: i32) -> Self {
    match self {
      &CellState::Alive => {
        if around == 2 || around == 3 {
          CellState::Alive
        } else {
          CellState::Dead
        }
      }
      &CellState::Dead => {
        if around == 3 {
          CellState::Alive
        } else {
          CellState::Dead
        }
      }
    }
  }
}

impl From<CellState> for i32 {
  fn from(c: CellState) -> Self {
    match c {
      CellState::Alive => 1,
      CellState::Dead => 0,
    }
  }
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
    assert_eq!(cell.pos, Pt::from([0, 0]));
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
    let points: Vec<Pt> = grid.points_around(Pt::from([0, 0])).collect();
    assert_eq!(points.len(), 3);
    let points: Vec<Pt> = grid.points_around(Pt::from([5, 5])).collect();
    assert_eq!(points.len(), 8);
    let points: Vec<Pt> = grid.points_around(Pt::from([0, 1])).collect();
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

  #[test]
  fn vec() {
    let a = vec![vec![1, 2, 3]];
    let mut b = a.to_vec();
    b[0][0] = 7;
    println!("{:?}", &a);
    println!("{:?}", &b);
  }

  #[test]
  fn generation() {
    let dim = Pt::from([3, 3]);
    let mut grid = Grid::new(dim);
    for c in &mut grid.matrix[1] {
      c.state = CellState::Alive;
    }

    grid.compute_next();
    for i in 0..3 {
      assert_eq!(
        grid.matrix_next[i][1].state,
        CellState::Alive,
        "{} {}",
        i,
        1
      );
    }
  }
}
