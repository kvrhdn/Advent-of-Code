package snailfish

import (
	"strings"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestParseNumber(t *testing.T) {
	testCases := []string{
		"[1,2]",
		"[[1,2],3]",
		"[9,[8,7]]",
		"[[1,9],[8,5]]",
		"[[[[1,2],[3,4]],[[5,6],[7,8]]],9]",
		"[[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]",
		"[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]",
	}
	for _, tt := range testCases {
		t.Run(tt, func(t *testing.T) {
			number, _ := ParseNumber(tt)
			assert.Equal(t, tt, number.String())
		})
	}
}

func TestAdd(t *testing.T) {
	testCases := []struct {
		numbers []string
		result  string
	}{
		{[]string{"[1,2]", "[[3,4],5]"}, "[[1,2],[[3,4],5]]"},
		{[]string{"[1,1]", "[2,2]", "[3,3]", "[4,4]"}, "[[[[1,1],[2,2]],[3,3]],[4,4]]"},
		{[]string{"[1,1]", "[2,2]", "[3,3]", "[4,4]", "[5,5]"}, "[[[[3,0],[5,3]],[4,4]],[5,5]]"},
		{[]string{"[1,1]", "[2,2]", "[3,3]", "[4,4]", "[5,5]", "[6,6]"}, "[[[[5,0],[7,4]],[5,5]],[6,6]]"},
		{[]string{"[[[[4,3],4],4],[7,[[8,4],9]]]", "[1,1]"}, "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"},
		{
			[]string{
				"[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]",
				"[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]",
				"[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]",
				"[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]",
				"[7,[5,[[3,8],[1,4]]]]",
				"[[2,[2,2]],[8,[8,1]]]",
				"[2,9]",
				"[1,[[[9,3],9],[[9,0],[0,7]]]]",
				"[[[5,[7,4]],7],1]",
				"[[[[4,2],2],6],[8,7]]",
			},
			"[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
		},
	}
	for _, tt := range testCases {
		t.Run(strings.Join(tt.numbers, "+"), func(t *testing.T) {
			assert.Equal(t, tt.result, Add(tt.numbers...).String())
		})
	}
}

func Test_fullyReduce(t *testing.T) {
	testCases := []struct {
		before, after string
	}{
		{"[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]"},
		{"[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]"},
		{"[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]"},
		{"[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]", "[[3,[2,[8,0]]],[9,[5,[7,0]]]]"},
		{"[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]", "[[3,[2,[8,0]]],[9,[5,[7,0]]]]"},
	}
	for _, tt := range testCases {
		t.Run(tt.before, func(t *testing.T) {
			sn, _ := ParseNumber(tt.before)
			sn = fullyReduce(sn)
			assert.Equal(t, tt.after, sn.String())
		})
	}
}

func TestNumber_Magnitude(t *testing.T) {
	testCases := []struct {
		number   string
		expected int
	}{
		{"[9,1]", 29},
		{"[1,9]", 21},
		{"[[9,1],[1,9]]", 129},
		{"[[1,2],[[3,4],5]]", 143},
		{"[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", 1384},
		{"[[[[1,1],[2,2]],[3,3]],[4,4]]", 445},
		{"[[[[3,0],[5,3]],[4,4]],[5,5]]", 791},
		{"[[[[5,0],[7,4]],[5,5]],[6,6]]", 1137},
	}
	for _, tt := range testCases {
		t.Run(tt.number, func(t *testing.T) {
			number, _ := ParseNumber(tt.number)
			assert.Equal(t, tt.expected, number.Magnitude())
		})
	}
}
