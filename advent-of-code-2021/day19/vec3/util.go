package vec3

func times(times int, fn func()) {
	for i := 0; i < times; i++ {
		fn()
	}
}
