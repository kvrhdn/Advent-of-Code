package day16

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestParseDanceMove(t *testing.T) {
	cases := []struct {
		input    string
		expected DanceMove
	}{
		{"s1", Spin{1}},
		{"x3/4", Exchange{3, 4}},
		{"pe/b", Partner{'e', 'b'}},
	}
	for _, c := range cases {
		assert.Equal(t, c.expected, parseDanceMove(c.input))
	}
}

func TestSpinApply(t *testing.T) {
	input := []rune{'a', 'b', 'c', 'd', 'e'}

	spin := Spin{1}
	spin.apply(&input)

	assert.Equal(t, []rune{'e', 'a', 'b', 'c', 'd'}, input)
}

func TestExchangeApply(t *testing.T) {
	input := []rune{'a', 'b', 'c', 'd', 'e'}

	exchange := Exchange{1, 3}
	exchange.apply(&input)

	assert.Equal(t, []rune{'a', 'd', 'c', 'b', 'e'}, input)
}

func TestPartnerApply(t *testing.T) {
	input := []rune{'a', 'b', 'c', 'd', 'e'}

	partner := Partner{'b', 'd'}
	partner.apply(&input)

	assert.Equal(t, []rune{'a', 'd', 'c', 'b', 'e'}, input)
}
