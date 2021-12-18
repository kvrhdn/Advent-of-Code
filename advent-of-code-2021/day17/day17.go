package day17

import (
	"context"
	"errors"
	"regexp"
	"strconv"

	"github.com/kvrhdn/advent-of-code/advent-of-code-2021/aoc"
)

var Solution = aoc.NewDayGen(generator, part1, part2)

type vec2 struct {
	x, y int
}

func (v *vec2) Add(other vec2) {
	v.x += other.x
	v.y += other.y
}

func (v *vec2) Within(area [2]vec2) bool {
	// assumption
	if area[0].x >= area[1].x || area[0].y >= area[1].y {
		panic("area[0] must be smaller than area[1]")
	}

	return v.x >= area[0].x && v.x <= area[1].x &&
		v.y >= area[0].y && v.y <= area[1].y
}

func generator(ctx context.Context, input string) ([2]vec2, error) {
	re := regexp.MustCompile("-?[0-9]+")

	matches := re.FindAllString(input, -1)

	var numbers []int
	for _, match := range matches {
		n, err := strconv.Atoi(match)
		if err != nil {
			return [2]vec2{}, err
		}
		numbers = append(numbers, n)
	}

	return [2]vec2{
		{x: numbers[0], y: numbers[2]},
		{x: numbers[1], y: numbers[3]},
	}, nil
}

// followProbeTrajectory launches a probe from (0, 0) with the given velocity. Every
// position is passed to visit until visit returns false.
func followProbeTrajectory(v vec2, visit func(vec2) bool) {
	// assumption: the target area always has positive x values
	if v.x < 0 {
		panic("x velocity must be greater than or equal to 0")
	}

	p := vec2{0, 0}
	for {
		p.Add(v)

		// apply drag
		if v.x > 0 {
			v.x -= 1
		}
		v.y -= 1

		if !visit(p) {
			break
		}
	}
}

func probeTrajectoryReachesTargetArea(v vec2, area [2]vec2) (maxHeight int, reaches bool) {
	followProbeTrajectory(v, func(p vec2) bool {
		if p.y > maxHeight {
			maxHeight = p.y
		}

		if p.x > area[1].x || p.y < area[0].y {
			// we passed the target area
			return false
		}
		if p.Within(area) {
			reaches = true
			return false
		}

		return true
	})
	return
}

func part1(ctx context.Context, input [2]vec2) (interface{}, error) {
	// just try all vertical speeds from +100 until 0 ¯\_(ツ)_/¯
	for y := 100; y >= 0; y-- {
		for x := 0; x <= input[1].x; x++ {
			height, reached := probeTrajectoryReachesTargetArea(vec2{x, y}, input)
			if reached {
				return height, nil
			}
		}
	}
	return nil, errors.New("probe never reached the target area")
}

func part2(ctx context.Context, input [2]vec2) (interface{}, error) {
	hasReached := 0

	// just try all vertical speeds from +100 until min-Y of target area
	for y := 100; y >= input[0].y; y-- {
		for x := 0; x <= input[1].x; x++ {
			_, reached := probeTrajectoryReachesTargetArea(vec2{x, y}, input)
			if reached {
				hasReached += 1
			}
		}
	}

	return hasReached, nil
}
