package day09

import (
	"reflect"
	"testing"
)

func Test_processTheGarbageStream_score(t *testing.T) {
	var cases = []struct {
		in       string
		expected int
	}{
		{"{}", 1},
		{"{{{}}}", 6},
		{"{{},{}}", 5},
		{"{{{},{},{{}}}}", 16},
		{"{<a>,<a>,<a>,<a>}", 1},
		{"{{<ab>},{<ab>},{<ab>},{<ab>}}", 9},
		{"{{<!!>},{<!!>},{<!!>},{<!!>}}", 9},
		{"{{<a!>},{<a!>},{<a!>},{<ab>}}", 3},
		{"{<{}>}", 1},
	}
	for _, c := range cases {
		got, _ := processTheGarbageStream(c.in)
		if !reflect.DeepEqual(got, c.expected) {
			t.Errorf("processTheGarbageStream(%q) = %v, _, but expected %v, _", c.in, got, c.expected)
		}
	}
}

func Test_processTheGarbageStream_garbageCount(t *testing.T) {
	var cases = []struct {
		in       string
		expected int
	}{
		{`<>`, 0},
		{`<random characters>`, 17},
		{`<<<<>`, 3},
		{`<{!>}>`, 2},
		{`<!!>`, 0},
		{`<!!!>>`, 0},
		{`<{o"i!a,<{i<a>`, 10},
	}
	for _, c := range cases {
		_, got := processTheGarbageStream(c.in)
		if !reflect.DeepEqual(got, c.expected) {
			t.Errorf("processTheGarbageStream(%q) = _, %v, but expected _, %v", c.in, got, c.expected)
		}
	}
}
