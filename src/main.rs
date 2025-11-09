use std::i32;

use bracket_lib::prelude::{Algorithm2D, DistanceAlg, *};

const SCREEN_WIDTH: i32 = 40;
const SCREEN_HEIGHT: i32 = 25;

#[derive(Clone, Copy, PartialEq)]
enum TileType {
    Wall,
    Floor,
    Exit,
    Pickup,
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
        (y as usize * SCREEN_WIDTH as usize) + x as usize
    }
}

struct MapBuilder {
    map: Map,
    rooms: Vec<Rect>,
    player_start: Point,
    exit_pos: Point,
}

#[derive(Clone, Copy, PartialEq)]
struct Rect {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

impl Rect {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Rect {
        Rect {
            x1: x,
            y1: y,
            x2: x + w,
            y2: y + h,
        }
    }

    pub fn overlaps(&self, other: &Rect) -> bool {
        self.x1 <= other.x2 && self.x2 >= other.x1 && self.y1 <= other.y2 && self.y2 >= other.y1
    }

    pub fn center(&self) -> Point {
        Point::new((self.x1 + self.x2) / 2, (self.y1 + self.y2) / 2)
    }
}

impl MapBuilder {
    fn new() -> Self {
        let mut mb = MapBuilder {
            map: Map {
                tiles: vec![
                    Tile {
                        tile_type: TileType::Wall,
                        last_seen: -1000
                    };
                    (SCREEN_WIDTH * SCREEN_HEIGHT) as usize
                ],
            },
            rooms: Vec::new(),
            player_start: Point::zero(),
            exit_pos: Point::zero(),
        };

        const MAX_ROOMS: i32 = 10;
        const MIN_SIZE: i32 = 4;
        const MAX_SIZE: i32 = 8;
        let mut rng = RandomNumberGenerator::new();

        for _ in 0..MAX_ROOMS {
            let w = rng.range(MIN_SIZE, MAX_SIZE);
            let h = rng.range(MIN_SIZE, MAX_SIZE);
            let x = rng.range(1, SCREEN_WIDTH - w - 1);
            let y = rng.range(1, SCREEN_HEIGHT - h - 1);
            let new_rooms = Rect::new(x, y, w, h);

            let mut ok: bool = true;
            for other in mb.rooms.iter() {
                if new_rooms.overlaps(other) {
                    ok = false
                }
            }
            if ok {
                // TODO: Implement apply_room_to_map() method
                // mb.apply_room_to_map();
                mb.rooms.push(new_rooms);
            }
        }

        // Connect Rooms
        mb.rooms.sort_by(|a, b| a.center().x.cmp(&b.center().x));
        for i in 0..mb.rooms.len() - 1 {
            let room_a = mb.rooms[i];
            let room_b = mb.rooms[i + 1];
            let center_a = mb.rooms[i].center();
            let center_b = mb.rooms[i + 1].center();

            // TODO: implement apply_horizontal and vertical_tunnels func
            // mb.apply_horizontal_tunnel(center_a.x, center_b.x, center_a.y);
            // mb.apply_vertical_tunnel(center_a.y, center_b.y, center_b.x);
        }

        mb
    }

    fn find_farthest_exit(&self) -> Point {
        let mut farthest_distance = 0.0;
        let mut farthest_pos = Point::zero();

        for (idx, tile) in self.map.tiles.iter().enumerate() {
            if tile.tile_type == TileType::Floor {
                let x = idx % SCREEN_WIDTH as usize;
                let y = idx / SCREEN_WIDTH as usize;
                let pos = Point::new(x, y);

                let distance = DistanceAlg::Pythagoras.distance2d(pos, self.player_start);
                if distance > farthest_distance {
                    farthest_distance = distance;
                    farthest_pos = pos;
                }
            }
        }
        farthest_pos
    }
}

struct PlayingState {
    map: Map,
    player_x: i32,
    player_y: i32,
    exit_x: i32,
    exit_y: i32,
    frame_time: i32,
    pings_left: i32,
    depth: i32,
}

enum State {
    MainMenu,
    Playing(PlayingState),
    GameOver,
    Victory,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self {
            State::MainMenu => self.main_menu(ctx),
            State::Playing(playing_state) => {
                // Call the static function, which no longer borrows self
                let next_state = Self::player_input(ctx, playing_state);

                // Draw the current frame
                Self::play(playing_state, ctx);

                // AFTER drawing, check if we need to change state for the NEXT frame
                if let Some(new_state) = next_state {
                    *self = new_state;
                }
            }

            State::GameOver => self.game_over(ctx),
            State::Victory => self.victory(ctx),
        }
    }
}

impl BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx as usize].tile_type == TileType::Wall
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(SCREEN_WIDTH, SCREEN_HEIGHT)
    }

    fn point2d_to_index(&self, pt: Point) -> usize {
        Map::xy_to_index(pt.x, pt.y)
    }
}

impl State {
    // The Main menu
    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();

        ctx.print_centered(5, "Welcome to Sonar Maze");
        ctx.print_centered(8, "Please press ENTER to start the game");

