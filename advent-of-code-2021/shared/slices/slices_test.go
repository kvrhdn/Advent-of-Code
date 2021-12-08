package slices

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestCopy(t *testing.T) {
	originalSlice := []int{1, 2, 3}

	newSlice := Copy(originalSlice)
	assert.Equal(t, originalSlice, newSlice)

	newSlice[0] += 5
	assert.Equal(t, 1, originalSlice[0])

	originalSlice[2] += 5
	assert.Equal(t, 3, newSlice[2])
}

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

func TestSum(t *testing.T) {
	assert.Equal(t, 6, Sum([]int{1, 2, 3}, func(i int) int {
		return i
	}))

	assert.Equal(t, 6, Sum([]string{"a", "ab", "abc"}, func(s string) int {
		return len(s)
	}))
}
