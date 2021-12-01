package day01

import (
	"context"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestSolution(t *testing.T) {
	Solution.VerifySolution(t, 1, "1184", "1158")
}

func TestExample(t *testing.T) {
	ctx := context.Background()
	example := `199
200
208
210
200
207
240
269
260
263`

	gen, err := generator(ctx, example)
	assert.NoError(t, err)

	output1, err := part1(ctx, gen)
	assert.NoError(t, err)
	assert.Equal(t, "7", output1)

	output2, err := part2(ctx, gen)
	assert.NoError(t, err)
	assert.Equal(t, "5", output2)
}
