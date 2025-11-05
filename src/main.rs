use bracket_lib::prelude::*;

struct State {
    player_x: i32,
    player_y: i32,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();

        ctx.set(self.player_x, self.player_y, WHITE, BLACK, to_cp437('@'));
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("The Sonar Maze")
        .build()?;

    main_loop(
        context,
        State {
            player_x: 40,
            player_y: 25,
        },
    )
}
