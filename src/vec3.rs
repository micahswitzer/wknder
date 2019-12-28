use rand::Rng;

#[derive(Copy, Clone, Default, Debug)]
pub struct Vec3(pub f32, pub f32, pub f32);

impl Vec3 {
    pub fn random_in_unit_sphere() -> Self {
        let mut rng = rand::thread_rng();
        loop {
            let v: Vec3 = rng.gen();
            let p = 2.0 * v - Vec3::from(1.0);
            if p.dot(p) < 1.0 {
                return p;
            }
        }
    }

    pub fn random_in_unit_disk() -> Self {
        let mut rng = rand::thread_rng();
        loop {
            let p = 2.0 * Vec3(rng.gen(), rng.gen(), 0.0) - Vec3(1.0, 1.0, 0.0);
            if p.dot(p) < 1.0 {
                return p;
            }
        }
    }

    pub fn reflect_about(self, n: Vec3) -> Self {
        self - 2.0 * self.dot(n) * n
    }

    pub fn mut_from(&mut self, other: Vec3) {
        self.0 = other.0;
        self.1 = other.1;
        self.2 = other.2;
    }

    #[inline]
    pub fn dot(&self, other: Self) -> f32 {
        self.zip_with(other, core::ops::Mul::mul)
            .reduce(core::ops::Add::add)
    }

    #[inline]
    pub fn cross(&self, other: &Self) -> Self {
        Vec3(
            self.1 * other.2 - self.2 * other.1,
            -(self.0 * other.2 - self.2 * other.0),
            self.0 * other.1 - self.1 * other.0,
        )
    }

    #[inline]
    pub fn length(&self) -> f32 {
        self.dot(*self).sqrt()
    }

    #[inline]
    pub fn into_unit(self) -> Self {
        self / self.length()
    }

    #[inline]
    pub fn map(self, mut f: impl FnMut(f32) -> f32) -> Self {
        Vec3(f(self.0), f(self.1), f(self.2))
    }

    #[inline]
    pub fn each(self, mut f: impl FnMut(f32) -> ()) {
        f(self.0);
        f(self.1);
        f(self.2);
    }

    #[inline]
    pub fn zip_with(
        self,
        other: Vec3,
        mut f: impl FnMut(f32, f32) -> f32,
    ) -> Self {
        Vec3(
            f(self.0, other.0),
            f(self.1, other.1),
            f(self.2, other.2),
        )
    }

    #[inline]
    pub fn zip_with3(
        self,
        other1: Vec3,
        other2: Vec3,
        mut f: impl FnMut(f32, f32, f32) -> f32,
    ) -> Self {
        Vec3(
            f(self.0, other1.0, other2.0),
            f(self.1, other1.1, other2.1),
            f(self.2, other1.2, other2.2),
        )
    }

    #[inline]
    pub fn reduce(self, f: impl Fn(f32, f32) -> f32) -> f32 {
        f(f(self.0, self.1), self.2)
    }
}

impl From<f32> for Vec3 {
    #[inline]
    fn from(v: f32) -> Self {
        Vec3(v, v, v)
    }
}

impl rand::distributions::Distribution<Vec3> for rand::distributions::Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vec3 {
        Vec3(rng.gen(), rng.gen(), rng.gen())
    }
}

impl std::ops::Mul for Vec3 {
    type Output = Vec3;

    #[inline]
    fn mul(self, rhs: Vec3) -> Self::Output {
        self.zip_with(rhs, std::ops::Mul::mul)
    }
}

impl std::ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    #[inline]
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::from(self) * rhs
    }
}

impl std::ops::Div for Vec3 {
    type Output = Vec3;

    #[inline]
    fn div(self, rhs: Vec3) -> Self::Output {
        self.zip_with(rhs, std::ops::Div::div)
    }
}

impl std::ops::Div<f32> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        self.map(|x| x / rhs)
    }
}

impl std::ops::Add for Vec3 {
    type Output = Vec3;

    #[inline]
    fn add(self, rhs: Vec3) -> Self::Output {
        self.zip_with(rhs, std::ops::Add::add)
    }
}

impl std::ops::Add<Vec3> for f32 {
    type Output = Vec3;

    #[inline]
    fn add(self, rhs: Vec3) -> Self::Output {
        rhs.map(|x| self + x)
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Vec3;

    #[inline]
    fn sub(self, rhs: Vec3) -> Self::Output {
        self.zip_with(rhs, std::ops::Sub::sub)
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Vec3;

    #[inline]
    fn neg(self) -> Self::Output {
        self.map(std::ops::Neg::neg)
    }
}

impl std::ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl std::ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        self.0 /= rhs;
        self.1 /= rhs;
        self.2 /= rhs;
    }
}

impl std::iter::Sum for Vec3 {
    #[inline]
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Vec3::default(), std::ops::Add::add)
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Channel {
    R,
    G,
    B,
}

use Channel::*;

impl ::std::ops::Index<Channel> for Vec3 {
    type Output = f32;

    #[inline]
    fn index(&self, idx: Channel) -> &Self::Output {
        match idx {
            R => &self.0,
            G => &self.1,
            B => &self.2,
        }
    }
}

impl ::std::ops::IndexMut<Channel> for Vec3 {
    #[inline]
    fn index_mut(&mut self, idx: Channel) -> &mut Self::Output {
        match idx {
            R => &mut self.0,
            G => &mut self.1,
            B => &mut self.2,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Axis {
    X,
    Y,
    Z,
}

use Axis::*;

impl ::std::ops::Index<Axis> for Vec3 {
    type Output = f32;

    #[inline]
    fn index(&self, idx: Axis) -> &Self::Output {
        match idx {
            X => &self.0,
            Y => &self.1,
            Z => &self.2,
        }
    }
}

impl ::std::ops::IndexMut<Axis> for Vec3 {
    #[inline]
    fn index_mut(&mut self, idx: Axis) -> &mut Self::Output {
        match idx {
            X => &mut self.0,
            Y => &mut self.1,
            Z => &mut self.2,
        }
    }
}
