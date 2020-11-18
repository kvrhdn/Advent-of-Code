package day17

import (
	"strconv"

	"github.com/kvrhdn/advent-of-code/advent-of-code-2017/day17/circular"
)

func SolvePart1(inputString string) interface{} {
	input, err := strconv.Atoi(inputString)
	if err != nil {
		panic(err)
	}

	return spinlockAlgorithmFindValueAfter2017(input)
}

func SolvePart2(inputString string) interface{} {
	input, err := strconv.Atoi(inputString)
	if err != nil {
		panic(err)
	}

	return spinlockAlgorithmFindValueAfter0(input, 50000000)
}

func spinlockAlgorithmFindValueAfter2017(steps int) (valueAfter2017 int) {
	buffer := circular.NewBuffer()

	index := 0
	value := 0

	for {
		index = buffer.InsertAfter(index+steps, value)

		if value == 2017 {
			return buffer.Get(index + 1)
		}

		value += 1
	}
}

func spinlockAlgorithmFindValueAfter0(steps int, stopAt int) (valueAfter0 int) {
	// simulation buffer
	bufferLength := 1
	value := 1

	index := 0

	for {
		// 'insert' at new index
		index = (index + steps) % bufferLength
		index += 1
		bufferLength += 1

		// if inserted after 0
		if index == 1 {
			valueAfter0 = value
		}

		value += 1

		if value > stopAt {
			return
		}
	}
}
