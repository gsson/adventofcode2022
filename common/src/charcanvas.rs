use crate::vec2i::{Bounds, Point, Size};
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Index, IndexMut};

pub struct CharCanvas {
    pub bounds: Bounds,
    empty: char,
    tiles: Vec<char>,
}

impl CharCanvas {
    #[inline]
    pub fn size(&self) -> Size {
        self.bounds.size()
    }

    #[inline]
    fn index(&self, p: Point) -> usize {
        self.bounds.index(p)
    }

    pub fn new(empty: char) -> Self {
        Self {
            empty,
            bounds: Bounds::new(0, -1, -1, 0),
            tiles: vec![],
        }
    }

    pub fn with_bounds(empty: char, bounds: impl Into<Bounds>) -> Self {
        let bounds = bounds.into();
        let tiles = bounds.size().area();
        Self {
            empty,
            bounds,
            tiles: vec![empty; tiles as usize],
        }
    }

    pub fn with_size(empty: char, size: impl Into<Size>) -> Self {
        let size = size.into();
        let bounds = Bounds::with_size(size);
        let tiles = bounds.size().area();
        Self {
            empty,
            bounds,
            tiles: vec![empty; tiles as usize],
        }
    }

    pub fn clear(&mut self) {
        self.tiles.fill(self.empty);
    }
}

impl Index<Point> for CharCanvas {
    type Output = char;

    fn index(&self, p: Point) -> &Self::Output {
        if self.bounds.contains(&p) {
            let index = self.index(p);
            &self.tiles[index]
        } else {
            &self.empty
        }
    }
}

impl IndexMut<Point> for CharCanvas {
    fn index_mut(&mut self, p: Point) -> &mut Self::Output {
        if !self.bounds.contains(&p) {
            let empty = self.empty;
            let bounds = self.bounds.extend_to(&p);
            let size = bounds.size();
            let mut tiles = vec![empty; size.area() as usize];
            let offset = self.bounds.top_left().vector(&bounds.top_left());
            let [old_width, old_height]: [i32; 2] = self.size().into();
            for row in 0..old_height {
                let src_start = (row * old_width) as usize;
                let src_end = src_start + old_width as usize;
                let dst_start = ((row + offset.y()) * size.width() + offset.x()) as usize;
                let dst_end = dst_start + old_width as usize;
                tiles[dst_start..dst_end].copy_from_slice(&self.tiles[src_start..src_end]);
            }
            *self = Self {
                empty,
                bounds,
                tiles,
            }
        }
        let index = self.index(p);
        &mut self.tiles[index]
    }
}

impl Default for CharCanvas {
    fn default() -> Self {
        Self::new('.')
    }
}

impl Debug for CharCanvas {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl Display for CharCanvas {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in self.bounds.iter_row_indices() {
            writeln!(f, "{}", String::from_iter(&self.tiles[row]))?;
        }
        Ok(())
    }
}

#[test]
fn test_charcanvas_resize() {
    let mut canvas = CharCanvas::default();
    assert_eq!(0, canvas.bounds.size().width());
    assert_eq!(0, canvas.bounds.size().height());
    assert_eq!("", canvas.to_string());

    let p0 = Point::new(0, 0);

    canvas[p0] = 'A';

    assert_eq!(p0, canvas.bounds.top_left());
    assert_eq!(p0, canvas.bounds.bottom_right());
    assert_eq!("A\n", canvas.to_string());

    let p1 = Point::new(2, 2);

    canvas[p1] = 'B';

    assert_eq!(p0, canvas.bounds.top_left());
    assert_eq!(p1, canvas.bounds.bottom_right());
    assert_eq!(
        "A..\n\
        ...\n\
        ..B\n",
        canvas.to_string()
    );
    let p2 = Point::new(-1, -1);

    canvas[p2] = 'C';

    assert_eq!(p2, canvas.bounds.top_left());
    assert_eq!(p1, canvas.bounds.bottom_right());
    assert_eq!(
        "C...\n\
        .A..\n\
        ....\n\
        ...B\n",
        canvas.to_string()
    );
}
