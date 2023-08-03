use crate::prelude::*;

#[system]
#[read_component(Point)]
#[write_component(FieldOfView)]
pub fn fov(
    ecs: &mut SubWorld,
    #[resource] map: &Map,
) {
    let mut views = <(&Point, &mut FieldOfView)>::query();

    views
        .iter_mut(ecs)
        .filter(|(_,fov)| fov.render_state == TileFoVState::Dirty)
        .for_each(|(pos, fov)| {
            fov.visible_tiles = field_of_view_set(*pos, fov.radius, map);
            fov.render_state = TileFoVState::Clean;
        });
}