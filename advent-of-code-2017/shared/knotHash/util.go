package knotHash

func createSequence(size int) []int {
	sequence := make([]int, size)

	for i := range sequence {
		sequence[i] = i
	}

	return sequence
}
