package main

import (
	"fmt"
	"github.com/koenaad/Advent-of-Code-2017/day17/circular"
)

const input = 356

func main() {
	fmt.Println("Advent of Code 2017 - day 17")

	fmt.Printf("Puzzle 1: value after 2017 = %v\n", spinlockAlgorithmFindValueAfter2017(input))
	fmt.Printf("Puzzle 2: value after 0 = %v\n", spinlockAlgorithmFindValueAfter0(input, 50000000))
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
