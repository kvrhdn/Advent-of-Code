package day04

import (
	"testing"

	"github.com/kvrhdn/advent-of-code/advent-of-code-2017/aoc"
	"github.com/stretchr/testify/assert"
)

func TestExamplesPart1(t *testing.T) {
	cases := []struct {
		input    string
		expected bool
	}{
		{"aa bb cc dd ee", true},
		{"aa bb cc dd aa", false},
		{"aa bb cc dd aaa", true},
	}
	for _, c := range cases {
		assert.Equal(t, c.expected, validatePassphrase(c.input, validatorNoDuplicates), "validatePassphrase(%v, validatorNoDuplicates)", c.input)
	}
}

func TestExamplesPart2(t *testing.T) {
	cases := []struct {
		input    string
		expected bool
	}{
		{"abcde fghij", true},
		{"abcde xyz ecdab", false},
		{"a ab abc abd abf abj", true},
		{"iiii oiii ooii oooi oooo", true},
		{"oiii ioii iioi iiio", false},
	}
	for _, c := range cases {
		assert.Equal(t, c.expected, validatePassphrase(c.input, validatorNoAnagrams), "validatePassphrase(%v, validatorNoAnagrams)", c.input)
	}
}

func TestRealInput(t *testing.T) {
	input := aoc.ReadInputRelative(2017, 4, "../")

	assert.Equal(t, 325, SolvePart1(input))
	assert.Equal(t, 119, SolvePart2(input))
}
