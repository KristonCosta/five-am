use euclid::default::{
    Point2D as EuclidPoint2D, Rect as EuclidRect, Size2D as EuclidSize2D,
    Vector2D as EuclidVector2D,
};

pub type Rect = EuclidRect<i32>;
pub type Point = EuclidPoint2D<i32>;
pub type Vector = EuclidVector2D<i32>;
pub type Size = EuclidSize2D<i32>;

pub trait To<T>: Sized {
    fn to(self) -> T;
}
