use crate::prelude::*;

const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum TileType {
    Wall,
    Floor,
}

pub fn map_index(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
}

pub struct Map {
    pub tiles: Vec<TileType>,
}

impl Map {
    pub fn can_enter_tile(&self, point: Point) -> bool {
        self.in_bounds(point) && self.tiles[map_index(point.x, point.y)] == TileType::Floor
    }

    pub fn in_bounds(&self, point: Point) -> bool {
        point.x >= 0 && point.x < SCREEN_WIDTH && point.y >= 0 && point.y < SCREEN_HEIGHT
    }

    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
        }
    }

    pub fn render(&self, ctx: &mut BTerm, camera: &Camera) {
        ctx.set_active_console(0);

        for y in camera.viewport.y1 .. camera.viewport.y2 {
            for x in camera.viewport.x1 .. camera.viewport.x2 {
                if self.in_bounds(Point::new(x, y)) {
                    let index = map_index(x, y);
                    
                    match self.tiles[index] {
                        TileType::Floor => {
                            ctx.set(x - camera.viewport.x1,
                                y - camera.viewport.y1, 
                                WHITE, 
                                BLACK, 
                                to_cp437('.')
                            );
                        }
                        TileType::Wall => {
                            ctx.set(
                                x - camera.viewport.x1, 
                                y - camera.viewport.y1,
                                WHITE, 
                                BLACK, 
                                to_cp437('#')
                            );
                        }
                    }
                }
                
                
            }
        }
    }

    pub fn try_index(&self, point: Point) -> Option<usize> {
        if !self.in_bounds(point) {
            None
        } else {
            Some(map_index(point.x, point.y))
        }
    }
}
