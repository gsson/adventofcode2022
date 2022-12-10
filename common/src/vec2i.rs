use std::fmt::{Debug, Formatter};
use std::ops::{Add, Div, Mul, Neg, Sub};
use std::simd::Which::{First, Second};
use std::simd::{simd_swizzle, Simd, SimdInt, SimdOrd, SimdPartialOrd};
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
    pub fn x(&self) -> i32 {
        self.0[Point::X]
    }
    #[inline]
    pub fn y(&self) -> i32 {
        self.0[Point::Y]
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

impl From<(i32, i32)> for Point {
    fn from((x, y): (i32, i32)) -> Self {
        Point(Simd::from_array([x, y]))
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
    #[inline]
    const fn new(x: i32, y: i32) -> Self {
        Self(Simd::from_array([x, y]))
    }
    #[inline]
    pub fn x(&self) -> i32 {
        self.0[0]
    }
    #[inline]
    pub fn y(&self) -> i32 {
        self.0[1]
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

impl const From<(i32, i32)> for Vector {
    fn from((x, y): (i32, i32)) -> Self {
        Self(Simd::from_array([x, y]))
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

    pub const fn new(top: i32, right: i32, bottom: i32, left: i32) -> Self {
        Self(Point::new(left, top), Point::new(right, bottom))
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
    pub fn top_right(&self) -> Point {
        Point(simd_swizzle!(self.0 .0, self.1 .0, [Second(0), First(1)]))
    }

    #[inline]
    pub fn bottom_left(&self) -> Point {
        Point(simd_swizzle!(self.0 .0, self.1 .0, [First(0), Second(1)]))
    }

    #[inline]
    pub const fn bottom_right(&self) -> Point {
        self.1
    }

    #[inline]
    pub fn top(&self) -> i32 {
        self.0.y()
    }

    #[inline]
    pub fn right(&self) -> i32 {
        self.1.x()
    }

    #[inline]
    pub fn bottom(&self) -> i32 {
        self.1.y()
    }

    #[inline]
    pub fn left(&self) -> i32 {
        self.0.x()
    }

    #[inline]
    pub fn contains(&self, point: &Point) -> bool {
        point.0.simd_ge(self.0 .0).all() && point.0.simd_le(self.1 .0).all()
    }

    #[inline]
    pub fn index(&self, p: &Point) -> usize {
        debug_assert!(self.contains(p));
        ((p.0 - self.top_left().0) * Simd::from_array([1, self.size().width()])).reduce_sum()
            as usize
    }

    #[inline]
    pub fn size(&self) -> Size {
        Size(self.1 .0 - self.0 .0 + Self::SIZE_ADJUST)
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
    pub fn width(&self) -> i32 {
        self.0[Size::WIDTH]
    }

    #[inline]
    pub fn height(&self) -> i32 {
        self.0[Size::HEIGHT]
    }

    pub fn area(&self) -> i32 {
        self.0.reduce_product()
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
fn test_accessors() {
    let bounds = Bounds::new(1, 2, 3, 4);

    assert_eq!(Point::new(2, 1), bounds.top_right());
    assert_eq!(Point::new(4, 1), bounds.top_left());
    assert_eq!(Point::new(2, 3), bounds.bottom_right());
    assert_eq!(Point::new(4, 3), bounds.bottom_left());
}
