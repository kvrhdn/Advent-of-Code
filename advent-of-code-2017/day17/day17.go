package day17

import (
	"strconv"

	"github.com/kvrhdn/advent-of-code/advent-of-code-2017/day17/circular"
)

func SolvePart1(input string) interface{} {
	steps, err := strconv.Atoi(input)
	if err != nil {
		panic(err)
	}

	buffer := circular.New([]int{0})

	index := 0

	for value := 1; value <= 2017; value++ {
		index = buffer.InsertAfter(index+steps, value)
	}

	return buffer.Get(index + 1)
}

func SolvePart2(input string) interface{} {
	steps, err := strconv.Atoi(input)
	if err != nil {
		panic(err)
	}

	return spinlockAlgorithmFindValueAfter0(steps, 50000000)
}

func spinlockAlgorithmFindValueAfter0(steps int, stopAt int) (valueAfter0 int) {
	// simulate the circular buffer
	bufferLength := 1

	index := 0

	for value := 1; value <= stopAt; value++ {
		// 'insert' at new index
		index = (index + steps) % bufferLength
		index += 1
		bufferLength += 1

		// track insertions after value 0
		if index == 1 {
			valueAfter0 = value
		}
	}

	return valueAfter0
}
