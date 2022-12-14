use std::fmt::{Debug, Formatter};
use std::ops::{Add, Div, Mul, Neg, Range, Sub};
use std::simd::Which::{First, Second};
use std::simd::{simd_swizzle, Simd, SimdInt, SimdOrd, SimdPartialEq, SimdPartialOrd};
/*
  Note: This is for me to learn the basics of portable SIMD. It's unlikely to be more performant
  than using non-SIMD operations
*/

pub const ORIGIN: Point = Point::new(0, 0);

#[derive(Copy, Clone, Default, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Point(Simd<i32, 2>);

impl Point {
    const X: usize = 0;
    const Y: usize = 1;

    #[inline]
    pub const fn new(x: i32, y: i32) -> Self {
        Self(Simd::from_array([x, y]))
    }

    #[inline]
    pub const fn x(&self) -> i32 {
        self.0.as_array()[Point::X]
    }

    #[inline]
    pub const fn y(&self) -> i32 {
        self.0.as_array()[Point::Y]
    }

    #[inline]
    pub fn max_component(&self) -> i32 {
        self.0.reduce_max()
    }

    #[inline]
    pub fn vector(&self, other: &Point) -> Vector {
        Vector(self.0 - other.0)
    }

    #[inline]
    pub fn manhattan_distance(&self, other: &Point) -> i32 {
        self.vector(other).manhattan_len()
    }
}

impl From<Point> for [i32; 2] {
    fn from(value: Point) -> Self {
        value.0.to_array()
    }
}

impl AsRef<[i32; 2]> for Point {
    fn as_ref(&self) -> &[i32; 2] {
        self.0.as_array()
    }
}

impl AsMut<[i32; 2]> for Point {
    fn as_mut(&mut self) -> &mut [i32; 2] {
        self.0.as_mut_array()
    }
}

impl From<[i32; 2]> for Point {
    fn from(p: [i32; 2]) -> Self {
        Point(Simd::from_array(p))
    }
}

impl Add<Vector> for Point {
    type Output = Point;

    fn add(self, rhs: Vector) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub<Vector> for Point {
    type Output = Point;

    fn sub(self, rhs: Vector) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Debug for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Point")
            .field("x", &self.x())
            .field("y", &self.y())
            .finish()
    }
}

pub const UP: Vector = Vector::new(0, -1);
pub const UP_RIGHT: Vector = Vector::new(1, -1);
pub const RIGHT: Vector = Vector::new(1, 0);
pub const DOWN_RIGHT: Vector = Vector::new(1, 1);
pub const DOWN: Vector = Vector::new(0, 1);
pub const DOWN_LEFT: Vector = Vector::new(-1, 1);
pub const LEFT: Vector = Vector::new(-1, 0);
pub const UP_LEFT: Vector = Vector::new(-1, -1);

#[derive(Copy, Clone, Default, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Vector(Simd<i32, 2>);

impl Vector {
    const X: usize = 0;
    const Y: usize = 1;

    #[inline]
    pub const fn new(x: i32, y: i32) -> Self {
        Self(Simd::from_array([x, y]))
    }

    #[inline]
    pub const fn x(&self) -> i32 {
        self.0.as_array()[Self::X]
    }

    #[inline]
    pub const fn y(&self) -> i32 {
        self.0.as_array()[Self::Y]
    }

    #[inline]
    pub fn abs(&self) -> Self {
        Self(self.0.abs())
    }

    #[inline]
    pub fn max_component(&self) -> i32 {
        self.0.reduce_max()
    }

    #[inline]
    pub fn signum(&self) -> Self {
        Self(self.0.signum())
    }

    #[inline]
    pub fn manhattan_len(&self) -> i32 {
        self.0.abs().reduce_sum()
    }
}

impl const From<[i32; 2]> for Vector {
    fn from(p: [i32; 2]) -> Self {
        Self(Simd::from_array(p))
    }
}

impl Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Mul<i32> for Vector {
    type Output = Vector;

    fn mul(self, rhs: i32) -> Self::Output {
        Self(self.0 * Simd::splat(rhs))
    }
}

impl Div<i32> for Vector {
    type Output = Vector;

