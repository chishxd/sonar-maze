use std::i32;

use bracket_lib::prelude::{Algorithm2D, *};

#[derive(Clone, Copy, PartialEq)]
enum TileType {
    Wall,
    Floor,
    Exit,
}

#[derive(PartialEq, Copy, Clone)]

struct Tile {
    tile_type: TileType,
    last_seen: i32,
}

struct Map {
    tiles: Vec<Tile>,
}

impl Map {
    fn xy_to_index(x: i32, y: i32) -> usize {
        (y as usize * 80) + x as usize
    }
}

struct PlayingState {
    map: Map,
    player_x: i32,
    player_y: i32,
    frame_time: i32,
    pings_left: i32,
}

enum State {
    MainMenu,
    Playing(PlayingState),
    GameOver,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {}
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
    fn play(&mut self, playing_state: &mut PlayingState, ctx: &mut BTerm) {
        self.player_input(ctx);

        ctx.cls();

        const REVEAL_DURATION: i32 = 20;

        let mut y = 0;

        for tile in playing_state.map.tiles.iter() {
            let x = y % 80;
            let py = y / 80;

            if playing_state.frame_time - tile.last_seen < REVEAL_DURATION {
                match tile.tile_type {
                    TileType::Floor => {
                        ctx.set(x, py, GHOSTWHITE, BLACK, to_cp437(' '));
                    }
                    TileType::Wall => {
                        ctx.set(x, py, RED, BLACK, to_cp437('#'));
                    }
                    TileType::Exit => {
                        ctx.set(x, py, MAGENTA, BLACK, to_cp437('>'));
                    }
                }
            }
            y += 1;
        }

        ctx.print(1, 1, format!("Pings Left: {}", playing_state.pings_left));

        ctx.set(
            playing_state.player_x,
            playing_state.player_y,
            WHITE,
            BLACK,
            to_cp437('@'),
        );
        playing_state.frame_time += 1;
    }

    fn player_input(&mut self, ctx: &mut BTerm) {
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::Left
                | VirtualKeyCode::Right
                | VirtualKeyCode::Up
                | VirtualKeyCode::Down => {
                    let (delta_x, delta_y) = match key {
                        VirtualKeyCode::Left => (-1, 0),
                        VirtualKeyCode::Right => (1, 0),
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

                        let new_idx = Map::xy_to_index(self.player_x, self.player_y);
                        if self.map.tiles[new_idx].tile_type == TileType::Exit {
                            ctx.quit(); //The Win Condition
                        }
                    }

                    let ext_idx = Map::xy_to_index(78, 48); //FIXME: Make ts dynamic
                    if self.pings_left == 0 && self.map.tiles[ext_idx].last_seen != i32::MAX {
                        ctx.quit(); // COOKED(Game Over)
                    }
                }
                VirtualKeyCode::Space => {
                    if self.pings_left > 0 {
                        self.reveal_map();
                        self.pings_left -= 1;
                    } else {
                        // TODO: Add audio or smth
                    }
                }
                _ => {}
            }
        }
    }

    fn reveal_map(&mut self) {
        let player_pos = Point::new(self.player_x, self.player_y);
        let fov = field_of_view(player_pos, 8, &self.map);

        for point in fov.iter() {
            let idx = Map::xy_to_index(point.x, point.y);

            if self.map.tiles[idx].tile_type == TileType::Exit {
                self.map.tiles[idx].last_seen = i32::MAX;
            } else {
                self.map.tiles[idx].last_seen = self.frame_time;
            }
        }
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("The Sonar Maze")
        .build()?;

    let mut new_map = Map {
        tiles: vec![
            Tile {
                tile_type: TileType::Floor,
                last_seen: -1000
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

    let ext_idx = Map::xy_to_index(78, 48); // FIXME: Change to dynamic value
    new_map.tiles[ext_idx].tile_type = TileType::Exit;

    main_loop(
        context,
        State {
            map: new_map,
            player_x: 40,
            player_y: 25,
            frame_time: 0,
            pings_left: 10,
        },
    )
}
