use crate::components::Component;

#[derive(PartialEq)]
pub struct PhysicsComponent {
    pub mass: f32,
    pub acceleration: (f32, f32),
    pub velocity: (f32, f32),
    pub friction: f32,
    pub bounciness: f32,
    pub airtime: u32,
}

impl PhysicsComponent {
    pub fn new(mass: f32, friction: f32, bounciness: f32) -> Self {
        PhysicsComponent {
            mass,
            acceleration: (0.0, 0.0),
            velocity: (0.0, 0.0),
            friction,
            bounciness,
            airtime: 0,
        }
    }

    pub fn default() -> Self {
        PhysicsComponent {
            mass: 1.0,
            acceleration: (0.0, 0.0),
            velocity: (0.0, 0.0),
            friction: 0.5,
            bounciness: 1.0,
            airtime: 0,
        }
    }

    pub fn update(&mut self) {
        self.velocity.0 += self.acceleration.0;
        self.velocity.1 += self.acceleration.1;
    }

    pub fn bounce_x(&mut self) {
        self.acceleration.0 *= -self.bounciness;
        self.velocity.0 *= -self.bounciness;
    }

    pub fn bounce_y(&mut self) {
        self.acceleration.1 *= -self.bounciness;
        self.velocity.1 *= -self.bounciness;
    }

    pub fn friction_x(&mut self) {
        let f = self.mass * self.velocity.0 * self.friction;
        self.apply_force((-f, 0.0));
    }

    pub fn friction_y(&mut self) {
        let f = self.mass * self.velocity.1 * self.friction;
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

    pub fn from_xml(data: roxmltree::Node) -> Component {
        let mut mass = 0.0;
        let mut friction = 0.0;
        let mut bounciness = 0.0;

        data.attributes()
            .map(|a| {
                match a.name() {
                    "mass" => match a.value().parse::<f32>() {
                        Ok(d) => mass = d,
                        Err(_) => (),
                    },
                    "friction" => match a.value().parse::<f32>() {
                        Ok(d) => friction = d,
                        Err(_) => (),
                    },
                    "bounciness" => match a.value().parse::<f32>() {
                        Ok(d) => bounciness = d,
                        Err(_) => (),
                    },
                    _ => (),
                }
            }
        ).for_each(drop);

        Component::Physics(
            PhysicsComponent {
                mass,
                friction,
                bounciness,
                acceleration: (0.0, 0.0),
                velocity: (0.0, 0.0),
                airtime: 0,
            }
        )
    }

}