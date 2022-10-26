use crate::prelude::*;

#[system(for_each)]
#[read_component(Player)]
pub fn movement(
    entity: &Entity,
    intent: &IntendsToMove,
    #[resource] map: &Map,
    #[resource] camera: &mut Camera,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
) {
    if map.can_enter_tile(intent.destination) {
        commands.add_component(intent.entity, intent.destination);

        if ecs
            .entry_ref(intent.entity)
            .unwrap()
            .get_component::<Player>()
            .is_ok()
        {
            camera.on_player_move(intent.destination);
        }
    }

    commands.remove(*entity);
}
