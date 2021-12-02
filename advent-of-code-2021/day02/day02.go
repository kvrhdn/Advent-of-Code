package day02

import (
	"context"
	"fmt"
	"strconv"
	"strings"

	"github.com/kvrhdn/advent-of-code/advent-of-code-2021/aoc"
)

var Solution = aoc.NewDay(part1, part2)

const (
	forward string = "forward"
	down           = "down"
	up             = "up"
)

func iterateCommands(input string, fn func(cmd string, amount int)) error {
	for _, l := range strings.Split(input, "\n") {
		segments := strings.Split(l, " ")
		if len(segments) != 2 {
			return fmt.Errorf("command has unexpected format: %v", segments)
		}

		amount, err := strconv.Atoi(segments[1])
		if err != nil {
			return err
		}

		fn(segments[0], amount)
	}
	return nil
}

func part1(ctx context.Context, input string) (interface{}, error) {
	type Submarine struct {
		position, depth int
	}
	sub := Submarine{}

	err := iterateCommands(input, func(cmd string, amount int) {
		switch cmd {
		case forward:
			sub.position += amount
		case up:
			sub.depth -= amount
		case down:
			sub.depth += amount
		}
	})

	return sub.position * sub.depth, err
}

func part2(ctx context.Context, input string) (interface{}, error) {
	type Submarine struct {
		position, depth, aim int
	}
	sub := Submarine{}

	err := iterateCommands(input, func(cmd string, amount int) {
		switch cmd {
		case forward:
			sub.position += amount
			sub.depth += sub.aim * amount
		case up:
			sub.aim -= amount
		case down:
			sub.aim += amount
		}
	})

	return sub.position * sub.depth, err
}
