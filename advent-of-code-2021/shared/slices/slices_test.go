package slices

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestFilter(t *testing.T) {
	outputInt := Filter([]int{1, 2, 3, 4, 5}, func(i int) bool {
		return i%2 == 0
	})
	assert.Equal(t, []int{2, 4}, outputInt)

	outputString := Filter([]string{"foo", "bar", "bzz"}, func(s string) bool {
		return s[0] == 'b'
	})
	assert.Equal(t, []string{"bar", "bzz"}, outputString)
}
