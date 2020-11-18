package day03

import (
	"strconv"

	"github.com/kvrhdn/advent-of-code/advent-of-code-2017/util"
)

func SolvePart1(input string) interface{} {
	return DetermineDistanceToStorageLocationInSpiralMemory(input)
}

func SolvePart2(input string) interface{} {
	return FirstLoadtestValueGreaterThan(input)
}

func doTheSpiral(eachStep func(util.Pos), stopWhen func(util.Pos) bool) util.Pos {
	pos := util.Pos{0, 0}

	direction := 'R'
	var minX, maxX, minY, maxY int

	for {
		eachStep(pos)

		if stopWhen(pos) {
			return pos
		}

		switch direction {
		case 'R':
			pos.X += 1
			if pos.X > maxX {
				maxX = pos.X
				direction = 'U'
			}
		case 'L':
			pos.X -= 1
			if pos.X < minX {
				minX = pos.X
				direction = 'D'
			}

		case 'U':
			pos.Y += 1
			if pos.Y > maxY {
				maxY = pos.Y
				direction = 'L'
			}
		case 'D':
			pos.Y -= 1
			if pos.Y < minY {
				minY = pos.Y
				direction = 'R'
			}
		}
	}
}

func DetermineDistanceToStorageLocationInSpiralMemory(inputString string) interface{} {
	input, err := strconv.Atoi(inputString)
	if err != nil {
		panic(err)
	}

	if input < 1 {
		panic("input can't be smaller than 1")
	}

	currData := 0

	eachStep := func(util.Pos) {
		currData += 1
	}
	stopWhen := func(util.Pos) bool {
		return currData >= input
	}

	lastPos := doTheSpiral(eachStep, stopWhen)

	return util.ManhattenDistance(util.Pos{0, 0}, lastPos)
}

func FirstLoadtestValueGreaterThan(inputString string) interface{} {
	input, err := strconv.Atoi(inputString)
	if err != nil {
		panic(err)
	}

	grid := NewZeroGrid()

	eachStep := func(pos util.Pos) {
		if pos.X == 0 && pos.Y == 0 {
			grid.Set(pos, 1)
		} else {
			sum := 0

			for i := -1; i <= 1; i++ {
				for j := -1; j <= 1; j++ {
					sum += grid.Get(util.Pos{pos.X + i, pos.Y + j})
				}
			}

			grid.Set(pos, sum)
		}
	}
	stopWhen := func(pos util.Pos) bool {
		return grid.Get(pos) > input
	}

	lastPos := doTheSpiral(eachStep, stopWhen)

	return grid.Get(lastPos)
}

type ZeroGrid struct {
	grid map[util.Pos]int
}

func NewZeroGrid() ZeroGrid {
	return ZeroGrid{make(map[util.Pos]int)}
}

func (g *ZeroGrid) Get(pos util.Pos) int {
	return g.grid[pos]
}

func (g *ZeroGrid) Set(pos util.Pos, value int) {
	g.grid[pos] = value
}
