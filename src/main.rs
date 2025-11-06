use bracket_lib::prelude::{Algorithm2D, *};

#[derive(Clone, Copy, PartialEq)]
enum TileType {
    Wall,
    Floor,
}

#[derive(PartialEq, Copy, Clone)]

struct Tile {
    tile_type: TileType,
    revealed: bool,
}

struct Map {
    tiles: Vec<Tile>,
}

impl Map {
    fn xy_to_index(x: i32, y: i32) -> usize {
        (y as usize * 80) + x as usize
    }

    pub fn get_dims(&self) -> Point {
        Point::new(80, 50)
    }
}

struct State {
    map: Map,
    player_x: i32,
    player_y: i32,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        self.player_input(ctx);

        ctx.cls();

        let mut y = 0;

        for tile in self.map.tiles.iter() {
            let x = y % 80;
            let py = y / 80;

            match tile.tile_type {
                TileType::Floor => {
                    ctx.set(x, py, GHOSTWHITE, BLACK, to_cp437(' '));
                }
                TileType::Wall => {
                    ctx.set(x, py, RED, BLACK, to_cp437('#'));
                }
            }
            y += 1;
        }

        ctx.set(self.player_x, self.player_y, WHITE, BLACK, to_cp437('@'));
    }
}

impl BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx as usize].tile_type == TileType::Wall
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(80, 50)
    }

    fn point2d_to_index(&self, pt: Point) -> usize {
        Map::xy_to_index(pt.x, pt.y)
    }
}

impl State {
    fn player_input(&mut self, ctx: &mut BTerm) {
        if let Some(key) = ctx.key {
            let delta_x;
            let delta_y;

            match key {
                VirtualKeyCode::Left
                | VirtualKeyCode::Right
                | VirtualKeyCode::Up
                | VirtualKeyCode::Down => {
                    let (delta_x, delta_y) = match key {
                        VirtualKeyCode::Left => (-1, 0),
                        VirtualKeyCode::Right => (0, 1),
                        VirtualKeyCode::Up => (0, -1),
                        VirtualKeyCode::Down => (0, 1),
                        _ => (0, 0),
                    };
                    let new_x = self.player_x + delta_x;
                    let new_y = self.player_y + delta_y;
                    let index = Map::xy_to_index(new_x, new_y);

            if self.map.tiles[index].tile_type != TileType::Wall {
                self.player_x = new_x;
                self.player_y = new_y;
            }
        }
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("The Sonar Maze")
        .build()?;

    let mut new_map = Map {
        // Start with a vector of default tiles (Floor, not revealed)
        tiles: vec![
            Tile {
                tile_type: TileType::Floor,
                revealed: false
            };
            80 * 50
        ],
    };

    for x in 0..80 {
        let top_idx = Map::xy_to_index(x, 0);
        let bottom_idx = Map::xy_to_index(x, 49);
        new_map.tiles[top_idx].tile_type = TileType::Wall;
        new_map.tiles[bottom_idx].tile_type = TileType::Wall;
    }
    for y in 0..50 {
        let left_idx = Map::xy_to_index(0, y);
        let right_idx = Map::xy_to_index(79, y);

        new_map.tiles[left_idx].tile_type = TileType::Wall;
        new_map.tiles[right_idx].tile_type = TileType::Wall;
    }

    main_loop(
        context,
        State {
            map: new_map,
            player_x: 40,
            player_y: 25,
        },
    )
}
