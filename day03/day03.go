package main

import (
	"fmt"
	"github.com/koenaad/Advent-of-Code-2017/util"
)

var input = 277678

func main() {
	fmt.Println("Advent of Code 2017 - day 03")

	fmt.Println("solution 1 =", ManhattanDistanceFromStorageLocationOf(input))
	fmt.Println("solution 2 =", FirstLoadtestValueGreaterThan(input))
}

func ManhattanDistanceFromStorageLocationOf(data int) int {
	dataPos := determineStorageLocationInSpiralMemory(data)

	return util.ManhattenDistance(dataPos, util.Pos{0, 0})
}

func determineStorageLocationInSpiralMemory(data int) util.Pos {
	if data < 1 {
		panic("data can't smaller than 1")
	}

	pos := util.Pos{0, 0}

	direction := 'R'

	minX := 0
	maxX := 0
	minY := 0
	maxY := 0

	fmt.Printf("determining position of %v\n", data)

	for currData := 1; currData < data; currData++ {

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

		fmt.Printf("\tcurrent data = %v, position = %v\n", currData, pos)
	}
	fmt.Printf("position of data %v: %v\n", data, pos)

	return pos
}

func FirstLoadtestValueGreaterThan(i int) int {
	return 0
}
