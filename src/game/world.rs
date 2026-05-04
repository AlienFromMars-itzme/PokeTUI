#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Walkable,
    Blocked,
    Grass,
    Water,
}

pub struct WorldMap {
    pub width: i32,
    pub height: i32,
    pub tiles: Vec<Tile>,
}

impl WorldMap {
    pub fn starter() -> Self {
        let width = 20;
        let height = 10;
        let mut tiles = vec![Tile::Walkable; (width * height) as usize];
        for x in 5..15 {
            tiles[(3 * width + x) as usize] = Tile::Grass;
        }
        tiles[(1 * width + 1) as usize] = Tile::Blocked;
        Self {
            width,
            height,
            tiles,
        }
    }

    pub fn tile_at(&self, x: i32, y: i32) -> Option<Tile> {
        if x < 0 || y < 0 || x >= self.width || y >= self.height {
            return None;
        }
        self.tiles.get((y * self.width + x) as usize).copied()
    }
}
