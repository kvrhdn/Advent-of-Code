package day16

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func Test_hexToBinary(t *testing.T) {
	testCases := []struct {
		hex      string
		expected string
	}{
		{"D2FE28", "110100101111111000101000"},
		{"38006F45291200", "00111000000000000110111101000101001010010001001000000000"},
		{"EE00D40C823060", "11101110000000001101010000001100100000100011000001100000"},
	}
	for _, tt := range testCases {
		t.Run(tt.hex, func(t *testing.T) {
			assert.Equal(t, tt.expected, hexToBinary(tt.hex))
		})
	}

	assert.Equal(t, "110100101111111000101000", hexToBinary("D2FE28"))
}

func Test_parseBinary(t *testing.T) {
	testCases := []struct {
		binary   string
		expected int
	}{
		{"110", 6},
		{"100", 4},
	}
	for _, tt := range testCases {
		t.Run(tt.binary, func(t *testing.T) {
			assert.Equal(t, tt.expected, parseBinary(tt.binary))
		})
	}
}
