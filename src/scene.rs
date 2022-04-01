use crate::entity::Entity;
use crate::physics::Physics;
use crate::shaders::Shader;
use crate::sprite::Sprite;

pub struct Scene {
    pub physics: Physics,
    pub entities: Vec<Entity>,
    pub player_index: usize,
}

impl Scene {
    pub fn new(entities: Vec<Entity>, player_index: usize) -> Self {
        let mut physics = Physics::new();
        physics.acceleration.1 = 0.001;

        let entities = vec![
            // Background
            Entity::new(
                Sprite::with_color(
                    (-1.0, -1.0),
                    (2.0, 2.0),
                    [0.1, 0.1, 0.1],
                ),
                false,
            ),
            // Player
            Entity::new(
                Sprite::rainbow(
                    (-0.5, -0.5),
                    (0.1, 0.1),
                ),
                true,
            ),
            // Borders
            Entity::new(
                Sprite::invisible(
                    (-1.0, -1.0),
                    (0.0, 2.0),
                ),
                true,
            ),
            Entity::new(
                Sprite::invisible(
                    (-1.0, -1.0),
                    (2.0, 0.0),
                ),
                true,
            ),
            Entity::new(
                Sprite::invisible(
                    (-1.0, 1.0),
                    (2.0, 0.0),
                ),
                true,
            ),
            Entity::new(
                Sprite::invisible(
                    (1.0, -1.0),
                    (0.0, 2.0),
                ),
                true,
            ),
        ];

        Scene {
            entities,
            player_index,
            physics,
        }
    }
}