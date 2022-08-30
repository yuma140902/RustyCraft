use std::ops::{Deref, DerefMut};

#[derive(PartialEq, PartialOrd, Clone, Copy, Debug)]
pub struct Deg(pub f32);

#[derive(PartialEq, PartialOrd, Clone, Copy, Debug)]
pub struct Rad(pub f32);

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
