package day14

import (
	"fmt"

	"github.com/kvrhdn/advent-of-code/advent-of-code-2017/shared/knotHash"
)

func SolvePart1(input string) interface{} {
	grid := parseInput(input)

	count := 0

	for row, _ := range grid {
		for _, value := range grid[row] {
			if value != 0 {
				count++
			}
		}
	}

	return count
}

func SolvePart2(input string) interface{} {
	grid := parseInput(input)

	return grid.clusterRegions()
}

func parseInput(input string) (grid Grid) {
	for r := 0; r < 128; r++ {
		hash := knotHash.DenseKnotHash(fmt.Sprintf("%v-%v", input, r))

		// a hash contains 128 bits encoded as 32 chars of 4 bits each
		for i, c := range []rune(hash) {
			var value int

			_, err := fmt.Sscanf(string(c), "%x", &value)
			if err != nil {
				panic(err)
			}

			grid[r][4*i+0] = value >> 3 & 0x1
			grid[r][4*i+1] = value >> 2 & 0x1
			grid[r][4*i+2] = value >> 1 & 0x1
			grid[r][4*i+3] = value >> 0 & 0x1
		}
	}
	return
}
