#[derive(Clone, PartialEq)]
pub struct Physics {
    pub mass: f32,
    pub acceleration: (f32, f32),
    velocity: (f32, f32),
    friction: f32,
}

impl Physics {
    pub fn new(mass: f32) -> Self {
        Physics {
            mass,
            acceleration: (0.0, 0.0),
            velocity: (0.0, 0.0),
            friction: 0.2,
        }
    }

    pub fn update(&mut self) {
        self.velocity.0 += self.acceleration.0;
        self.velocity.1 += self.acceleration.1;
    }

    pub fn invert_x(&mut self) {
        let f = 1.0 - self.friction;
        self.acceleration.0 *= -f;
        self.velocity.0 *= -f;
    }

    pub fn invert_y(&mut self) {
        let f = 1.0 - self.friction;
        self.acceleration.1 *= -f;
        self.velocity.1 *= -f;
    }

    pub fn friction_x(&mut self) {
        let f = self.velocity.0 * self.friction;
        self.apply_force((-f, 0.0));
    }

    pub fn friction_y(&mut self) {
        let f = self.velocity.1 * self.friction;
        self.apply_force((0.0, -f));
    }

    pub fn apply_force(&mut self, force: (f32, f32)) {
        self.acceleration.0 += force.0 / self.mass;
        self.acceleration.1 += force.1 / self.mass;
    }

    pub fn get_displacement(&self) -> (f32, f32) {
        let displacement = (self.velocity.0, self.velocity.1);
        displacement
    }

    pub fn reset(&mut self) {
        self.acceleration = (0.0, 0.0);
    }
}