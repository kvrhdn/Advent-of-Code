package main

import (
	"fmt"
	"github.com/koenaad/Advent-of-Code-2017/day10/knotHash"
)

var input = "wenycdww"

func main() {
	fmt.Println("Advent of Code 2017 - day 14")

	g := initGrid(input)

	fmt.Printf("Puzzle 1: squares used = %v", g.SquaresUsed())
}

func initGrid(input string) (g Grid) {
	for r := 0; r < 128; r++ {
		hash := knotHash.DenseKnotHash(fmt.Sprintf("%v-%v", input, r))

		// a hash contains 128 bits encoded as 32 chars of 4 bits each
		for i, c := range []rune(hash) {
			var value int

			_, err := fmt.Sscanf(string(c), "%x", &value)
			if err != nil {
				panic(err)
			}

			g.d[r][(4*i)+0] = (value & 0x8) > 0
			g.d[r][(4*i)+1] = (value & 0x4) > 0
			g.d[r][(4*i)+2] = (value & 0x2) > 0
			g.d[r][(4*i)+3] = (value & 0x1) > 0
		}
	}

	return g
}

type Grid struct {
	d [128][128]bool
}

func (g *Grid) SquaresUsed() int {
	count := 0

	for _, row := range g.d {
		for _, sq := range row {
			if sq {
				count += 1
			}
		}
	}

	return count
}
