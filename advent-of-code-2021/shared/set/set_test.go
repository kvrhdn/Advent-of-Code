package set

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestSet(t *testing.T) {
	set := New[string]()

	assert.False(t, set.Contains("foo"))
	assert.False(t, set.Contains("bar"))
	assert.Equal(t, 0, set.Len())

	set.Add("foo")

	assert.True(t, set.Contains("foo"))
	assert.False(t, set.Contains("bar"))
	assert.Equal(t, 1, set.Len())

	set.Remove("bar")

	assert.True(t, set.Contains("foo"))
	assert.False(t, set.Contains("bar"))
	assert.Equal(t, 1, set.Len())

	set.Remove("foo")

	assert.False(t, set.Contains("foo"))
	assert.False(t, set.Contains("bar"))
	assert.Equal(t, 0, set.Len())
}

func TestSet_Range(t *testing.T) {
	set := New[string]()

	set.Add("foo")
	set.Add("bar")
	set.Add("bzz")
	set.Remove("bar")

	var visited []string
	for _, value := range set.Values() {
		visited = append(visited, value)
	}

	assert.ElementsMatch(t, []string{"foo", "bzz"}, visited)
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
