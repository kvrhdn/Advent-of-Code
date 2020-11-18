package main

import (
	"reflect"
	"testing"
)

func TestCalculateSumOfRanges(t *testing.T) {
	in := "5 1 9 5\n7 5 3\n2 4 6 8"
	expected := 18

	got := CalculateSumOfRanges(in)
	if got != expected {
		t.Errorf("CalculateSumOfRanges(%v) = %v, but expected %v", in, got, expected)
	}
}

func TestCalculateSumOfEvenlyDivisibleResults(t *testing.T) {
	in := "5 9 2 8\n9 4 7 3\n3 8 6 5"
	expected := 9

	got := CalculateSumOfEvenlyDivisibleResults(in)
	if got != expected {
		t.Errorf("CalculateSumOfEvenlyDivisibleResults((%v) = %v, but expected %v", in, got, expected)
	}
}

func TestDay02InputToInts(t *testing.T) {
	cases := []struct {
		in       string
		expected [][]int
	}{
		{"1 2\n3 4", asSlice([]int{1, 2}, []int{3, 4})},
		{"321\n3312 41231 234342", asSlice([]int{321}, []int{3312, 41231, 234342})},
	}
	for _, c := range cases {
		got := Day02InputToInts(c.in)
		if !reflect.DeepEqual(got, c.expected) {
			t.Errorf("Day02InputToInts(%v) = %v, but expected %v", c.in, got, c.expected)
		}
	}
}

func asSlice(slices ...[]int) (sliceOfSlices [][]int) {
	for _, slice := range slices {
		sliceOfSlices = append(sliceOfSlices, slice)
	}
	return
}

func TestRangeOf(t *testing.T) {
	cases := []struct {
		in       []int
		expected int
	}{
		{[]int{1, 2, 3}, 2},
		{[]int{4, 1, 3}, 3},
	}
	for _, c := range cases {
		got := RangeOf(c.in)
		if got != c.expected {
			t.Errorf("RangeOf(%v) = %v, but expected %v", c.in, got, c.expected)
		}
	}
}

func TestEvenlyDivisbleResult(t *testing.T) {
	cases := []struct {
		in       []int
		expected int
	}{
		{[]int{5, 9, 2, 8}, 4},
		{[]int{9, 4, 7, 3}, 3},
		{[]int{3, 8, 6, 5}, 2},
	}
	for _, c := range cases {
		got := EvenlyDivisibleResultOf(c.in)
		if got != c.expected {
			t.Errorf("EvenlyDivisibleResultOf(%v) = %v, but expected %v", c.in, got, c.expected)
		}
	}
}
