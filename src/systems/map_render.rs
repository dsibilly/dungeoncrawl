use crate::prelude::*;

#[system]
#[read_component(FieldOfView)]
#[read_component(Player)]
pub fn map_render(
    ecs: &SubWorld,
    #[resource] map: &Map, 
    #[resource] camera: &Camera
) {
    let mut fov = <&FieldOfView>::query().filter(component::<Player>());
    let player_fov = fov.iter(ecs).next().unwrap();

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(0);
    for y in camera.viewport.y1..=camera.viewport.y2 {
        for x in camera.viewport.x1..=camera.viewport.x2 {
            let pt = Point::new(x, y);
            let offset = Point::new(camera.viewport.x1, camera.viewport.y1);
            let index = map_index(x, y);

            if map.is_tile_in_bounds(pt) && player_fov.visible_tiles.contains(&pt) | map.revealed_tiles[index] {
                let tint = if player_fov.visible_tiles.contains(&pt) {
                    WHITE
                } else {
                    DARK_GREY
                };
                
                match map.tiles[index] {
                    TileType::Floor => {
                        draw_batch.set(
                            pt - offset,
                            ColorPair::new(
                                tint,
                                BLACK
                            ),
                            to_cp437('.')
                        );
                    }
                    TileType::Wall => {
                        draw_batch.set(
                            pt - offset,
                            ColorPair::new(tint, BLACK),
                            to_cp437('#')
                        );
                    }
                }
            }
        }
    }

    draw_batch.submit(0).expect("Batch error");
}
