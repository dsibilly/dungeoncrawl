use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
#[write_component(Health)]
pub fn player_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState,
) {
    let mut players = <(Entity, &Point)>::query().filter(component::<Player>());

    if let Some(key) = *key {
        let delta = match key {
            VirtualKeyCode::Left | VirtualKeyCode::A => {
                debug!("Key Press: LEFT");
                Point::new(-1, 0)
            }
            VirtualKeyCode::Right | VirtualKeyCode::D => {
                debug!("Key Press: RIGHT");
                Point::new(1, 0)
            }
            VirtualKeyCode::Up | VirtualKeyCode::W => {
                debug!("Key Press: UP");
                Point::new(0, -1)
            }
            VirtualKeyCode::Down | VirtualKeyCode::S => {
                debug!("Key Press: DOWN");
                Point::new(0, 1)
            }
            VirtualKeyCode::Space => {
                debug!("Key Press: SKIP");
                Point::zero()
            },
            _ => {
                Point::zero()
            }
        };

        let (player_entity, destination) = players
            .iter(ecs)
            .map(|(entity, pos)| (*entity, *pos + delta))
            .next()
            .unwrap();

        let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());

        let mut action = PlayerActions::Heal;
        if delta.x != 0 || delta.y != 0 {
            let mut hit_something = false;
            enemies
                .iter(ecs)
                .filter(|(_, pos)| **pos == destination)
                .for_each(|(entity, _)| {
                    hit_something = true;
                    action = PlayerActions::Attack;
                    debug!("Queueing an attack");
                    commands.push((
                        (),
                        IntendsToAttack {
                            attacker: player_entity,
                            defender: *entity,
                        },
                    ));
                });

            if !hit_something {
                action = PlayerActions::Move;
                debug!("Queueing a move to {:?}", destination);
                commands.push((
                    (),
                    IntendsToMove {
                        entity: player_entity,
                        destination,
                    },
                ));
            }
        }

        if let PlayerActions::Heal = action {
            debug!("Player did nothing...");
            if let Ok(health) = ecs
                .entry_mut(player_entity)
                .unwrap()
                .get_component_mut::<Health>()
            {
                let new_health = i32::min(health.max, health.current + 1);
                
                if new_health > health.current {
                    debug!("Healing up!");
                    debug!("Health before healing: {}", health.current);
                    health.current = new_health;
                    debug!("Health after healing: {}", health.current);
                }
            }
        }

        *turn_state = TurnState::PlayerTurn;
    }
}
