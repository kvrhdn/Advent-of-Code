package day12

type Set map[int]int

func newSet() Set {
	return make(map[int]int)
}

func (s Set) contains(value int) bool {
	_, ok := s[value]
	return ok
}

func (s Set) anyInCommon(other Set) bool {
	for v1 := range other {
		if s.contains(v1) {
			return true
		}
	}
	return false
}

func (s Set) insert(value int) {
	s[value] = value
}

func (s Set) merge(other Set) {
	for o := range other {
		s[o] = o
	}
}
