package util

func Times(times int, do func()) {
	for i := 0; i < times; i++ {
		do()
	}
}
