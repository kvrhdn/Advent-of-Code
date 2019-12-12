use crate::vec3::Vec3;

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct Universe {
    moons: Vec<Moon>,
}

impl Universe {
    pub fn new(moons: Vec<Moon>) -> Self {
        Self { moons }
    }

    pub fn step(&mut self, delta: u32) {
        for _ in 0..delta {
            for i in 0..self.moons.len() {
                for j in 0..self.moons.len() {
                    if i == j {
                        continue;
                    }

                    let other_pos = self.moons.get(j).unwrap().pos;
                    self.moons.get_mut(i).unwrap().gravitate_towards(other_pos);
                }
            }
            self.moons.iter_mut().for_each(|moon| moon.step(1));
        }
    }

    pub fn total_energy(&self) -> u32 {
        self.moons.iter().map(|moon| moon.total_energy()).sum()
    }
}

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct Moon {
    pos: Vec3,
    vel: Vec3,
}

impl Moon {
    pub fn new(pos: Vec3) -> Self {
        Self {
            pos,
            vel: Vec3 { x: 0, y: 0, z: 0 },
        }
    }

    fn step(&mut self, delta: i32) {
        self.pos.add(Vec3::scaled(self.vel, delta));
    }

    fn gravitate_towards(&mut self, pos: Vec3) {
        self.vel.x += -comparison(self.pos.x, pos.x);
        self.vel.y += -comparison(self.pos.y, pos.y);
        self.vel.z += -comparison(self.pos.z, pos.z);
    }

    fn total_energy(&self) -> u32 {
        let potential_energy = self.pos.sum_of_abs_coords();
        let kinetic_energy = self.vel.sum_of_abs_coords();

        potential_energy * kinetic_energy
    }
}

/// Compares two integers, returns -1 if v1 < v2, returns 1 if v1 > v2 or
/// returns 0 if v1 == v2.
fn comparison(v1: i32, v2: i32) -> i32 {
    match v1 {
        _ if v1 < v2 => -1,
        _ if v1 > v2 => 1,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_energy() {
        let moon = Moon {
            pos: Vec3 { x: -2, y: 1, z: 3 },
            vel: Vec3 { x: 3, y: 2, z: -1 },
        };
        assert_eq!(moon.total_energy(), 36);
    }
}
