pub struct CollisionSystem { }

impl CollisionSystem {
        /*
        // Collision check
        let player = &self.scene.entities[entity_index];
        let mut collision = None;

        for entity in self.scene.entities.iter() {
            if !entity.collideable {
                continue;
            }
            if entity == player {
                continue;
            }

            if Entity::are_colliding(player, entity) {
                collision = Some((
                    entity.clone(),
                    Entity::get_collision_intersection(player, entity),
                ));
            }
        }

        // Collision handling
        let player = &mut self.scene.entities[entity_index];

        if let Some((e, d)) = collision {
            let x_dist = d[1] - d[0];
            let y_dist = d[3] - d[2];

            let collision_axis = if x_dist < y_dist { Axis::X } else { Axis::Y };
            let edge = match collision_axis {
                Axis::X => {
                    player.physics.bounce_x();
                    player.physics.friction_y();
                    if e.sprite.position.0 == d[0] { Edge::Left } else { Edge::Right }
                },
                Axis::Y => {
                    player.physics.bounce_y();
                    player.physics.friction_x();
                    if e.sprite.position.1 == d[2] { Edge::Top } else { Edge::Bottom }
                },
            };

            match edge {
                Edge::Left => {
                    player.sprite.position.0 -= x_dist;
                },
                Edge::Right => {
                    player.sprite.position.0 += x_dist;
                },
                Edge::Top => {
                    player.sprite.position.1 -= y_dist;
                    if player.physics.velocity.1.abs() < global.1.abs() {
                        player.airtime = 0;
                    }
                },
                Edge::Bottom => {
                    player.sprite.position.1 += y_dist;
                },
            }
        }
        */
}