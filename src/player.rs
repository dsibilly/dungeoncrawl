use crate::prelude::*;

pub struct Player {
    pub position: Point,
}

impl Player {
    pub fn new(position: Point) -> Self {
        Self { position }
    }

    pub fn render(&self, ctx: &mut BTerm, camera: &Camera) {
        ctx.set_active_console(1);
        let new_x = self.position.x - camera.viewport.x1;
        let new_y = self.position.y - camera.viewport.y1;
        
        // debug!("[PLAYER] Rendering at ({:?}, {:?})", new_x, new_y);
        ctx.set(
            new_x,
            new_y,
            WHITE,
            BLACK,
            to_cp437('@'),
        );
    }

    pub fn update(&mut self, ctx: &mut BTerm, map: &Map, camera: &mut Camera) {
        ctx.set_active_console(1);
        if let Some(key) = ctx.key {
            let delta = match key {
                VirtualKeyCode::Left => Point::new(-1, 0),
                VirtualKeyCode::Right => Point::new(1, 0),
                VirtualKeyCode::Up => Point::new(0, -1),
                VirtualKeyCode::Down => Point::new(0, 1),
                _ => Point::zero(),
            };

            let new_position = self.position + delta;
            if map.can_enter_tile(new_position) {
                debug!("[PLAYER] Moving from {:?} to {:?}", self.position, new_position);
                self.position = new_position;
                camera.on_player_move(new_position);
            }
        }
    }
}