        if let Some(VirtualKeyCode::Return) = ctx.key {
            *self = State::new_game();
        }
    }

    // The main game logic
    fn play(playing_state: &mut PlayingState, ctx: &mut BTerm) {
        ctx.cls();

        const REVEAL_DURATION: i32 = 20;

        let mut y = 0;

        for tile in playing_state.map.tiles.iter() {
            let x = y % SCREEN_WIDTH;
            let py = y / SCREEN_WIDTH;

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
                    TileType::Pickup => {
                        ctx.set(x, py, CYAN, BLACK, to_cp437('+'));
                    }
                }
            }
            y += 1;
        }

        ctx.print(
            1,
            1,
            format!(
                "Depth: {} | Pings Left: {}",
                playing_state.depth, playing_state.pings_left
            ),
        );

        ctx.set(
            playing_state.player_x,
            playing_state.player_y,
            WHITE,
            BLACK,
            to_cp437('@'),
        );
        playing_state.frame_time += 1;
    }

    //  ============ GAME OVER DISPLAY ==================
    fn game_over(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "You are lost in the dark");
        ctx.print_centered(8, "Press ENTER to try again");

        if let Some(VirtualKeyCode::Return) = ctx.key {
            *self = State::new_game();
        }
    }

    // ================= VICTORY DISPLAY ======================

    fn victory(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "You have escaped the darkness!");
        ctx.print_centered(8, "Press ENTER to play again");

        if let Some(VirtualKeyCode::Return) = ctx.key {
            *self = State::new_game();
        }
    }

    fn new_game() -> State {
        let mb = MapBuilder::new();

        let playing_state = PlayingState {
            map: mb.map,
            player_x: mb.player_start.x,
            player_y: mb.player_start.y,
            exit_x: mb.exit_pos.x,
            exit_y: mb.exit_pos.y,
            frame_time: 0,
            pings_left: 15,
            depth: 1,
        };

        State::Playing(playing_state)
    }

    // This function moves the game to next level
    fn new_level(playing_state: &mut PlayingState) -> State {
        let mb = MapBuilder::new();

        let new_playing_state = PlayingState {
            map: mb.map,
            player_x: mb.player_start.x,
            player_y: mb.player_start.y,
            exit_x: mb.exit_pos.x,
            exit_y: mb.exit_pos.y,
            frame_time: 0,
            pings_left: playing_state.pings_left + 5,
            depth: playing_state.depth + 1,
        };

        State::Playing(new_playing_state)
    }

    fn player_input(ctx: &mut BTerm, playing_state: &mut PlayingState) -> Option<State> {
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
                    let new_x = playing_state.player_x + delta_x;
                    let new_y = playing_state.player_y + delta_y;
                    let index = Map::xy_to_index(new_x, new_y);

                    if playing_state.map.tiles[index].tile_type != TileType::Wall {
                        playing_state.player_x = new_x;
                        playing_state.player_y = new_y;

                        let new_idx =
                            Map::xy_to_index(playing_state.player_x, playing_state.player_y);

                        let new_tile = &mut playing_state.map.tiles[new_idx];

                        if new_tile.tile_type == TileType::Pickup {
                            playing_state.pings_left += 1;

                            new_tile.tile_type = TileType::Floor;
                        }

                        if playing_state.map.tiles[new_idx].tile_type == TileType::Exit {
                            if playing_state.depth == 3 {
                                return Some(State::Victory); //The Win Condition
                            } else {
                                return Some(State::new_level(playing_state));
                            }
                        }
                    }

                    let ext_idx = Map::xy_to_index(playing_state.exit_x, playing_state.exit_y);
                    if playing_state.pings_left == 0
                        && playing_state.map.tiles[ext_idx].last_seen != i32::MAX
                    {
                        return Some(State::GameOver); // COOKED(Game Over)
                    }
                }
                VirtualKeyCode::Space => {
                    if playing_state.pings_left > 0 {
                        Self::reveal_map(playing_state);
                        playing_state.pings_left -= 1;
                    } else {
                        // TODO: Add audio or smth
                    }
                }

                // The cheat code to reveal map
                VirtualKeyCode::Tab => {
                    for tile in playing_state.map.tiles.iter_mut() {
                        tile.last_seen = i32::MAX;
                    }
                }

                _ => {}
            }
        }
        None
    }

    fn reveal_map(playing_state: &mut PlayingState) {
        let player_pos = Point::new(playing_state.player_x, playing_state.player_y);
        let fov = field_of_view(player_pos, 8, &playing_state.map);

        for point in fov.iter() {
            let idx = Map::xy_to_index(point.x, point.y);

            if playing_state.map.tiles[idx].tile_type == TileType::Exit {
                playing_state.map.tiles[idx].last_seen = i32::MAX;
            } else {
                playing_state.map.tiles[idx].last_seen = playing_state.frame_time;
            }
        }
    }
}

fn main() -> BError {
    let context = BTermBuilder::new()
        .with_title("Sonar Maze")
        .with_resource_path("resources/")
        .with_font("vga8x16.png", 8, 16)
        .with_simple_console(SCREEN_WIDTH, SCREEN_HEIGHT, "vga8x16.png")
        .build()?;

    main_loop(context, State::MainMenu)
}
