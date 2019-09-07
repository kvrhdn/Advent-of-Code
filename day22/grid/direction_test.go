package grid

import "testing"

func TestLeftOf(t *testing.T) {
	t.Parallel()

	cases := []struct {
		in, expected Dir
	}{
		{North, West},
		{East, North},
		{South, East},
		{West, South},
	}

	for _, c := range cases {
		got := LeftOf(c.in)

		if got != c.expected {
			t.Errorf("LeftOf(%v) = %v, expected %v", c.in, got, c.expected)
		}
	}
}

func TestRightOf(t *testing.T) {
	t.Parallel()

	cases := []struct {
		in, expected Dir
	}{
		{North, East},
		{East, South},
		{South, West},
		{West, North},
	}

	for _, c := range cases {
		got := RightOf(c.in)

		if got != c.expected {
			t.Errorf("RightOf(%v) = %v, expected %v", c.in, got, c.expected)
		}
	}
}

func TestReverseOf(t *testing.T) {
	t.Parallel()

	cases := []struct {
		in, expected Dir
	}{
		{North, South},
		{East, West},
		{South, North},
		{West, East},
	}

	for _, c := range cases {
		got := ReverseOf(c.in)

		if got != c.expected {
			t.Errorf("ReverseOf(%v) = %v, expected %v", c.in, got, c.expected)
		}
	}
}
