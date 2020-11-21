package day08

import "strings"

func SolvePart1(input string) interface{} {
	instructions := parseInput(input)

	memory := make(map[string]int)

	for _, ir := range instructions {
		ir.execute(memory)
	}

	largest := 0

	for _, value := range memory {
		if value > largest {
			largest = value
		}
	}

	return largest
}

func SolvePart2(input string) interface{} {
	instructions := parseInput(input)

	memory := make(map[string]int)

	largest := 0

	for _, ir := range instructions {
		ir.execute(memory)

		for _, value := range memory {
			if value > largest {
				largest = value
			}
		}
	}

	return largest
}

func parseInput(input string) (instructions []Instruction) {
	for _, line := range strings.Split(input, "\n") {
		instructions = append(instructions, parseInstruction(line))
	}
	return
}
