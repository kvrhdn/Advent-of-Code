package day07

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestParseProgram(t *testing.T) {
	var cases = []struct {
		input    string
		expected ProgramDescription
	}{
		{"pbga (66)", ProgramDescription{Name: "pbga", Weight: 66}},
		{"fwft (72) -> ktlj, cntj, xhth", ProgramDescription{Name: "fwft", Weight: 72, IsHolding: []string{"ktlj", "cntj", "xhth"}}},
	}
	for _, c := range cases {
		assert.Equal(t, c.expected, parseProgram(c.input), "parseProgram(%v)", c.input)
	}
}
