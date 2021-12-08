package day08

import (
	"context"
	"fmt"
	"strings"

	"github.com/kvrhdn/advent-of-code/advent-of-code-2021/aoc"
	"github.com/kvrhdn/advent-of-code/advent-of-code-2021/shared/slices"
	aoc_strings "github.com/kvrhdn/advent-of-code/advent-of-code-2021/shared/strings"
)

var Solution = aoc.NewDayGen(generator, part1, part2)

type entry struct {
	signalPatterns []string
	outputs        []string
}

func parseEntry(s string) (entry, error) {
	splitLine := strings.Split(s, " | ")
	if len(splitLine) != 2 {
		return entry{}, fmt.Errorf("expected line to be split in 2, got: %v", splitLine)
	}

	return entry{
		signalPatterns: strings.Fields(splitLine[0]),
		outputs:        strings.Fields(splitLine[1]),
	}, nil
}

func (e *entry) outputsWithUniqueSegmentCount() int {
	count := 0
	for _, output := range e.outputs {
		segments := len(output)
		if segments == 2 || segments == 3 || segments == 4 || segments == 7 {
			count += 1
		}
	}
	return count
}

const (
	allSegments = "abcdefg"

	zero  = "abcefg"  // len: 6
	one   = "cf"      // len: 2 (unique)
	two   = "acdeg"   // len: 5
	three = "acdfg"   // len: 5
	four  = "bcdf"    // len: 4 (unique)
	five  = "abdfg"   // len: 5
	six   = "abdefg"  // len: 6
	seven = "acf"     // len: 3 (unique)
	eight = "abcdefg" // len: 7 (unique)
	nine  = "abcdfg"  // len: 6
)

func (e *entry) decodeOutput() int {
	// map with signal wire -> segments they could map to
	signalWireToSegmentMapping := make(map[rune]string)

	// start with every signal wire mapped to every segment
	for _, segment := range allSegments {
		signalWireToSegmentMapping[segment] = allSegments
	}

	// function to update signalWireToSegmentMapping when we find a match
	signalPatternIsConnectedTo := func(signalPattern, segments string) {
		for _, signal := range allSegments {
			mapping := signalWireToSegmentMapping[signal]

			if strings.Contains(signalPattern, string(signal)) {
				// if this signal is part of the pattern, it must connect to a segment of this number
				mapping = aoc_strings.KeepRunes(mapping, segments)
			} else {
				// else it must not connect to a segment of this number
				mapping = aoc_strings.RemoveRunes(mapping, segments)
			}

			signalWireToSegmentMapping[signal] = mapping
		}
	}

	signalPatterns := e.signalPatterns
	var sixPattern, sevenPattern string // once we know 6 and 7, use this to filter some more

	for {
		var unusedSignalPatterns []string

		for _, signalPattern := range signalPatterns {
			switch len(signalPattern) {
			case 2: // 1
				signalPatternIsConnectedTo(signalPattern, one)
			case 3: // 7
				signalPatternIsConnectedTo(signalPattern, seven)
				sevenPattern = signalPattern
			case 4: // 4
				signalPatternIsConnectedTo(signalPattern, four)
			case 5: // 2, 3, 5
				if sixPattern == "" || sevenPattern == "" {
					// wait with processing until we know 6 and 7
					unusedSignalPatterns = append(unusedSignalPatterns, signalPattern)
					continue
				}

				commonWithSeven := len(aoc_strings.KeepRunes(signalPattern, sevenPattern))

				if commonWithSeven == 3 { // 3
					signalPatternIsConnectedTo(signalPattern, three)
				} else {
					commonWithSix := len(aoc_strings.KeepRunes(signalPattern, sixPattern))

					if commonWithSix == 5 { // 5
						signalPatternIsConnectedTo(signalPattern, five)
					} else { // 2
						signalPatternIsConnectedTo(signalPattern, two)
					}
				}
			case 6: // 0, 6, 9
				if sevenPattern == "" {
					// wait with processing until we know 7
					unusedSignalPatterns = append(unusedSignalPatterns, signalPattern)
					continue
				}

				commonWithSeven := len(aoc_strings.KeepRunes(signalPattern, sevenPattern))

				if commonWithSeven != 3 { // 6
					signalPatternIsConnectedTo(signalPattern, six)
					sixPattern = signalPattern
				}
				// discard 0, 9
			case 7: // 8
				// discard 8, contains no useful information
			}
		}

		signalPatterns = unusedSignalPatterns

		// we are done if all segments map to one segment
		count := 0
		for _, to := range signalWireToSegmentMapping {
			count += len(to)
		}
		if count == 7 {
			break
		}
	}

	value := 0
	for _, output := range e.outputs {
		output = strings.Map(func(r rune) rune {
			return []rune(signalWireToSegmentMapping[r])[0]
		}, output)

		output = aoc_strings.Sort(output)

		value *= 10 // shift everything one digit left

		switch output { // lolsob
		case zero:
			value += 0
		case one:
			value += 1
		case two:
			value += 2
		case three:
			value += 3
		case four:
			value += 4
		case five:
			value += 5
		case six:
			value += 6
		case seven:
			value += 7
		case eight:
			value += 8
		case nine:
			value += 9
		}
	}

	return value
}

func generator(ctx context.Context, input string) ([]entry, error) {
	var entries []entry

	for _, line := range strings.Split(input, "\n") {
		entry, err := parseEntry(line)
		if err != nil {
			return nil, err
		}
		entries = append(entries, entry)
	}

	return entries, nil
}

func part1(ctx context.Context, input []entry) (interface{}, error) {
	return slices.Sum(input, func(e entry) int {
		return e.outputsWithUniqueSegmentCount()
	}), nil
}

func part2(ctx context.Context, input []entry) (interface{}, error) {
	return slices.Sum(input, func(e entry) int {
		return e.decodeOutput()
	}), nil
}
