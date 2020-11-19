package day01

func SolvePart1(input string) interface{} {
	digits := parseInput(input)
	return sumOfDigitsThatMatchWithOffset(digits, 1)
}

func SolvePart2(input string) interface{} {
	digits := parseInput(input)
	return sumOfDigitsThatMatchWithOffset(digits, len(digits)/2)
}

func parseInput(input string) (digits []int) {
	for _, r := range []rune(input) {
		digits = append(digits, int(r-'0'))
	}
	return
}

func sumOfDigitsThatMatchWithOffset(digits []int, offset int) int {
	sum := 0

	for i := range digits {
		j := (i + offset) % len(digits)

		if digits[i] == digits[j] {
			sum += digits[i]
		}
	}

	return sum
}
