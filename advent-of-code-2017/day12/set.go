package day12

type Set map[int]int

func newSet() Set {
	return make(map[int]int)
}

func newSetFromSlice(slice []int) Set {
	s := newSet()
	for _, v := range slice {
		s[v] = v
	}
	return s
}

func (s Set) contains(value int) bool {
	_, ok := s[value]
	return ok
}

func (s Set) containAnyOf(other []int) bool {
	for _, v := range other {
		if s.contains(v) {
			return true
		}
	}
	return false
}

func (s Set) insertAll(other []int) {
	for _, o := range other {
		s[o] = o
	}
}

func (s Set) merge(other Set) {
	for o := range other {
		s[o] = o
	}
}
