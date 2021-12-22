use std::fs::File;
use std::io::Write;

#[derive(Copy, Clone)]
pub struct Vec3f32
{
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3f32
{
    pub fn zeroes() -> Vec3f32
    {
        Vec3f32
        {
            x: 0f32,
            y: 0f32,
            z: 0f32,
        }
    }

    pub fn new_from_points(x: f32, y: f32, z: f32) -> Vec3f32
    {
        Vec3f32
        {
            x,
            y,
            z,
        }
    }

    pub fn dot_product(&self, product: &Vec3f32) -> f32
    {
        (self.x * product.x) + (self.y * product.y) + (self.z * product.z)
    }

    pub fn cross_product(&self, product: &Vec3f32) -> Vec3f32
    {
        Vec3f32
        {
            x: self.y * product.z - self.z *product.y,
            y: self.z * product.x - self.x * product.z,
            z: self.x * product.y - self.y * product.x,
        }
    }

    pub fn norm(&self) -> f32
    {
        ((self.x * self.x) + (self.y * self.y) + (self.z * self.z)).sqrt()
    }

    pub fn length(&self) -> f32
    {
        (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
    }

    pub fn normalize(&self) -> Vec3f32
    {
        let magnitude = self.dot_product(&self).sqrt();
        let mut new_x: f32 = self.x;
        let mut new_y: f32 = self.y;
        let mut new_z: f32 = self.z;
        if magnitude > 0.0
        {
            let inverse_mag = 1.0 / magnitude;
            new_x *= inverse_mag;
            new_y *= inverse_mag;
            new_z *= inverse_mag;
        }
        Vec3f32
        {
            x: new_x,
            y: new_y,
            z: new_z,
        }
    }

    pub fn unit_vector(&self) -> Vec3f32
    {
        *self / self.length()
    }

    pub fn write_vec_as_int(&self, file: &mut File)
    {
        write!(file, "{} {} {}\n", self.x as i32, self.y as i32, self.z as i32).unwrap();
    }
}

impl std::ops::Add<Vec3f32> for Vec3f32
{
    type Output = Self;

    fn add(self, other: Self) -> Self
    {
        Self
        {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl std::ops::Add<f32> for Vec3f32
{
    type Output = Self;

    fn add(self, other: f32) -> Self
    {
        Self
        {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
        }
    }
}

impl std::ops::Mul<Vec3f32> for Vec3f32
{
    type Output = Self;

    fn mul(self, other: Self) -> Self
    {
        Self
        {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl std::ops::Mul<f32> for Vec3f32
{
    type Output = Self;

    fn mul(self, other: f32) -> Self
    {
        Self
        {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl std::ops::Sub<Vec3f32> for Vec3f32
{
    type Output = Self;

    fn sub(self, other: Self) -> Self
    {
        Self
        {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl std::ops::Sub<f32> for Vec3f32
{
    type Output = Self;

    fn sub(self, other: f32) -> Self
    {
        Self
        {
            x: self.x - other,
            y: self.y - other,
            z: self.z - other,
        }
    }
}

impl std::ops::Div<f32> for Vec3f32
{
    type Output = Self;

    fn div (self, other: f32) -> Self
    {
        Self
        {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl std::ops::MulAssign<f32> for Vec3f32
{
    fn mul_assign(&mut self, other: f32)
    {
        self.x *= other;
        self.y *= other;
        self.z *= other;
    }
}

impl std::ops::MulAssign<Vec3f32> for Vec3f32
{
    fn mul_assign(&mut self, other: Vec3f32)
    {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
    }
}
