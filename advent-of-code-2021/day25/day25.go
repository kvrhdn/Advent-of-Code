package day25

import (
	"context"
	"fmt"
	"strings"

	"github.com/kvrhdn/advent-of-code/advent-of-code-2021/aoc"
	"github.com/kvrhdn/advent-of-code/advent-of-code-2021/shared/set"
)

var Solution = aoc.NewDayGen(generator, part1, nil)

type vec2 struct {
	x, y int
}

type seaCucumberMap struct {
	width, height         int
	movesEast, movesSouth set.Set[vec2]
}

func (m *seaCucumberMap) Println() {
	for y := 0; y < m.height; y++ {
		for x := 0; x < m.width; x++ {
			p := vec2{x, y}
			if m.movesEast.Contains(p) {
				fmt.Print(">")
			} else if m.movesSouth.Contains(p) {
				fmt.Print("v")
			} else {
				fmt.Print(".")
			}
		}
		fmt.Println()
	}
}

func generator(ctx context.Context, input string) (m seaCucumberMap, err error) {
	m.movesEast = set.New[vec2]()
	m.movesSouth = set.New[vec2]()

	for y, line := range strings.Split(input, "\n") {
		for x, char := range line {
			if char == '>' {
				m.movesEast.Add(vec2{x, y})
			}
			if char == 'v' {
				m.movesSouth.Add(vec2{x, y})
			}

			m.width = x + 1
		}

		m.height = y + 1
	}

	return m, nil
}

func part1(ctx context.Context, m seaCucumberMap) (interface{}, error) {
	steps := 0

	for {
		cucumberMoved := false
		newMovesEast := set.New[vec2]()
		newMovesSouth := set.New[vec2]()

		for _, c := range m.movesEast.Values() {
			newPosition := vec2{(c.x + 1) % m.width, c.y}
			if m.movesEast.Contains(newPosition) || m.movesSouth.Contains(newPosition) {
				newMovesEast.Add(c)
			} else {
				newMovesEast.Add(newPosition)
				cucumberMoved = true
			}
		}

		for _, c := range m.movesSouth.Values() {
			newPosition := vec2{c.x, (c.y + 1) % m.height}
			if newMovesEast.Contains(newPosition) || m.movesSouth.Contains(newPosition) {
				newMovesSouth.Add(c)
			} else {
				newMovesSouth.Add(newPosition)
				cucumberMoved = true
			}
		}

		steps += 1
		m.movesEast = newMovesEast
		m.movesSouth = newMovesSouth

		//fmt.Println("---", steps, "---")
		//m.Println()
		//fmt.Println()

		if !cucumberMoved {
			return steps, nil
		}
	}
}
