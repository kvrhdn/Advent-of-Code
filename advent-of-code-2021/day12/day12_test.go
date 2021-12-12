package day12

import (
	"strconv"
	"testing"
)

func TestSolution(t *testing.T) {
	Solution.VerifySolution(t, 12, 3000, 74222)
}

func TestExample(t *testing.T) {
	testCases := []struct {
		input        string
		part1, part2 int
	}{
		{
			`start-A
start-b
A-c
A-b
b-d
A-end
b-end`,
			10, 36,
		},
		{
			`dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc`,
			19, 103,
		},
		{
			`fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW`,
			226, 3509,
		},
	}

	for i, testCase := range testCases {
		t.Run(strconv.Itoa(i), func(t *testing.T) {
			Solution.VerifyInput(t, testCase.input, testCase.part1, testCase.part2)
		})
	}
}
