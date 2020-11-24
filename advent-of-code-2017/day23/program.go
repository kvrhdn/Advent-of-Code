package day23

// runDirectAssembly contains the input directly translated into Go statements.
func runDirectAssembly(initA int) (registers map[rune]int, mulCalled int) {
	a := initA
	b := 0
	c := 0
	d := 0
	e := 0
	f := 0
	g := 0
	h := 0

	//  0 | set b 93
	b = 93
	//  1 | set c b
	c = b
	//  2 | jnz a 2
	if a != 0 {
		goto ir4
	}
	//  3 | jnz 1 5
	if 1 != 0 {
		goto ir8
	}
	//  4 | mul b 100
ir4:
	mulCalled++
	b *= 100
	//  5 | sub b -100000
	b -= -100000
	//  6 | set c b
	c = b
	//  7 | sub c -17000
ir7:
	c -= -17000
	//  8 | set f 1
ir8:
	f = 1
	//  9 | set d 2
	d = 2
	// 10 | set e 2
ir10:
	e = 2
	// 11 | set g d
ir11:
	g = d
	// 12 | mul g e
	mulCalled++
	g *= e
	// 13 | sub g b
	g -= b
	// 14 | jnz g 2
	if g != 0 {
		goto ir16
	}
	// 15 | set f 0
	f = 0
	// 16 | sub e -1
ir16:
	e -= -1
	// 17 | set g e
	g = e
	// 18 | sub g b
	g -= b
	// 19 | jnz g -8
	if g != 0 {
		goto ir11
	}
	// 20 | sub d -1
	d -= -1
	// 21 | set g d
	g = d
	// 22 | sub g b
	g -= b
	// 23 | jnz g -13
	if g != 0 {
		goto ir10
	}
	// 24 | jnz f 2
	if f != 0 {
		goto ir26
	}
	// 25 | sub h -1
	h -= -1
	// 26 | set g b
ir26:
	g = b
	// 27 | sub g c
	g -= c
	// 28 | jnz g 2
	if g != 0 {
		goto ir30
	}
	// 29 | jnz 1 3
	if 1 != 0 {
		goto ir32
	}
	// 30 | sub b -17
ir30:
	b -= -17
	// 31 | jnz 1 -23
	if 1 != 0 {
		goto ir7
	}
ir32:
	return pack(a, b, c, d, e, f, g, h), mulCalled
}

// runOptimized is an optimized version of runDirectAssembly in proper Go
func runOptimized(initA int) (registers map[rune]int) {
	a := initA
	b := 93
	c := 93
	d := 0
	e := 0
	f := 0
	g := 0
	h := 0

	if a != 0 {
		b = 109300
		c = 126300
	}

	for {
		f = 1

		// if b is not a prime number -> f = 0
		for d = 2; d != b; d++ {
			for e = 2; e != b; e++ {
				if (d * e) == b {
					f = 0
				}
			}
		}

		// if f == 0 -> h++
		if f == 0 {
			h++
		}

		// if b == c, done
		if b == c {
			return pack(a, b, c, d, e, f, g, h)
		}

		b += 17
	}
}

// optimizedPart2 is functionally the same code as runOptimized but completely
// focussed on h
func optimizedPart2() int {
	b := 109300
	c := 126300

	h := 0

	for x := b; x <= c; x += 17 {
		for e := 2; e < x; e++ {
			if x%e == 0 {
				h++
				break
			}
		}
	}

	return h
}

func pack(a, b, c, d, e, f, g, h int) map[rune]int {
	return map[rune]int{
		'a': a,
		'b': b,
		'c': c,
		'd': d,
		'e': e,
		'f': f,
		'g': g,
		'h': h,
	}
}
