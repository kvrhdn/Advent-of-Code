use arcade::*;
use crate::intcode::load_program;
use aoc_runner_derive::aoc;

mod arcade;
mod sparse_grid;

/// See: https://adventofcode.com/2019/day/13
#[aoc(day13, part1)]
pub fn solve_part1(input: &str) -> Result<u32, &'static str> {
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
#[aoc(day13, part2)]
pub fn solve_part2(input: &str) -> Result<i32, &'static str> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn real_input() {
        let input = include_str!("../input/2019/day13.txt");

        assert_eq!(solve_part1(input), Ok(216));
        assert_eq!(solve_part2(input), Ok(10025));
    }
}