    fn div(self, rhs: i32) -> Self::Output {
        Self(self.0 / Simd::splat(rhs))
    }
}

impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Self::Output {
        Self(self.0.neg())
    }
}

impl Debug for Vector {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Vector")
            .field("x", &self.x())
            .field("y", &self.y())
            .finish()
    }
}

impl From<Vector> for [i32; 2] {
    fn from(value: Vector) -> Self {
        value.0.to_array()
    }
}

#[test]
fn test_manhattan_len() {
    assert_eq!(1, Vector::new(0, 1).manhattan_len());
    assert_eq!(2, Vector::new(1, 1).manhattan_len());
    assert_eq!(2, Vector::new(-1, 1).manhattan_len());
    assert_eq!(2, Vector::new(-1, -1).manhattan_len());
}

#[derive(Copy, Clone, Default, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Bounds(Point, Point);

impl Bounds {
    const SIZE_ADJUST: Simd<i32, 2> = Simd::from_array([1, 1]);

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0 .0.simd_gt(self.1 .0).any()
    }

    #[inline]
    pub const fn new(top: i32, right: i32, bottom: i32, left: i32) -> Self {
        Self(Point::new(left, top), Point::new(right, bottom))
    }

    #[inline]
    pub fn with_size(size: impl Into<Size>) -> Self {
        let size = size.into();
        Self(ORIGIN, Point(size.0 - Self::SIZE_ADJUST))
    }

    #[inline]
    pub fn extend_to(&self, p: &Point) -> Bounds {
        let top_left = self.top_left().0.simd_min(p.0);
        let bottom_right = self.bottom_right().0.simd_max(p.0);
        Bounds(Point(top_left), Point(bottom_right))
    }

    #[inline]
    pub const fn top_left(&self) -> Point {
        self.0
    }

    #[inline]
    pub const fn top_right(&self) -> Point {
        Point::new(self.right(), self.top())
    }

    #[inline]
    pub fn bottom_left(&self) -> Point {
        Point::new(self.left(), self.bottom())
    }

    #[inline]
    pub const fn bottom_right(&self) -> Point {
        self.1
    }

    #[inline]
    pub const fn top(&self) -> i32 {
        self.0.y()
    }

    #[inline]
    pub const fn right(&self) -> i32 {
        self.1.x()
    }

    #[inline]
    pub const fn bottom(&self) -> i32 {
        self.1.y()
    }

    #[inline]
    pub const fn left(&self) -> i32 {
        self.0.x()
    }

    pub fn cardinals(&self, p: Point) -> impl Iterator<Item = Point> {
        debug_assert!(self.contains(&p));
        let pointpoint = simd_swizzle!(p.0, [0, 1, 0, 1]);
        let lefttoprightbottom = simd_swizzle!(
            self.0 .0,
            self.1 .0,
            [First(0), First(1), Second(0), Second(1)]
        );
        let at_edge = pointpoint.simd_ne(lefttoprightbottom).to_array();

        at_edge
            .into_iter()
            .zip([LEFT, UP, RIGHT, DOWN])
            .filter_map(|(mask, direction)| mask.then_some(direction))
            .map(move |direction| p + direction)
    }

    #[inline]
    pub fn contains(&self, point: &Point) -> bool {
        point.0.simd_ge(self.0 .0).all() && point.0.simd_le(self.1 .0).all()
    }

    #[inline]
    pub fn index(&self, p: Point) -> usize {
        debug_assert!(self.contains(&p));
        ((p.0 - self.top_left().0) * Simd::from_array([1, self.size().width()])).reduce_sum()
            as usize
    }

    #[inline]
    pub fn from_index(&self, index: usize) -> Point {
        let width = self.size().width();
        let p = Point::new(index as i32 % width, index as i32 / width);
        debug_assert!(self.contains(&p));
        p
    }

    pub fn iter_indices(&self) -> IndexIter {
        IndexIter::new(*self)
    }

    pub fn iter_row_indices(&self) -> IndexRowIter {
        IndexRowIter::new(*self)
    }

    #[inline]
    pub fn size(&self) -> Size {
        Size(self.1 .0 - self.0 .0 + Self::SIZE_ADJUST)
    }

    #[inline]
    pub fn iter_points(&self) -> PointIter {
        PointIter::new(*self)
    }
}

