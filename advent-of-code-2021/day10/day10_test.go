package day10

import (
	"testing"
)

func TestSolution(t *testing.T) {
	Solution.VerifySolution(t, 10, 167379, 2776842859)
}

func TestExample(t *testing.T) {
	example := `[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]`
	Solution.VerifyInput(t, example, 26397, 288957)
}
