use std::ops::RangeInclusive;

use glam::*;

type Matrix = Vec<Vec<Cell>>;
type Pt = IVec2;
pub struct Grid {
  dim: Pt,
  matrix: Matrix,
}

impl Grid {
  pub fn new(dim: Pt) -> Self {
    let get_row = |x: i32| (0..dim.y).map(|y| Cell::new(Pt::from([x, y]))).collect();
    let matrix: Matrix = (0..dim.x)
      .map(get_row)
      .collect();
    Self { dim, matrix }
  }

  pub fn cell_at(&self, pt: Pt) -> Option<&Cell> {
    if self.in_range(pt) {
      return Some(&self.matrix[pt.x as usize][pt.y as usize])
    }
    None
  }

  pub fn in_range(&self, pt: Pt) -> bool {
    Pt::ZERO.cmple(pt).all() && pt.cmplt(self.dim).all()
  }

  pub fn points_arround<'a>(&'a self, pt: Pt) -> impl Iterator<Item = Pt> + 'a {
    const _RNG: RangeInclusive<i32> = -1..=1;
    _RNG
      .flat_map(move |i| {
        _RNG.map(move |j| Pt::from([i, j]) + pt)
      })
      .filter(move |p| self.in_range(*p) && p.ne(&pt))
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
    assert_eq!(cell.pos(), Pt::from([0,0]));
  }

  #[test]
  fn range() {
    let grid = grid();
    assert_eq!(grid.in_range(Pt::from([-1,0])), false);
    assert_eq!(grid.in_range(Pt::from([0,0])), true);
    assert_eq!(grid.in_range(Pt::from([5,5])), true);
    assert_eq!(grid.in_range(Pt::from([10,9])), false);
  }

  #[test]
  fn points_arround() {
    let grid = grid();
    let points: Vec<Pt> = grid.points_arround(Pt::from([0,0])).collect();
    assert_eq!(points.len(), 3);
    let points: Vec<Pt> = grid.points_arround(Pt::from([5,5])).collect();
    assert_eq!(points.len(), 8);
    let points: Vec<Pt> = grid.points_arround(Pt::from([0,1])).collect();
    assert_eq!(points.len(), 5);
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
}

#[derive(Debug)]
enum CellState {
  Dead,
  Alive,
}