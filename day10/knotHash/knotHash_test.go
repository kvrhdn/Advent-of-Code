package knotHash

import (
	"github.com/koenaad/Advent-of-Code-2017/util"
	"testing"
)

func TestCreateNumbersUpTo(t *testing.T) {
	numbers := CreateNumbersUpTo(4)

	expected := []int{0, 1, 2, 3, 4}

	if !util.SliceEquals(numbers, expected) {
		t.Errorf("CreateNumbersUpTo(5): result is %v, but expected %v", numbers, expected)
	}
}

func TestKnotHashRound(t *testing.T) {
	numbers := CreateNumbersUpTo(4)
	lenghts := []int{3, 4, 1, 5}

	KnotHashRound(numbers, lenghts, 0, 0)

	expected := []int{3, 4, 2, 1, 0}

	if !util.SliceEquals(numbers, expected) {
		t.Errorf("hashed result is %v, but expected %v", numbers, expected)
	}
}

func TestDenseKnotHash(t *testing.T) {
	cases := []struct {
		in, expected string
	}{
		{"", "a2582a3a0e66e6e86e3812dcb672a272"},
		{"AoC 2017", "33efeb34ea91902bb2f59c9920caa6cd"},
		{"1,2,3", "3efbe78a8d82f29979031a4aa0b16a9d"},
		{"1,2,4", "63960835bcdc130f0b66d7ff4f6a5a8e"},
	}
	for _, c := range cases {
		got := DenseKnotHash(c.in)
		if got != c.expected {
			t.Errorf("DenseKnotHash(%q): got %q, but expected %q", c.in, got, c.expected)
		}
	}
}

func Test_createLengthsFromInput(t *testing.T) {
	in := "1,2,3"
	expected := []int{49, 44, 50, 44, 51, 17, 31, 73, 47, 23}

	got := createLengthsFromInput(in)
	if !util.SliceEquals(expected, got) {
		t.Errorf("createLengthsFromInput(%q): got %v, but expected %v", in, got, expected)
	}
}

func Test_createDenseHash(t *testing.T) {
	in := []int{65, 27, 9, 1, 4, 3, 40, 50, 91, 7, 6, 0, 2, 5, 68, 22}
	expected := []int{64}

	got := createDenseHash(in)
	if !util.SliceEquals(expected, got) {
		t.Errorf("createDenseHash(%v): got %v, but expected %v", in, got, expected)
	}
}

func Test_hashToString(t *testing.T) {
	in := []int{64, 7, 255}
	expected := "4007ff"

	got := hashToString(in)
	if expected != got {
		t.Errorf("hashToString(%v): got %q, but expected %q", in, got, expected)
	}
}
