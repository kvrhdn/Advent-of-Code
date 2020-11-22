package knotHash

func createSequence(size int) []int {
	sequence := make([]int, size)

	for i := range sequence {
		sequence[i] = i
	}

	return sequence
}

func times(times int, do func()) {
	for i := 0; i < times; i++ {
		do()
	}
}
