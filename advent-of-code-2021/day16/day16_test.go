package day16

import (
	"context"
	"testing"

	"github.com/stretchr/testify/assert"

	"github.com/kvrhdn/advent-of-code/advent-of-code-2021/aoc"
)

func TestSolution(t *testing.T) {
	Solution.VerifySolution(t, 16, 1007, int64(834151779165))
}

func TestExample_part1(t *testing.T) {
	// only test part 1
	solutionPart1 := aoc.NewDay(part1, nil)

	examples := []struct {
		transmission string
		versionSum   int
	}{
		{"8A004A801A8002F478", 16},
		{"620080001611562C8802118E34", 12},
		{"C0015000016115A2E0802F182340", 23},
		{"A0016C880162017C3686B18A3D4780", 31},
	}

	for _, example := range examples {
		t.Run(example.transmission, func(t *testing.T) {
			solutionPart1.VerifyInput(t, example.transmission, example.versionSum, 0)
		})
	}
}

func TestExample_part2(t *testing.T) {
	// only test part 2
	solutionPart2 := aoc.NewDay(nil, part2)

	examples := []struct {
		transmission string
		value        int64
	}{
		{"C200B40A82", 3},
		{"04005AC33890", 54},
		{"880086C3E88112", 7},
		{"CE00C43D881120", 9},
		{"D8005AC2A8F0", 1},
		{"F600BC2D8F", 0},
		{"9C005AC2F8F0", 0},
		{"9C0141080250320F1802104A08", 1},
	}

	for _, example := range examples {
		t.Run(example.transmission, func(t *testing.T) {
			solutionPart2.VerifyInput(t, example.transmission, 0, example.value)
		})
	}
}

func Test_decodeTransmission(t *testing.T) {
	testCases := []struct {
		transmission string
		expected     packet
	}{
		{
			"D2FE28",
			packet{
				version: 6,
				typeID:  4,
				literal: 2021,
			},
		},
		{
			"38006F45291200",
			packet{
				version:      1,
				typeID:       6,
				lengthTypeID: 0,
				totalLength:  27,
				subPackets: []packet{
					{
						version: 6,
						typeID:  4,
						literal: 10,
					},
					{
						version: 2,
						typeID:  4,
						literal: 20,
					},
				},
			},
		},
		{
			"EE00D40C823060",
			packet{
				version:         7,
				typeID:          3,
				lengthTypeID:    1,
				numberOfPackets: 3,
				subPackets: []packet{
					{
						version: 2,
						typeID:  4,
						literal: 1,
					},
					{
						version: 4,
						typeID:  4,
						literal: 2,
					},
					{
						version: 1,
						typeID:  4,
						literal: 3,
					},
				},
			},
		},
	}
	for _, tt := range testCases {
		t.Run(tt.transmission, func(t *testing.T) {
			packets, _ := decodePackets(context.Background(), hexToBinary(tt.transmission))
			assert.Equal(t, tt.expected, packets)
		})
	}
}
