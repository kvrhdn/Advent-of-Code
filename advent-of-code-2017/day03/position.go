package day03

import (
	"fmt"
	"math"
)

type Direction int

const (
	Up Direction = iota
	Down
	Left
	Right
)

func (direction Direction) TurnLeft() Direction {
	switch direction {
	case Up:
		return Left
	case Left:
		return Down
	case Down:
		return Right
	case Right:
		return Up
	}
	panic(fmt.Sprintf("Unrecognized direction %v", direction))
}

type Pos struct {
	X, Y int
}

func (pos Pos) Move(direction Direction) Pos {
	switch direction {
	case Up:
		return Pos{pos.X, pos.Y + 1}
	case Down:
		return Pos{pos.X, pos.Y - 1}
	case Left:
		return Pos{pos.X - 1, pos.Y}
	case Right:
		return Pos{pos.X + 1, pos.Y}
	}
	panic(fmt.Sprintf("Unrecognized direction %v", direction))
}

func (pos Pos) String() string {
	return fmt.Sprintf("(%v, %v)", pos.X, pos.Y)
}

func (pos Pos) ManhattanDistanceToOrigin() int {
	return int(math.Abs(float64(pos.X)) + math.Abs(float64(pos.Y)))
}
