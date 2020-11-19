package day03

import (
	"strconv"
)

func SolvePart1(input string) interface{} {
	inputValue := parseInput(input)

	count := 0

	position := walkInSpiral(
		func(_ Pos) {
			count += 1
		},
		func(_ Pos) bool {
			return count == inputValue
		},
	)

	return position.ManhattanDistanceToOrigin()
}

func SolvePart2(input string) interface{} {
	inputValue := parseInput(input)

	data := make(map[Pos]int)

	// add initial value
	data[Pos{0, 0}] = 1

	position := walkInSpiral(
		func(pos Pos) {
			sum := 0

			for i := -1; i <= 1; i++ {
				for j := -1; j <= 1; j++ {
					sum += data[Pos{pos.X + i, pos.Y + j}]
				}
			}

			data[pos] = sum
		},
		func(pos Pos) bool {
			return data[pos] > inputValue
		},
	)

	return data[position]
}

func parseInput(input string) int {
	value, err := strconv.Atoi(input)
	if err != nil {
		panic(err)
	}
	return value
}

func walkInSpiral(eachStep func(Pos), stopWhen func(Pos) bool) Pos {
	pos := Pos{0, 0}

	direction := Right
	var minX, maxX, minY, maxY int

	for {
		eachStep(pos)

		if stopWhen(pos) {
			return pos
		}

		pos = pos.Move(direction)

		switch direction {
		case Up:
			if pos.Y > maxY {
				maxY = pos.Y
				direction = direction.TurnLeft()
			}
		case Left:
			if pos.X < minX {
				minX = pos.X
				direction = direction.TurnLeft()
			}
		case Down:
			if pos.Y < minY {
				minY = pos.Y
				direction = direction.TurnLeft()
			}
		case Right:
			if pos.X > maxX {
				maxX = pos.X
				direction = direction.TurnLeft()
			}
		}
	}
}
