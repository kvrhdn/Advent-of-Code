package day01

import (
	"strconv"
)

func SolvePart1(input string) interface{} {
	return SumOfDigitsThatMatchNext(input)
}

func SolvePart2(input string) interface{} {
	return SumOfDigitsThatMatchHalfway(input)
}

func SumOfDigitsThatMatchNext(input string) interface{} {
	digits := splitIntoDigits(input)

	sum := sumOfDigitsThatMatchWithOffset(digits, 1)

	return sum
}

func SumOfDigitsThatMatchHalfway(input string) interface{} {
	digits := splitIntoDigits(input)

	sum := sumOfDigitsThatMatchWithOffset(digits, len(digits)/2)

	return sum
}

func sumOfDigitsThatMatchWithOffset(digits []int, offset int) int {
	sum := 0

	for i := 0; i < len(digits); i++ {
		j := (i + offset) % len(digits)

		if digits[i] == digits[j] {
			sum += digits[i]
		}
	}

	return sum
}

func splitIntoDigits(input string) (digits []int) {
	runes := []rune(input)

	for _, rune := range runes {
		digit, err := strconv.Atoi(string(rune))

		if err != nil {
			panic(err)
		}

		digits = append(digits, digit)
	}

	return
}
