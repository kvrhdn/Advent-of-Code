package day22

import (
	"context"
	"regexp"
	"strconv"
	"strings"

	"github.com/kvrhdn/advent-of-code/advent-of-code-2021/aoc"
	"github.com/kvrhdn/advent-of-code/advent-of-code-2021/shared/set"
)

type vec3 struct {
	x, y, z int
}

type cube struct {
	on         bool
	xMin, xMax int
	yMin, yMax int
	zMin, zMax int
}

var Solution = aoc.NewDayGen(generator, part1, part2)

func generator(ctx context.Context, input string) ([]cube, error) {
	var cubes []cube

	re := regexp.MustCompile("-?[0-9]+")
	for _, line := range strings.Split(input, "\n") {
		var nums []int

		matches := re.FindAllString(line, -1)
		for _, match := range matches {
			n, err := strconv.Atoi(match)
			if err != nil {
				return nil, err
			}
			nums = append(nums, n)
		}

		c := cube{
			on:   strings.HasPrefix(line, "on "),
			xMin: min(nums[0], nums[1]),
			xMax: max(nums[0], nums[1]),
			yMin: min(nums[2], nums[3]),
			yMax: max(nums[2], nums[3]),
			zMin: min(nums[4], nums[5]),
			zMax: max(nums[4], nums[5]),
		}
		cubes = append(cubes, c)
	}

	return cubes, nil
}

func part1(ctx context.Context, input []cube) (interface{}, error) {
	space := set.New[vec3]()

	for _, c := range input {
		for z := max(c.zMin, -50); z <= min(c.zMax, 50); z += 1 {
			for y := max(c.yMin, -50); y <= min(c.yMax, 50); y += 1 {
				for x := max(c.xMin, -50); x <= min(c.xMax, 50); x += 1 {
					if c.on {
						space.Add(vec3{x, y, z})
					} else {
						space.Remove(vec3{x, y, z})
					}
				}
			}
		}
	}

	return space.Len(), nil
}

func part2(ctx context.Context, input []cube) (interface{}, error) {
	space := set.New[vec3]()

	// Nope
	//for _, c := range input {
	//	for z := c.zMin; z <= c.zMax; z += 1 {
	//		for y := c.yMin; y <= c.yMax; y += 1 {
	//			for x := c.xMin; x <= c.xMax; x += 1 {
	//				if c.on {
	//					space.Add(vec3{x, y, z})
	//				} else {
	//					space.Remove(vec3{x, y, z})
	//				}
	//			}
	//		}
	//	}
	//}

	return space.Len(), nil
}
