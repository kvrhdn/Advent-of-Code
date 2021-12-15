package priorityqueue

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestPriorityQueue(t *testing.T) {
	queue := New[string]()

	assert.True(t, queue.IsEmpty())

	queue.Push(1, "one")

	assert.False(t, queue.IsEmpty())

	assert.Equal(t, "one", queue.Pop())
	assert.True(t, queue.IsEmpty())

	queue.Push(3, "three")
	queue.Push(4, "four")
	queue.Push(2, "two")

	assert.Equal(t, "two", queue.Pop())
	assert.Equal(t, "three", queue.Pop())
	assert.Equal(t, "four", queue.Pop())
}
