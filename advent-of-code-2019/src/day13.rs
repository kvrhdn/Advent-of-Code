use crate::intcode::load_program;
use aoc_runner_derive::aoc;
use arcade::*;

mod arcade;

#[aoc(day13, part1)]
fn solve_part1(input: &str) -> usize {
    let program = load_program(input).unwrap();
    let mut arcade = Arcade::new(program);

    arcade.run().unwrap();

    arcade.tiles(Tile::Block).count()
}

#[aoc(day13, part2)]
fn solve_part2(input: &str) -> i64 {
    let program = load_program(input).unwrap();
    let mut arcade = Arcade::new(program);

    arcade.insert_quarter().unwrap();

    loop {
        arcade.run().unwrap();

        if arcade.get_state() != State::AwaitingInput {
            break;
        }

        // let the paddle follow the ball
        let ball = arcade.tiles(Tile::Ball).next().unwrap();
        let paddle = arcade.tiles(Tile::HorizontalPaddle).next().unwrap();

        let joystick_position = match paddle.x {
            _ if ball.x < paddle.x => JoystickPosition::Left,
            _ if ball.x > paddle.x => JoystickPosition::Right,
            _ => JoystickPosition::Neutral,
        };

        arcade.set_joystick(joystick_position);
    }

    arcade.get_score()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn real_input() {
        let input = include_str!("../input/2019/day13.txt");

        assert_eq!(solve_part1(input), 216);
        assert_eq!(solve_part2(input), 10025);
    }
}
