pub trait Vector: Sized {
    fn perp(&self) -> Self;
    fn dot_product(&self, other: &Self) -> f32;

    fn perp_dot_product(&self, other: &Self) -> f32 {
        self.perp().dot_product(other)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    #[inline(always)]
    pub fn new(x: f32, y: f32) -> Vec2 {
        Vec2 { x, y }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct IVec2 {
    pub x: i32,
    pub y: i32,
}

impl IVec2 {
    #[inline(always)]
    pub fn new(x: i32, y: i32) -> IVec2 {
        IVec2 { x, y }
    }
}

impl Vector for IVec2 {
    fn perp(&self) -> Self {
        IVec2 { x:-self.x, y:self.y }
    }

    fn dot_product(&self, other: &Self) -> f32 {
        (self.x * other.x + self.y * other.y) as f32
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
