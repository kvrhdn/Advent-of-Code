package knotHash

import (
	"fmt"
	"strings"
)

func CreateNumbersUpTo(lastNumber int) []int {
	numbers := make([]int, lastNumber+1)

	for i := 0; i < lastNumber+1; i++ {
		numbers[i] = i
	}

	return numbers
}

func KnotHashRound(numbers []int, lengths []int, startPosition int, initialSkipSize int) (newPosition, newSkipSize int) {
	position := startPosition
	skipSize := initialSkipSize

	for _, length := range lengths {
		lowerIndex := position
		upperIndex := position + length - 1

		for {
			swapUsingWrappingIndex(numbers, lowerIndex, upperIndex)

			lowerIndex += 1
			upperIndex -= 1

			if lowerIndex >= upperIndex {
				break
			}
		}

		position += length + skipSize
		skipSize += 1
	}

	return position, skipSize
}

func swapUsingWrappingIndex(list []int, index1, index2 int) {
	wrappedIndex1 := index1 % len(list)
	wrappedIndex2 := index2 % len(list)

	list[wrappedIndex1], list[wrappedIndex2] = list[wrappedIndex2], list[wrappedIndex1]
}

func DenseKnotHash(input string) string {
	lengths := createLengthsFromInput(input)

	numbers := CreateNumbersUpTo(255)

	position := 0
	skipSize := 0

	for i := 0; i < 64; i++ {
		position, skipSize = KnotHashRound(numbers, lengths, position, skipSize)
	}

	denseHash := createDenseHash(numbers)

	return hashToString(denseHash)
}

func createLengthsFromInput(input string) []int {
	lengths := make([]int, len(input))

	for i, c := range []rune(input) {
		lengths[i] = int(c)
	}

	lengths = append(lengths, 17, 31, 73, 47, 23)

	return lengths
}

func createDenseHash(sparseHash []int) []int {
	if len(sparseHash)%16 != 0 {
		panic("sparse hash should contain a multiple of 16 elements")
	}

	denseHash := make([]int, len(sparseHash)/16)

	for i := 0; i < len(sparseHash)/16; i++ {
		offset := i * 16
		accumulator := sparseHash[i*16]

		for _, value := range sparseHash[offset+1 : offset+16] {
			accumulator ^= value
		}

		denseHash[i] = accumulator
	}

	return denseHash
}

func hashToString(hash []int) string {
	var builder strings.Builder

	for _, value := range hash {
		builder.WriteString(fmt.Sprintf("%02x", value))
	}

	return builder.String()
}
