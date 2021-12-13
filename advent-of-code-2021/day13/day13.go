package day13

import (
	"context"
	"strconv"
	"strings"

	"github.com/kvrhdn/advent-of-code/advent-of-code-2021/aoc"
	"github.com/kvrhdn/advent-of-code/advent-of-code-2021/shared/ints"
	"github.com/kvrhdn/advent-of-code/advent-of-code-2021/shared/set"
)

var Solution = aoc.NewDayGen(generator, part1, part2)

type point struct {
	x, y int
}

type foldLine struct {
	axis     string
	position int
}

func (f *foldLine) fold(dots set.Set[point]) {
	for _, dot := range dots.Values() {
		switch f.axis {
		case "x":
			if dot.x >= f.position {
				dots.Remove(dot)
				dots.Add(point{dot.x - (2 * (dot.x - f.position)), dot.y})
			}
		case "y":
			if dot.y >= f.position {
				dots.Remove(dot)
				dots.Add(point{dot.x, dot.y - (2 * (dot.y - f.position))})
			}
		}
	}
}

type manual struct {
	dots      set.Set[point]
	foldLines []foldLine
}

func generator(ctx context.Context, input string) (manual, error) {
	paragraphs := strings.Split(input, "\n\n")

	dots := set.New[point]()
	for _, line := range strings.Split(paragraphs[0], "\n") {
		coordinates, err := ints.Parse(line, func(s string) []string {
			return strings.Split(s, ",")
		})
		if err != nil {
			return manual{}, err
		}

		dots.Add(point{coordinates[0], coordinates[1]})
	}

	var foldLines []foldLine
	for _, line := range strings.Split(paragraphs[1], "\n") {
		line = strings.TrimPrefix(line, "fold along ")
		instruction := strings.Split(line, "=")

		position, err := strconv.Atoi(instruction[1])
		if err != nil {
			return manual{}, err
		}

		foldLines = append(foldLines, foldLine{instruction[0], position})
	}

	return manual{dots, foldLines}, nil
}

func part1(ctx context.Context, input manual) (interface{}, error) {
	dots := input.dots.Copy()

	input.foldLines[0].fold(dots)

	return dots.Len(), nil
}

func part2(ctx context.Context, input manual) (interface{}, error) {
	dots := input.dots.Copy()

	for _, foldLine := range input.foldLines {
		foldLine.fold(dots)
	}

	maxX := 0
	maxY := 0
	for _, dot := range dots.Values() {
		if dot.x > maxX {
			maxX = dot.x
		}
		if dot.y > maxY {
			maxY = dot.y
		}
	}

	var output strings.Builder

	for y := 0; y <= maxY; y++ {
		for x := 0; x <= maxX; x++ {
			if dots.Contains(point{x, y}) {
				output.WriteRune('#')
			} else {
				output.WriteRune(' ')
			}
		}
		output.WriteRune('\n')
	}

	return output.String(), nil
}
