pub struct Physics {
    pub force: (f32, f32),
}

impl Physics {
    pub fn new() -> Self {
        Physics {
            force: (0.0, 0.0),
        }
    }

    pub fn from_force(x: f32, y: f32) -> Self {
        Physics {
            force: (x, y),
        }
    }

    pub fn resultant(a: &Physics, b: &Physics) -> Self {
        Physics {
            force: (
                a.force.0 + b.force.0,
                a.force.1 + b.force.1,
            ),
        }
    }
}