package stack

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestStack(t *testing.T) {
	stack := New[int]()

	assert.True(t, stack.IsEmpty())

	stack.Push(1)
	stack.Push(2)
	stack.Push(3)

	assert.False(t, stack.IsEmpty())

	assert.Equal(t, 3, stack.Pop())
	assert.False(t, stack.IsEmpty())

	assert.Equal(t, 2, stack.Pop())
	assert.False(t, stack.IsEmpty())

	assert.Equal(t, 1, stack.Pop())

	assert.True(t, stack.IsEmpty())
}
