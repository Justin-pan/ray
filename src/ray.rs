use crate::vec3f;

#[derive(Copy, Clone)]
pub struct Ray
{
    pub a: vec3f::Vec3f32,
    pub b: vec3f::Vec3f32
}

impl Ray
{
    pub fn zeroes() -> Ray
    {
        Ray
        {
            a: vec3f::Vec3f32::zeroes(),
            b: vec3f::Vec3f32::zeroes(),
        }
    }

    pub fn new_from_vector(a: &vec3f::Vec3f32, b: &vec3f::Vec3f32) -> Ray
    {
        Ray
        {
            a: *a,
            b: *b,
        }
    }

    pub fn origin(&mut self) -> vec3f::Vec3f32
    {
        self.a
    }

    pub fn direction(&mut self) -> vec3f::Vec3f32
    {
        self.b
    }

    pub fn point_at_parameter(&self, t: &f32) -> vec3f::Vec3f32
    {
        self.a + (self.b * *t)
    }
}
