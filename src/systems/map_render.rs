use crate::prelude::*;

#[system]
pub fn map_render(#[resource] map: &Map, #[resource] camera: &Camera) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(0);
    for y in camera.viewport.y1..=camera.viewport.y2 {
        for x in camera.viewport.x1..=camera.viewport.x2 {
            let pt = Point::new(x, y);
            let offset = Point::new(camera.viewport.x1, camera.viewport.y1);

            if map.in_bounds(pt) {
                let index = map_index(x, y);
                let glyph = match map.tiles[index] {
                    TileType::Floor => to_cp437('.'),
                    TileType::Wall => to_cp437('#'),
                };

                draw_batch.set(pt - offset, ColorPair::new(WHITE, BLACK), glyph);
            }
        }
    }

    draw_batch.submit(0).expect("Batch error");
}
