pub struct Physics {
    pub position: (f32, f32),
    pub velocity: (f32, f32),
    pub acceleration: (f32, f32),
}

impl Physics {
    pub fn new() -> Self {
        Physics {
            position: (0.0, 0.0),
            velocity: (0.0, 0.0),
            acceleration: (0.0, 0.0),
        }
    }

    pub fn resultant(a: &Physics, b: &Physics) -> Self {
        Physics {
            position: a.acceleration,
            velocity: a.velocity,
            acceleration: (
                a.acceleration.0 + b.acceleration.0,
                a.acceleration.1 + b.acceleration.1,
            ),
        }
    }

    pub fn update_position(&mut self) {
        self.velocity.0 += self.acceleration.0 * 1.0;
        self.position.0 += self.velocity.0 * 1.0;
    }
}