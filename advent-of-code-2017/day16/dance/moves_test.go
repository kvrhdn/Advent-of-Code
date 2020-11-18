package dance

import (
	"reflect"
	"testing"
)

func TestParseMoves(t *testing.T) {
	input := "s1,x3/4,pe/b"
	expected := []Move{spin{1}, exchange{3, 4}, partner{'e', 'b'}}

	got := ParseMoves(input)
	if !reflect.DeepEqual(got, expected) {
		t.Errorf("ParseMoves() = %v, want %v", got, expected)
	}
}

func Test_spin_apply(t *testing.T) {
	input := []rune{'a', 'b', 'c', 'd', 'e'}
	expected := []rune{'e', 'a', 'b', 'c', 'd'}

	s := spin{1}

	got := s.apply(input)
	if !reflect.DeepEqual(got, expected) {
		t.Errorf("spin apply on %v got %v, but expected %v", input, got, expected)
	}
}

func Test_exchange_apply(t *testing.T) {
	input := []rune{'a', 'b', 'c', 'd', 'e'}
	expected := []rune{'a', 'd', 'c', 'b', 'e'}

	e := exchange{1, 3}

	got := e.apply(input)
	if !reflect.DeepEqual(got, expected) {
		t.Errorf("exchange apply on %v got %v, but expected %v", input, got, expected)
	}
}

func Test_partner_apply(t *testing.T) {
	input := []rune{'a', 'b', 'c', 'd', 'e'}
	expected := []rune{'a', 'd', 'c', 'b', 'e'}

	p := partner{'b', 'd'}

	got := p.apply(input)
	if !reflect.DeepEqual(got, expected) {
		t.Errorf("partner apply on %v got %v, but expected %v", input, got, expected)
	}
}
