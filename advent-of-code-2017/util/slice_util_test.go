package util

import "testing"

func TestSliceEquals(t *testing.T) {
	cases := []struct {
		in1, in2 []int
		expected bool
	}{
		{[]int{1, 2, 3}, []int{1, 2, 3}, true},
		{[]int{1, 2, 3}, []int{1, 3}, false},
		{[]int{3, 2, -1}, []int{3, 2, 1}, false},
	}
	for _, c := range cases {
		got := SliceEquals(c.in1, c.in2)
		if got != c.expected {
			t.Errorf("SliceEquals(%v, %v): expected %v, but got %v", c.in1, c.in2, c.expected, got)
		}
	}
}

func TestSliceCopy(t *testing.T) {
	original := []int{1, 2, 3}

	copy := SliceCopy(original)

	copy[0] = 4;

	if original[0] != 1 {
		t.Errorf("SliceCopy: original slice was impacted by copy")
	}
}

func TestSliceAtoi_convert(t *testing.T) {
	in := []string{"123", "4", "-5", "234238"}
	expected := []int{123, 4, -5, 234238}

	got := SliceAtoi(in)

	if !SliceEquals(expected, got) {
		t.Errorf("SliceAtoi(%v): expected %v, but got %v", in, expected, got)
	}
}

func TestSliceAtoi_panic(t *testing.T) {
	assertPanic(t, func() {
		SliceAtoi([]string{"123", "a", "5"})
	})
}

func assertPanic(t *testing.T, f func()) {
	defer func() {
		if r := recover(); r == nil {
			t.Errorf("The code did not panic")
		}
	}()

	f()
}
