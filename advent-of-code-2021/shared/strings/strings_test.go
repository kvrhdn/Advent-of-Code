package strings

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestSort(t *testing.T) {
	tests := []struct {
		s        string
		expected string
	}{
		{"abc", "abc"},
		{"acb", "abc"},
		{"cbCB", "BCbc"},
	}
	for _, tt := range tests {
		t.Run(tt.s, func(t *testing.T) {
			assert.Equal(t, tt.expected, Sort(tt.s))
		})
	}
}

func TestRemoveRunes(t *testing.T) {
	tests := []struct {
		s        string
		toRemove string
		expected string
	}{
		{"abcba", "a", "bcb"},
		{"abcba", "c", "abba"},
		{"abcba", "bc", "aa"},
	}
	for _, tt := range tests {
		t.Run(tt.s, func(t *testing.T) {
			assert.Equal(t, tt.expected, RemoveRunes(tt.s, tt.toRemove))
		})
	}
}

func TestKeepRunes(t *testing.T) {
	tests := []struct {
		s        string
		toKeep   string
		expected string
	}{
		{"abcba", "a", "aa"},
		{"abcba", "c", "c"},
		{"abcba", "bc", "bcb"},
	}
	for _, tt := range tests {
		t.Run(tt.s, func(t *testing.T) {
			assert.Equal(t, tt.expected, KeepRunes(tt.s, tt.toKeep))
		})
	}
}
