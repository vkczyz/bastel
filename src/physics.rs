#[derive(Clone, PartialEq)]
pub struct Physics {
    pub velocity: (f32, f32),
    pub acceleration: (f32, f32),
}

impl Physics {
    pub fn new() -> Self {
        Physics {
            velocity: (0.0, 0.0),
            acceleration: (0.0, 0.0),
        }
    }

    pub fn resultant(a: &Physics, b: &Physics) -> Self {
        Physics {
            velocity: a.velocity,
            acceleration: (
                a.acceleration.0 + b.acceleration.0,
                a.acceleration.1 + b.acceleration.1,
            ),
        }
    }

    pub fn get_position_delta(&self) -> (f32, f32) {
        let mass = 1.0;
        let velocity = (
            self.velocity.0 + self.acceleration.0 * mass,
            self.velocity.1 + self.acceleration.1 * mass,
        );

        let delta = (
            velocity.0 * 1.0,
            velocity.1 * 1.0,
        );

        delta
    }
}