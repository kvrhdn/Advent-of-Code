package set

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestSet(t *testing.T) {
	set := New[string]()

	assert.False(t, set.Contains("foo"))
	assert.False(t, set.Contains("bar"))

	set.Add("foo")

	assert.True(t, set.Contains("foo"))
	assert.False(t, set.Contains("bar"))
}

func TestSet_Copy(t *testing.T) {
	set := New[string]()

	set.Add("foo")

	assert.True(t, set.Contains("foo"))

	setCopy := set.Copy()

	assert.True(t, setCopy.Contains("foo"))

	set.Add("bar")

	assert.True(t, set.Contains("bar"))
	assert.False(t, setCopy.Contains("bar"))

	setCopy.Add("bzz")

	assert.False(t, set.Contains("bzz"))
	assert.True(t, setCopy.Contains("bzz"))
}
