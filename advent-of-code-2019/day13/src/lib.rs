mod arcade;
mod sparse_grid;

use arcade::*;
use common::console_utils::Timer;
use intcode::load_program;
use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// See: https://adventofcode.com/2019/day/13
#[wasm_bindgen]
pub fn part1(input: &str) -> Result<u32, JsValue> {
    Timer::new("rust::part1");

    let program = load_program(input)?;
    let mut arcade = Arcade::new(program);

    arcade.run()?;

    let block_count = arcade
        .get_screen()
        .iterate()
        .filter(|&tile| *tile == Tile::Block)
        .count() as u32;

    Ok(block_count)
}

/// See: https://adventofcode.com/2019/day/13#part2
#[wasm_bindgen]
pub fn part2(input: &str) -> Result<i32, JsValue> {
    Timer::new("rust::part2");

    let program = load_program(input)?;
    let mut arcade = Arcade::new(program);

    arcade.insert_quarters();

    loop {
        arcade.run()?;

        if arcade.get_state() != State::AwaitingInput {
            break;
        }

        move_joystick_to_follow_ball(&mut arcade)?;
    }

    Ok(arcade.get_score())
}

fn move_joystick_to_follow_ball(arcade: &mut Arcade) -> Result<(), &'static str> {
    let ball = arcade
        .get_screen()
        .find(Tile::Ball)
        .ok_or("could not find ball on screen")?;
    let paddle = arcade
        .get_screen()
        .find(Tile::HorizontalPaddle)
        .ok_or("could not find paddle on screen")?;

    let joystick_position = match paddle.x {
        _ if ball.x < paddle.x => JoystickPosition::Left,
        _ if ball.x > paddle.x => JoystickPosition::Right,
        _ => JoystickPosition::Neutral,
    };

    arcade.set_joystick(joystick_position);

    Ok(())
}
