use crate::prelude::*;

#[system(for_each)]
#[read_component(Player)]
#[read_component(FieldOfView)]
pub fn movement(
    entity: &Entity,
    intent: &IntendsToMove,
    #[resource] map: &mut Map,
    #[resource] camera: &mut Camera,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
) {
    if map.can_enter_tile(intent.destination) {
        commands.add_component(intent.entity, intent.destination);

        if let Ok(entry) = ecs.entry_ref(intent.entity) {
            if let Ok(fov) = entry.get_component::<FieldOfView>() {
                commands.add_component(intent.entity, fov.clone_dirty());

                if entry.get_component::<Player>().is_ok() {
                    camera.on_player_move(intent.destination);
                    fov.visible_tiles.iter().for_each(|pos| {
                        map.revealed_tiles[map_index(pos.x, pos.y)] = true;
                    }); 
                }
            }
        }
    }

    commands.remove(*entity);
}
