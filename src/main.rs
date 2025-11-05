use bracket_lib::prelude::*;

struct State {
    player_x: i32,
    player_y: i32,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        self.player_input(ctx);

        ctx.cls();

        ctx.set(self.player_x, self.player_y, WHITE, BLACK, to_cp437('@'));
    }
}

impl State {
    fn player_input(&mut self, ctx: &mut BTerm) {
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::Left => self.player_x -= 1,
                VirtualKeyCode::Right => self.player_x += 1,
                VirtualKeyCode::Up => self.player_y -= 1,
                VirtualKeyCode::Down => self.player_y += 1,

                _ => {}
            }
        }
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
