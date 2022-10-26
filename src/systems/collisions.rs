use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
pub fn collisions(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut player_position = Point::zero();
    let mut players = <&Point>::query().filter(component::<Player>());
    let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());

    players.iter(ecs).for_each(|pos| player_position = *pos);
    enemies
        .iter(ecs)
        .filter(|(_, pos)| **pos == player_position)
        .for_each(|(entity, _)| {
            debug!("Removing entity: {:?}", entity);
            commands.remove(*entity);
        });
}
