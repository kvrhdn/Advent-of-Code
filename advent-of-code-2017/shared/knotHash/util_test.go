package knotHash

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestCreateSequence(t *testing.T) {
	assert.Equal(t, []int{0, 1, 2, 3, 4, 5, 6, 7}, createSequence(8))
}