pub struct PointIter {
    bounds: Bounds,
    step: Vector,
    wrap: Vector,
    point: Option<Point>,
}

impl PointIter {
    const EMPTY: PointIter = Self {
        bounds: Bounds::new(0, -1, -1, 0),
        step: Vector::new(0, 0),
        wrap: Vector::new(0, 0),
        point: None,
    };
    fn new(bounds: Bounds) -> Self {
        if bounds.is_empty() {
            Self::EMPTY
        } else {
            let size = bounds.size();
            Self {
                bounds,
                step: Vector::new(1, 0),
                wrap: Vector::new(-size.width() + 1, 1),
                point: Some(bounds.top_left()),
            }
        }
    }
}

impl Iterator for PointIter {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(point) = self.point {
            let xy: [i32; 2] = point.vector(&self.bounds.bottom_right()).signum().into();
            self.point = match xy {
                [-1, _] => Some(point + self.step),
                [_, -1] => Some(point + self.wrap),
                _ => None,
            };

            Some(point)
        } else {
            None
        }
    }
}

pub struct IndexIter {
    end: usize,
    next_index: usize,
}

impl IndexIter {
    const EMPTY: IndexIter = Self {
        end: 0,
        next_index: 1,
    };

    fn new(bounds: Bounds) -> Self {
        if bounds.is_empty() {
            Self::EMPTY
        } else {
            Self {
                end: bounds.index(bounds.bottom_right()),
                next_index: bounds.index(bounds.top_left()),
            }
        }
    }
}

impl Iterator for IndexIter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.next_index;
        if index > self.end {
            None
        } else {
            self.next_index += 1;
            Some(index)
        }
    }
}

pub struct IndexRowIter {
    end: usize,
    width: usize,
    next_row_start: usize,
}

impl IndexRowIter {
    const EMPTY: IndexRowIter = Self {
        end: 0,
        width: 0,
        next_row_start: 1,
    };

    fn new(bounds: Bounds) -> Self {
        if bounds.is_empty() {
            Self::EMPTY
        } else {
            let width = bounds.size().width() as usize;
            Self {
                end: bounds.index(bounds.bottom_right()),
                width,
                next_row_start: bounds.index(bounds.top_left()),
            }
        }
    }
}

impl Iterator for IndexRowIter {
    type Item = Range<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        let row_start = self.next_row_start;
        if row_start > self.end {
            None
        } else {
            let next_row_start = row_start + self.width;
            self.next_row_start = next_row_start;
            Some(row_start..next_row_start)
        }
    }
}

impl Debug for Bounds {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Bounds")
            .field("top", &self.top())
            .field("right", &self.right())
            .field("bottom", &self.bottom())
            .field("left", &self.left())
            .finish()
    }
}

#[derive(Copy, Clone, Default, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Size(Simd<i32, 2>);

impl Size {
    const WIDTH: usize = 0;
    const HEIGHT: usize = 1;

    #[inline]
    pub const fn new(width: i32, height: i32) -> Self {
        Self(Simd::from_array([width, height]))
    }

    #[inline]
    pub const fn width(&self) -> i32 {
        self.0.as_array()[Size::WIDTH]
    }

    #[inline]
    pub const fn height(&self) -> i32 {
        self.0.as_array()[Size::HEIGHT]
    }

    pub fn area(&self) -> i32 {
        self.0.reduce_product()
    }
}

impl From<Size> for [i32; 2] {
    fn from(value: Size) -> Self {
        value.0.to_array()
    }
}

impl From<[i32; 2]> for Size {
    fn from(value: [i32; 2]) -> Self {
        Self(Simd::from_array(value))
    }
}

impl Debug for Size {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Size")
            .field("width", &self.width())
            .field("height", &self.height())
            .finish()
    }
}

