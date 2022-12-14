use crate::vec2i::{Bounds, Indexer, Point, Size};
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Index, IndexMut};

pub struct Grid2d<T> {
    pub bounds: Bounds,
    indexer: Indexer,
    empty: T,
    tiles: Vec<T>,
}

impl<T> Grid2d<T> {
    #[inline]
    pub fn size(&self) -> Size {
        self.bounds.size()
    }

    #[inline]
    fn index(&self, p: Point) -> usize {
        self.indexer.index(&p)
    }

    #[inline]
    pub fn from_index(&self, index: usize) -> Point {
        self.indexer.from_index(index)
    }
}

impl<T: Copy> Grid2d<T> {
    pub fn new(empty: T) -> Self {
        Self {
            empty,
            bounds: Bounds::EMPTY,
            indexer: Indexer::new(&Bounds::EMPTY),
            tiles: vec![],
        }
    }

    pub fn from_parts(empty: T, bounds: Bounds, tiles: Vec<T>) -> Self {
        debug_assert_eq!(tiles.len(), bounds.size().area() as usize);
        let indexer = Indexer::new(&bounds);

        Self {
            bounds,
            indexer,
            empty,
            tiles,
        }
    }

    pub fn with_bounds(empty: T, bounds: impl Into<Bounds>) -> Self {
        let bounds = bounds.into();
        let tiles = bounds.size().area();
        let indexer = Indexer::new(&bounds);
        Self {
            empty,
            bounds,
            indexer,
            tiles: vec![empty; tiles as usize],
        }
    }

    pub fn with_size(empty: T, size: impl Into<Size>) -> Self {
        let size = size.into();
        let bounds = Bounds::with_size(size);
        let tiles = bounds.size().area();
        let indexer = Indexer::new(&bounds);

        Self {
            empty,
            bounds,
            indexer,
            tiles: vec![empty; tiles as usize],
        }
    }

    pub fn line(&mut self, p1: &Point, p2: &Point, tile: T) {
        let step: [i32; 2] = p2.vector(p1).into();
        let new_bounds = self.bounds.extend_to(p1).extend_to(p2);
        self.extend_to_bounds(&new_bounds);
        let i1 = self.indexer.index(p1);
        let i2 = self.indexer.index(p2);

        match step {
            [0, 0] => {
                self.tiles[i1] = tile;
            }
            [dx, 0] => {
                if dx > 0 {
                    self.tiles[i1..=i2].fill(tile);
                } else {
                    self.tiles[i2..=i1].fill(tile);
                }
            }
            [0, dy] => {
                let width = new_bounds.size().width() as usize;
                if dy > 0 {
                    for i in (i1..=i2).step_by(width) {
                        self.tiles[i] = tile
                    }
                } else {
                    for i in (i2..=i1).step_by(width) {
                        self.tiles[i] = tile
                    }
                }
            }
            _ => unreachable!("Unsupported line slope {:?}", step),
        }
    }

    fn extend_grid(&self, bounds: Bounds) -> Self {
        let size = bounds.size();
        let mut tiles = vec![self.empty; size.area() as usize];
        let offset = self.bounds.top_left().vector(&bounds.top_left());
        let [old_width, old_height]: [i32; 2] = self.size().into();
        for row in 0..old_height {
            let src_start = (row * old_width) as usize;
            let src_end = src_start + old_width as usize;
            let dst_start = ((row + offset.y()) * size.width() + offset.x()) as usize;
            let dst_end = dst_start + old_width as usize;
            tiles[dst_start..dst_end].copy_from_slice(&self.tiles[src_start..src_end]);
        }
        Self::from_parts(self.empty, bounds, tiles)
    }

    pub fn extend_to_bounds(&mut self, b: &Bounds) {
        let new_bounds = self.bounds.extend_to_bounds(b);
        if new_bounds != self.bounds {
            *self = self.extend_grid(new_bounds)
        }
    }

    pub fn extend_to_point(&mut self, p: &Point) {
        let bounds = self.bounds.extend_to(p);
        if bounds != self.bounds {
            *self = self.extend_grid(bounds);
        }
    }

    pub fn clear(&mut self) {
        self.tiles.fill(self.empty);
    }
}

impl<T> Index<Point> for Grid2d<T> {
    type Output = T;

    #[inline]
    fn index(&self, p: Point) -> &Self::Output {
        if self.bounds.contains(&p) {
            let index = Grid2d::index(self, p);
            &self.tiles[index]
        } else {
            &self.empty
        }
    }
}

impl<T: Copy> IndexMut<Point> for Grid2d<T> {
    #[inline]
    fn index_mut(&mut self, p: Point) -> &mut Self::Output {
        if !self.bounds.contains(&p) {
            self.extend_to_point(&p);
        }
        let index = self.index(p);
        &mut self.tiles[index]
    }
}

impl<T: Copy + Default> Default for Grid2d<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T: Display> Debug for Grid2d<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl<T: Display> Display for Grid2d<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in self.bounds.iter_row_indices() {
            self.tiles[row]
                .iter()
                .try_for_each(|t| f.pad(&t.to_string()))?;
            writeln!(f)?;
        }
        Ok(())
    }
}

#[test]
fn test_grid2d_resize() {
    let mut grid = Grid2d::new('.');
    assert_eq!(0, grid.bounds.size().width());
    assert_eq!(0, grid.bounds.size().height());
    assert_eq!("", grid.to_string());

    let p0 = Point::new(0, 0);

    grid[p0] = 'A';

    assert_eq!(p0, grid.bounds.top_left());
    assert_eq!(p0, grid.bounds.bottom_right());
    assert_eq!("A\n", grid.to_string());

    let p1 = Point::new(2, 2);

    grid[p1] = 'B';

    assert_eq!(p0, grid.bounds.top_left());
    assert_eq!(p1, grid.bounds.bottom_right());
    assert_eq!(
        "A..\n\
        ...\n\
        ..B\n",
        grid.to_string()
    );
    let p2 = Point::new(-1, -1);

    grid[p2] = 'C';

    assert_eq!(p2, grid.bounds.top_left());
    assert_eq!(p1, grid.bounds.bottom_right());
    assert_eq!(
        "C...\n\
        .A..\n\
        ....\n\
        ...B\n",
        grid.to_string()
    );
}
