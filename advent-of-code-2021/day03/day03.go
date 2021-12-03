package day03

import (
	"context"
	"fmt"
	"strconv"
	"strings"

	"github.com/kvrhdn/advent-of-code/advent-of-code-2021/aoc"
	"github.com/kvrhdn/advent-of-code/advent-of-code-2021/shared/slices"
)

var Solution = aoc.NewDayGen(generator, part1, part2)

func generator(ctx context.Context, input string) ([]string, error) {
	return strings.Split(input, "\n"), nil
}

func part1(ctx context.Context, input []string) (interface{}, error) {
	// we assume every input number has the same length
	bits := len(input[0])

	gammaRateSlice := make([]rune, bits)
	epsilonRateSlice := make([]rune, bits)

	for i := 0; i < bits; i++ {
		zeroes, ones := countBit(input, i)
		if ones >= zeroes {
			gammaRateSlice[i] = '1'
			epsilonRateSlice[i] = '0'
		} else {
			gammaRateSlice[i] = '0'
			epsilonRateSlice[i] = '1'
		}
	}

	gammaRate, err := parseBinary(string(gammaRateSlice))
	if err != nil {
		return nil, err
	}
	epsilonRate, err := parseBinary(string(epsilonRateSlice))
	if err != nil {
		return nil, err
	}

	return int(gammaRate * epsilonRate), nil
}

func part2(ctx context.Context, input []string) (interface{}, error) {
	// we assume every input number has the same length
	bits := len(input[0])

	findCandidate := func(expectedValueIfMoreOrEqualOnes, expectedValueIfMoreZeroes rune) (int64, error) {
		candidates := input[:]

		for i := 0; i < bits; i++ {
			zeroes, ones := countBit(candidates, i)
			candidates = slices.Filter(candidates, func(s string) bool {
				if ones >= zeroes {
					return rune(s[i]) == expectedValueIfMoreOrEqualOnes
				} else {
					return rune(s[i]) == expectedValueIfMoreZeroes
				}
			})
			if len(candidates) == 1 {
				break
			}
		}

		if len(candidates) != 1 {
			return 0, fmt.Errorf("could not find a unique candidate: %v", candidates)
		}

		return parseBinary(candidates[0])
	}

	oxygenGeneratorRating, err := findCandidate('1', '0')
	if err != nil {
		return nil, err
	}
	co2ScrubberRating, err := findCandidate('0', '1')
	if err != nil {
		return nil, err
	}

	return int(oxygenGeneratorRating * co2ScrubberRating), nil
}

func countBit(input []string, position int) (zeroes, ones int) {
	for _, line := range input {
		switch line[position] {
		case '0':
			zeroes += 1
		case '1':
			ones += 1
		}
	}
	return
}

func parseBinary(line string) (int64, error) {
	return strconv.ParseInt(line, 2, 32)
}
