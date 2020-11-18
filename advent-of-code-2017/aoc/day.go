package aoc

import (
	"fmt"
	"time"
)

type PuzzleSolver func(input string) interface{}

type Day struct {
	Part1 PuzzleSolver
	Part2 PuzzleSolver
}

func (d *Day) run(year, day int, part *int) {
	input := readInput(year, day)

	if part == nil || *part == 1 {
		d.runPart(day, 1, input)
	}

	if part == nil || *part == 2 {
		d.runPart(day, 2, input)
	}
}

func (d *Day) runPart(day, part int, input string) {
	var solver PuzzleSolver

	if part == 1 {
		solver = d.Part1
	} else if part == 2 {
		solver = d.Part2
	}

	if solver == nil {
		return
	}

	result, elapsed := solveTimed(solver, input)

	fmt.Printf("Day %02d - part %d:  %v\n", day, part, result)
	fmt.Printf("         elapsed: %s\n\n", elapsed)
}

func solveTimed(solver PuzzleSolver, input string) (result interface{}, elapsed time.Duration) {
	start := time.Now()
	defer func() {
		elapsed = time.Since(start)
	}()

	result = solver(input)

	return
}
