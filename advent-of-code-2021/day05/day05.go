package day05

import (
	"context"
	"strings"

	"github.com/kvrhdn/advent-of-code/advent-of-code-2021/aoc"
	"github.com/kvrhdn/advent-of-code/advent-of-code-2021/shared/ints"
)

var Solution = aoc.NewDayGen(generator, part1, part2)

type line struct {
	from [2]int
	to   [2]int
}

func generator(ctx context.Context, input string) ([]line, error) {
	var lines []line

	for _, strLine := range strings.Split(input, "\n") {
		// x1,y1 -> x2,y2
		fields := strings.Fields(strLine)

		// parses x,y into [2]int{x, y}
		parseCoordinate := func(s string) ([2]int, error) {
			pair, err := ints.Parse(s, func(s string) []string {
				return strings.Split(s, ",")
			})
			if err != nil {
				return [2]int{}, err
			}
			return [2]int{pair[0], pair[1]}, nil
		}

		from, err := parseCoordinate(fields[0])
		if err != nil {
			return nil, err
		}
		to, err := parseCoordinate(fields[2])
		if err != nil {
			return nil, err
		}

		lines = append(lines, line{from, to})
	}

	return lines, nil
}

func part1(ctx context.Context, input []line) (interface{}, error) {
	// discard diagonal lines
	var lines []line
	for _, line := range input {
		if line.from[0] != line.to[0] && line.from[1] != line.to[1] {
			continue
		}
		lines = append(lines, line)
	}

	coverageMap := make(map[[2]int]int)

	for _, line := range lines {
		// add all combinations of x and y, this is fine for straight lines
		for _, x := range intsBetween(line.from[0], line.to[0]) {
			for _, y := range intsBetween(line.from[1], line.to[1]) {
				pos := [2]int{x, y}
				coverage := coverageMap[pos]
				coverageMap[pos] = coverage + 1
			}
		}
	}

	return countDangerousPlaces(coverageMap), nil
}

func part2(ctx context.Context, input []line) (interface{}, error) {
	coverageMap := make(map[[2]int]int)

	for _, line := range input {
		start := [2]int{line.from[0], line.from[1]}
		end := [2]int{line.to[0], line.to[1]}

		dir := [2]int{normalize(end[0] - start[0]), normalize(end[1] - start[1])}

		curr := start
		for curr != end {
			coverage := coverageMap[curr]
			coverageMap[curr] = coverage + 1

			curr[0] += dir[0]
			curr[1] += dir[1]
		}

		// add the end of the line as well
		coverage := coverageMap[end]
		coverageMap[end] = coverage + 1
	}

	return countDangerousPlaces(coverageMap), nil
}

func intsBetween(v1, v2 int) (values []int) {
	if v1 < v2 {
		for i := v1; i <= v2; i++ {
			values = append(values, i)
		}
	} else if v1 > v2 {
		for i := v2; i <= v1; i++ {
			values = append(values, i)
		}
	} else {
		return []int{v1}
	}
	return values
}

func countDangerousPlaces(coverageMap map[[2]int]int) (count int) {
	for _, coverage := range coverageMap {
		if coverage >= 2 {
			count += 1
		}
	}
	return
}

func normalize(value int) int {
	if value > 0 {
		return 1
	} else if value < 0 {
		return -1
	} else {
		return 0
	}
}
