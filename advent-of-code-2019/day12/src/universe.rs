use num::integer::lcm;

pub struct Universe {
    // Since the way the gravity is applied is dimension specific (dimensions
    // don't influence each other), we destructe moons into their dimensions
    // and work on the destructed dimensions only.
    dimensions: [OneDimensionalUniverse; 3],
}

impl Universe {
    /// Assumes moons is an array with 4 elements.
    pub fn new(moons: &[(i32, i32, i32)]) -> Self {
        let dimension_x = OneDimensionalUniverse::new([moons[0].0, moons[1].0, moons[2].0, moons[3].0]);
        let dimension_y = OneDimensionalUniverse::new([moons[0].1, moons[1].1, moons[2].1, moons[3].1]);
        let dimension_z = OneDimensionalUniverse::new([moons[0].2, moons[1].2, moons[2].2, moons[3].2]);

        Self {
            dimensions: [dimension_x, dimension_y, dimension_z],
        }
    }

    pub fn step(&mut self, delta: u32) {
        self.dimensions[0].step(delta);
        self.dimensions[1].step(delta);
        self.dimensions[2].step(delta);
    }

    pub fn total_energy(&self) -> i32 {
        let mut total_energy = 0;

        for i in 0..4 {
            let kinetic_energy = self.dimensions[0].bodies[i].abs()
                + self.dimensions[1].bodies[i].abs()
                + self.dimensions[2].bodies[i].abs();
            let potential_energy = self.dimensions[0].velocities[i].abs()
                + self.dimensions[1].velocities[i].abs()
                + self.dimensions[2].velocities[i].abs();

            total_energy += kinetic_energy * potential_energy;
        }

        total_energy
    }

    pub fn find_cycle_time(&mut self) -> u64 {
        let steps_x = self.dimensions[0].find_cycle_time();
        let steps_y = self.dimensions[1].find_cycle_time();
        let steps_z = self.dimensions[2].find_cycle_time();

        lcm(steps_x, lcm(steps_y, steps_z))
    }
}

#[derive(Clone, Eq, PartialEq)]
struct OneDimensionalUniverse {
    bodies: [i32; 4],
    velocities: [i32; 4],
}

impl OneDimensionalUniverse {
    fn new(bodies: [i32; 4]) -> Self {
        Self {
            bodies,
            velocities: [0; 4],
        }
    }

    fn step(&mut self, delta: u32) {
        for _ in 0..delta {
            for i in 0..4 {
                for j in 0..4 {
                    if i == j {
                        continue;
                    }

                    self.velocities[i] -= compare(self.bodies[i], self.bodies[j]);
                }
            }

            for i in 0..4 {
                self.bodies[i] += self.velocities[i];
            }
        }
    }

    /// Find the amount of steps needed to return to the same state.
    fn find_cycle_time(&mut self) -> u64 {
        let mut steps = 0;
        let original_state = self.clone();

        loop {
            self.step(1);
            steps += 1;

            if *self == original_state {
                return steps;
            }
        }
    }
}

/// Compares two integers, returns -1 if v1 < v2, returns 1 if v1 > v2 or
/// returns 0 if v1 == v2.
fn compare(v1: i32, v2: i32) -> i32 {
    match v1 {
        _ if v1 < v2 => -1,
        _ if v1 > v2 => 1,
        _ => 0,
    }
}