#[test]
fn test_bounds_is_empty() {
    assert!(!Bounds::new(0, 1, 1, 0).is_empty());
    assert!(!Bounds::new(0, 0, 0, 0).is_empty());
    assert!(Bounds::new(0, -1, 1, 0).is_empty());
    assert!(Bounds::new(0, -1, -1, 0).is_empty());
    assert!(Bounds::new(0, 1, -1, 0).is_empty());
}

#[test]
fn test_bounds_contains() {
    let bounds = Bounds::new(-10, 10, 10, -10);
    assert!(bounds.contains(&Point::new(0, 0)));
    assert!(bounds.contains(&Point::new(10, 0)));
    assert!(!bounds.contains(&Point::new(11, 0)));
    assert!(bounds.contains(&Point::new(-10, 0)));
    assert!(!bounds.contains(&Point::new(-11, 0)));
    assert!(bounds.contains(&Point::new(0, 10)));
    assert!(!bounds.contains(&Point::new(0, 11)));
    assert!(bounds.contains(&Point::new(0, -10)));
    assert!(!bounds.contains(&Point::new(0, -11)));
}

#[test]
fn test_bounds_size() {
    let bounds = Bounds::new(-5, 10, 5, -10);
    assert_eq!(Size::new(21, 11), bounds.size());
}

#[test]
fn test_bounds_iter_points() {
    let bounds = Bounds::new(-1, 1, 2, -2);
    let mut i = bounds.iter_points();
    assert_eq!(Some(Point::new(-2, -1)), i.next());
    assert_eq!(Some(Point::new(-1, -1)), i.next());
    assert_eq!(Some(Point::new(0, -1)), i.next());
    assert_eq!(Some(Point::new(1, -1)), i.next());

    assert_eq!(Some(Point::new(-2, 0)), i.next());
    assert_eq!(Some(Point::new(-1, 0)), i.next());
    assert_eq!(Some(Point::new(0, 0)), i.next());
    assert_eq!(Some(Point::new(1, 0)), i.next());

    assert_eq!(Some(Point::new(-2, 1)), i.next());
    assert_eq!(Some(Point::new(-1, 1)), i.next());
    assert_eq!(Some(Point::new(0, 1)), i.next());
    assert_eq!(Some(Point::new(1, 1)), i.next());

    assert_eq!(Some(Point::new(-2, 2)), i.next());
    assert_eq!(Some(Point::new(-1, 2)), i.next());
    assert_eq!(Some(Point::new(0, 2)), i.next());
    assert_eq!(Some(Point::new(1, 2)), i.next());

    assert_eq!(None, i.next());
}

#[test]
fn test_bounds_iter_row_indices() {
    let bounds = Bounds::new(-1, 1, 2, -2);
    let mut i = bounds.iter_row_indices();
    assert_eq!(Some(0..4), i.next());
    assert_eq!(Some(4..8), i.next());
    assert_eq!(Some(8..12), i.next());
    assert_eq!(Some(12..16), i.next());

    assert_eq!(None, i.next());
}

#[test]
fn test_bounds_iter_indices() {
    let bounds = Bounds::new(-1, 1, 2, -2);
    let mut i = bounds.iter_indices();
    assert_eq!(Some(0), i.next());
    assert_eq!(Some(1), i.next());
    assert_eq!(Some(2), i.next());
    assert_eq!(Some(3), i.next());

    assert_eq!(Some(4), i.next());
    assert_eq!(Some(5), i.next());
    assert_eq!(Some(6), i.next());
    assert_eq!(Some(7), i.next());

    assert_eq!(Some(8), i.next());
    assert_eq!(Some(9), i.next());
    assert_eq!(Some(10), i.next());
    assert_eq!(Some(11), i.next());

    assert_eq!(Some(12), i.next());
    assert_eq!(Some(13), i.next());
    assert_eq!(Some(14), i.next());
    assert_eq!(Some(15), i.next());

    assert_eq!(None, i.next());
}

#[test]
fn test_accessors() {
    let bounds = Bounds::new(1, 2, 3, 4);

    assert_eq!(Point::new(2, 1), bounds.top_right());
    assert_eq!(Point::new(4, 1), bounds.top_left());
    assert_eq!(Point::new(2, 3), bounds.bottom_right());
    assert_eq!(Point::new(4, 3), bounds.bottom_left());
}
