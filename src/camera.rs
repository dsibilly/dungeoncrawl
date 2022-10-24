use crate::prelude::*;

pub struct Camera {
	pub viewport: Rect
}

impl Camera {
	pub fn new(player_position: Point) -> Self {
		Self {
			viewport: Rect::with_exact(
				player_position.x - DISPLAY_WIDTH / 2, 
				player_position.y - DISPLAY_HEIGHT / 2, 
				player_position.x + DISPLAY_WIDTH / 2,
				player_position.y + DISPLAY_HEIGHT / 2)
		}
	}
	
	pub fn on_player_move(&mut self, player_position: Point) {
		self.viewport.x1 = player_position.x - DISPLAY_WIDTH / 2;
		self.viewport.y1 = player_position.y - DISPLAY_HEIGHT / 2;
		self.viewport.x2 = player_position.x + DISPLAY_WIDTH / 2;
		self.viewport.y2 = player_position.y + DISPLAY_HEIGHT / 2;
	}
}