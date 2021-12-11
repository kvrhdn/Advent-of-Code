package day11

import (
	"context"
	"math"
	"strings"

	"github.com/kvrhdn/advent-of-code/advent-of-code-2021/aoc"
	"github.com/kvrhdn/advent-of-code/advent-of-code-2021/shared/slices"
)

var Solution = aoc.NewDayGen(generator, part1, part2)

func generator(ctx context.Context, input string) ([]int, error) {
	var octopuses []int

	for _, line := range strings.Split(input, "\n") {
		for _, char := range line {
			octopuses = append(octopuses, int(char-'0'))
		}
	}

	return octopuses, nil
}

// performStep updates octopuses for one step and returns the amount of octopuses
// that flashed. Note this modifies the given slice in-place.
func performStep(octopuses []int) (flashes int) {
	// increase energy level of every octopus
	for i := range octopuses {
		octopuses[i] += 1
	}

	var octopusHasFlashed []int

	// loop until no more octopus flashes
	for {
		var octopusWillFlash []int

		// if energy level is above 9, octopus will flash!
		for id, energyLevel := range octopuses {
			if energyLevel > 9 {
				octopuses[id] = math.MinInt // temporary value to ensure it can not flash again this step
				octopusWillFlash = append(octopusWillFlash, id)
			}
		}

		if len(octopusWillFlash) == 0 {
			break
		}

		// increase energy level of neighbouring octopuses
		for _, id := range octopusWillFlash {
			safeIncrease := func(id int) {
				if id >= 0 && id < len(octopuses) {
					octopuses[id] += 1
				}
			}

			if id%10 != 0 { // only if not part of the left border
				safeIncrease(id - 10 - 1)
				safeIncrease(id - 1)
				safeIncrease(id + 10 - 1)
			}

			safeIncrease(id - 10)
			safeIncrease(id + 10)

			if id%10 != 9 { // only if not part of the right border
				safeIncrease(id - 10 + 1)
				safeIncrease(id + 1)
				safeIncrease(id + 10 + 1)
			}

			octopusHasFlashed = append(octopusHasFlashed, id)
		}
	}

	// after flashing energy level is 0
	for _, id := range octopusHasFlashed {
		octopuses[id] = 0
	}

	return len(octopusHasFlashed)
}

func part1(ctx context.Context, input []int) (interface{}, error) {
	octopuses := slices.Copy[int](input)
	flashes := 0

	for step := 0; step < 100; step++ {
		flashes += performStep(octopuses)
	}

	return flashes, nil
}

func part2(ctx context.Context, input []int) (interface{}, error) {
	octopuses := slices.Copy[int](input)

	step := 0
	for {
		flashes := performStep(octopuses)
		step += 1

		if flashes == 100 {
			return step, nil
		}
	}
}
