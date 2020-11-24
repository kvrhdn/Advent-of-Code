package day24

import (
	"strings"
)

func SolvePart1(input string) interface{} {
	components := parseInput(input)

	bridges := buildAllPossibleBridges(nil, 0, components)

	strongest := 0

	for _, b := range bridges {
		strength := b.strength()

		if strength > strongest {
			strongest = strength
		}
	}

	return strongest
}

func SolvePart2(input string) interface{} {
	components := parseInput(input)

	bridges := buildAllPossibleBridges(nil, 0, components)

	longest := 0
	strongest := 0

	for _, bridge := range bridges {
		length := bridge.length()
		strength := bridge.strength()

		if length > longest {
			longest = length
			strongest = strength
			continue
		}
		if length == longest && strength > strongest {
			strongest = strength
			continue
		}
	}

	return strongest
}

func parseInput(input string) (c []Component) {
	for i, line := range strings.Split(input, "\n") {
		c = append(c, parseComponent(i, line))
	}
	return
}
