#[derive(Clone, PartialEq)]
pub struct Physics {
    pub mass: f32,
    pub acceleration: (f32, f32),
    velocity: (f32, f32),
}

impl Physics {
    pub fn new(mass: f32) -> Self {
        Physics {
            mass,
            acceleration: (0.0, 0.0),
            velocity: (0.0, 0.0),
        }
    }

    pub fn update(&mut self) {
        self.velocity.0 += self.acceleration.0;
        self.velocity.1 += self.acceleration.1;
    }

    pub fn invert_x(&mut self) {
        self.acceleration.0 *= -1.0;
        self.velocity.0 *= -1.0;
    }

    pub fn invert_y(&mut self) {
        self.acceleration.1 *= -1.0;
        self.velocity.1 *= -1.0;
    }

    pub fn apply_force(&mut self, force: (f32, f32)) {
        self.acceleration.0 = force.0 / self.mass;
        self.acceleration.1 = force.1 / self.mass;
    }

    pub fn get_displacement(&self) -> (f32, f32) {
        let displacement = (self.velocity.0, self.velocity.1);
        displacement
    }
}