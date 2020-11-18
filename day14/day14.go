package main

import (
	"fmt"
	"github.com/koenaad/Advent-of-Code-2017/day14/grid"
)

var input = "wenycdww"

func main() {
	fmt.Println("Advent of Code 2017 - day 14")

	g := grid.InitGridKnotHash(input)

	fmt.Printf("Puzzle 1: squares used = %v", g.SquaresOccupied())
	fmt.Printf("Puzzle 2: regions after regionalizing = %v", g.Regionalize())
}
