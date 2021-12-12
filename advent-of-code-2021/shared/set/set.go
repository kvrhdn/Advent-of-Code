package set

type Set[T comparable] struct {
	m map[T]struct{}
}

func New[T comparable]() Set[T] {
	return Set[T]{
		m: make(map[T]struct{}),
	}
}

func (s *Set[T]) Contains(value T) bool {
	_, ok := s.m[value]
	return ok
}

func (s *Set[T]) Add(value T) {
	s.m[value] = struct{}{}
}

func (s *Set[T]) Copy() Set[T] {
	newSet := New[T]()
	for k, _ := range s.m {
		newSet.Add(k)
	}
	return newSet
}
