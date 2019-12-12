#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct Vec3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Vec3 {
    pub fn add(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }

    pub fn scaled(v: Vec3, value: i32) -> Self {
        Vec3 {
            x: v.x * value,
            y: v.y * value,
            z: v.z * value,
        }
    }

    pub fn sum_of_abs_coords(&self) -> u32 {
        (self.x.abs() + self.y.abs() + self.z.abs()) as u32
    }
}
