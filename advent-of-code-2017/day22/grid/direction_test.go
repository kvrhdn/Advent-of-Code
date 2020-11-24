package grid

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestLeftOf(t *testing.T) {
	assert.Equal(t, West, LeftOf(North))
	assert.Equal(t, North, LeftOf(East))
	assert.Equal(t, East, LeftOf(South))
	assert.Equal(t, South, LeftOf(West))
}

func TestRightOf(t *testing.T) {
	assert.Equal(t, East, RightOf(North))
	assert.Equal(t, South, RightOf(East))
	assert.Equal(t, West, RightOf(South))
	assert.Equal(t, North, RightOf(West))
}

func TestReverseOf(t *testing.T) {
	assert.Equal(t, South, ReverseOf(North))
	assert.Equal(t, West, ReverseOf(East))
	assert.Equal(t, North, ReverseOf(South))
	assert.Equal(t, East, ReverseOf(West))
}
