package linkedlist

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestNode(t *testing.T) {
	head := New[string]("0")

	assert.Nil(t, head.Next())

	nodeHalf := head.InsertAfter("1")
	assert.Equal(t, "1", nodeHalf.Value())

	nodeHalf = head.InsertAfter("0.5")
	assert.Equal(t, "0.5", nodeHalf.Value())
	assert.Equal(t, "1", nodeHalf.Next().Value())
}
