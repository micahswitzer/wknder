use crate::vec3::Vec3;

#[derive(Copy, Clone, Default, Debug)]
pub struct Ray(pub Vec3, pub Vec3);

impl Ray {
    pub fn mut_from(&mut self, other: Self) {
        self.0 = other.0;
        self.1 = other.1;
    }

    #[inline]
    pub fn point_at_parameter(self, t: f32) -> Vec3 {
        self.0 + t * self.1
    }

    #[inline]
    pub fn origin(self) -> Vec3 {
        self.0
    }

    #[inline]
    pub fn direction(self) -> Vec3 {
        self.1
    }
}
