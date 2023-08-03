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
    pub revealed_tiles: Vec<bool>
}

impl Map {
    pub fn can_enter_tile(&self, point: Point) -> bool {
        self.in_bounds(point) && self.tiles[map_index(point.x, point.y)] == TileType::Floor
    }

    pub fn is_tile_in_bounds(&self, point: Point) -> bool {
        point.x >= 0 && point.x < SCREEN_WIDTH && point.y >= 0 && point.y < SCREEN_HEIGHT
    }

    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
            revealed_tiles: vec![false; NUM_TILES],
        }
    }

    pub fn try_index(&self, point: Point) -> Option<usize> {
        if !self.is_tile_in_bounds(point) {
            None
        } else {
            Some(map_index(point.x, point.y))
        }
    }

    pub fn valid_move(&self, loc: Point, delta: Point) -> Option<usize> {
        let destination = loc + delta;
        if self.is_tile_in_bounds(destination) {
            if self.can_enter_tile(destination) {
                let index = self.point2d_to_index(destination);
                Some(index)
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(SCREEN_WIDTH, SCREEN_HEIGHT)
    }

    fn in_bounds(&self, pos: Point) -> bool {
        self.is_tile_in_bounds(pos)
    }
}

impl BaseMap for Map {
    fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let mut destinations = SmallVec::new();
        let location = self.index_to_point2d(idx);

        if let Some(index) = self.valid_move(location, Point::new(-1, 0)) {
            destinations.push((index, 1.0))
        }
        if let Some(index) = self.valid_move(location, Point::new(1,0)) {
            destinations.push((index, 1.0))
        }
        if let Some(index) = self.valid_move(location, Point::new(0, -1)) {
            destinations.push((index, 1.0))
        }
        if let Some(index) = self.valid_move(location, Point::new(0, 1)) {
            destinations.push((index, 1.0))
        }

        destinations
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        DistanceAlg::Pythagoras
            .distance2d(self.index_to_point2d(idx1), self.index_to_point2d(idx2))
    }

    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx] != TileType::Floor
    }
}