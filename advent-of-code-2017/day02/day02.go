package day02

import (
	"strconv"
	"strings"
)

func SolvePart1(input string) interface{} {
	return CalculateSumOfRanges(input)
}

func SolvePart2(input string) interface{} {
	return CalculateSumOfEvenlyDivisibleResults(input)
}

func CalculateSumOfRanges(input string) interface{} {
	values := Day02InputToInts(input)

	sum := 0

	for _, line := range values {
		sum += RangeOf(line)
	}

	return sum
}

func CalculateSumOfEvenlyDivisibleResults(input string) interface{} {
	values := Day02InputToInts(input)

	sum := 0

	for _, line := range values {
		sum += EvenlyDivisibleResultOf(line)
	}

	return sum
}

func Day02InputToInts(input string) (output [][]int) {
	lines := strings.Split(input, "\n")

	for _, line := range lines {
		words := strings.Fields(line)

		var values []int

		for i := 0; i < len(words); i++ {
			value, err := strconv.Atoi(words[i])
			if err != nil {
				panic(err)
			}
			values = append(values, value)
		}

		output = append(output, values)
	}

	return
}

func RangeOf(values []int) int {
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

func EvenlyDivisibleResultOf(values []int) int {
	for i, value1 := range values {
		for _, value2 := range values[i+1:] {

			if isEvenlyDivisible(value1, value2) {
				return value1 / value2
			}
			if isEvenlyDivisible(value2, value1) {
				return value2 / value1
			}
		}
	}

	panic("could not find any evenly divisible pair")
}

func isEvenlyDivisible(value1, value2 int) bool {
	return value1%value2 == 0
}
