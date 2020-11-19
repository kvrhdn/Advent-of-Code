package day02

import (
	"strconv"
	"strings"
)

func SolvePart1(input string) interface{} {
	values := parseInput(input)

	sum := 0
	for _, line := range values {
		sum += rangeOf(line)
	}
	return sum
}

func SolvePart2(input string) interface{} {
	values := parseInput(input)

	sum := 0
	for _, line := range values {
		sum += resultOfEvenlyDivisibleNumber(line)
	}
	return sum
}

func parseInput(input string) (output [][]int) {
	lines := strings.Split(input, "\n")

	for _, line := range lines {
		numbers := strings.Fields(line)

		var values []int

		for i := 0; i < len(numbers); i++ {
			v, err := strconv.Atoi(numbers[i])
			if err != nil {
				panic(err)
			}
			values = append(values, v)
		}

		output = append(output, values)
	}

	return
}

func rangeOf(values []int) int {
	min := values[0]
	max := values[0]

	for _, value := range values {
		if value < min {
			min = value
		}
		if value > max {
			max = value
		}
	}

	return max - min
}

func resultOfEvenlyDivisibleNumber(values []int) int {
	for i, value1 := range values {
		for _, value2 := range values[i+1:] {
			if value1%value2 == 0 {
				return value1 / value2
			}
			if value2%value1 == 0 {
				return value2 / value1
			}
		}
	}
	panic("Could not find any evenly divisible pair")
}
