package util

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestTimes(t *testing.T) {
	count := 0

	Times(5, func() {
		count += 1
	})

	assert.Equal(t, 5, count)
}
