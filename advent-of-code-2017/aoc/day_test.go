package aoc

import (
	"testing"
	"time"

	"github.com/stretchr/testify/assert"
)

func TestSolveTimed(t *testing.T) {
	var captureInput string

	solver := func(input string) interface{} {
		captureInput = input
		time.Sleep(200 * time.Millisecond)
		return "my-solution"
	}

	result, elapsed := solveTimed(solver, "my-input")

	assert.Equal(t, "my-input", captureInput)
	assert.Equal(t, "my-solution", result)
	assert.InDelta(t, 200*time.Millisecond, elapsed, float64(10*time.Millisecond))
}
