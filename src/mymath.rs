use std::ops::{Deref, DerefMut};

use nalgebra::Point3;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct BlockPosInChunk(nalgebra::Point3<u32>);

#[derive(PartialEq, PartialOrd, Clone, Copy, Debug)]
pub struct Deg(pub f32);

#[derive(PartialEq, PartialOrd, Clone, Copy, Debug)]
pub struct Rad(pub f32);

impl BlockPosInChunk {
    pub fn new(x: u32, y: u32, z: u32) -> Option<Self> {
        if x < 16 && y < 16 && z < 16 {
            Some(Self {
                0: Point3::<u32>::new(x, y, z),
            })
        } else {
            None
        }
    }

    pub fn index(&self) -> usize {
        (16 * 16 * self.y + 16 * self.z + self.x) as usize
    }
}
impl Deref for BlockPosInChunk {
    type Target = Point3<u32>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deg {
    pub fn rad(&self) -> Rad {
        Rad(self.rad_f32())
    }

    #[inline]
    fn rad_f32(&self) -> f32 {
        self.0 * std::f32::consts::PI / 180f32
    }

    pub fn sin(&self) -> f32 {
        self.rad_f32().sin()
    }

    pub fn cos(&self) -> f32 {
        self.rad_f32().cos()
    }
}
impl Deref for Deg {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Deg {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for Rad {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
