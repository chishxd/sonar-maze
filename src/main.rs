use bracket_lib::prelude::*;

#[derive(Clone, Copy, PartialEq)]

enum TileType {
    Wall,
    Floor,
}

struct Map {
    tiles: Vec<TileType>,
}

impl Map {
    fn xy_to_index(x: i32, y: i32) -> usize {
        (y as usize * 80) + x as usize
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

            match tile {
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

impl State {
    fn player_input(&mut self, ctx: &mut BTerm) {
        if let Some(key) = ctx.key {
            let delta_x;
            let delta_y;

            match key {
                VirtualKeyCode::Left => {
                    delta_x = -1;
                    delta_y = 0;
                }
                VirtualKeyCode::Right => {
                    delta_x = 1;
                    delta_y = 0
                }
                VirtualKeyCode::Up => {
                    delta_x = 0;
                    delta_y = -1;
                }
                VirtualKeyCode::Down => {
                    delta_x = 0;
                    delta_y = 1;
                }

                _ => {
                    return;
                }
            }

            let new_x = self.player_x + delta_x;
            let new_y = self.player_y + delta_y;
            let index = Map::xy_to_index(new_x, new_y);

            if self.map.tiles[index] != TileType::Wall {
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
        tiles: vec![TileType::Floor; 80 * 50],
    };

    for x in 0..80 {
        new_map.tiles[Map::xy_to_index(x, 0)] = TileType::Wall;
        new_map.tiles[Map::xy_to_index(x, 49)] = TileType::Wall;
    }
    for y in 0..50 {
        new_map.tiles[Map::xy_to_index(0, y)] = TileType::Wall;
        new_map.tiles[Map::xy_to_index(79, y)] = TileType::Wall;
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
