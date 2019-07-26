package main

import "testing"

func TestSumOfDigitsThatMatchNext(t *testing.T) {
	cases := []struct {
		in       string
		expected int
	}{
		{"1122", 3},
		{"1111", 4},
		{"1234", 0},
		{"91212129", 9},
	}
	for _, c := range cases {
		got := SumOfDigitsThatMatchNext(c.in)
		if got != c.expected {
			t.Errorf("SumOfDigitsThatMatchNext(%v) = %v, but expected %v", c.in, got, c.expected)
		}
	}
}

func TestSumOfDigitsThatMatchHalfway(t *testing.T) {
	cases := []struct {
		in       string
		expected int
	}{
		{"1212", 6},
		{"1221", 0},
		{"123425", 4},
		{"123123", 12},
		{"12131415", 4},
	}
	for _, c := range cases {
		got := SumOfDigitsThatMatchHalfway(c.in)
		if got != c.expected {
			t.Errorf("SumOfDigitsThatMatchHalfway(%v) = %v, but expected %v", c.in, got, c.expected)
		}
	}
}
