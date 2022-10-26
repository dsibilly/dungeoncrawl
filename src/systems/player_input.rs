use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(Player)]
pub fn player_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState,
) {
    if let Some(key) = *key {
        let delta = match key {
            VirtualKeyCode::Left => {
                debug!("Key Press: LEFT");
                Point::new(-1, 0)
            }
            VirtualKeyCode::Right => {
                debug!("Key Press: RIGHT");
                Point::new(1, 0)
            }
            VirtualKeyCode::Up => {
                debug!("Key Press: UP");
                Point::new(0, -1)
            }
            VirtualKeyCode::Down => {
                debug!("Key Press: DOWN");
                Point::new(0, 1)
            }
            _ => {
                debug!("Key Press: SKIP");
                Point::new(0, 0)
            }
        };

        let mut players = <(Entity, &Point)>::query().filter(component::<Player>());

        players.iter_mut(ecs).for_each(|(entity, pos)| {
            let destination = *pos + delta;

            commands.push((
                (),
                IntendsToMove {
                    entity: *entity,
                    destination,
                },
            ));
        });

        *turn_state = TurnState::PlayerTurn;
    }
}
