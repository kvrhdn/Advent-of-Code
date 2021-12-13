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

func (s *Set[T]) Remove(value T) {
	delete(s.m, value)
}

func (s *Set[T]) Len() int {
	return len(s.m)
}

func (s *Set[T]) Values() []T {
	keys := make([]T, 0, len(s.m))
	for k, _ := range s.m {
		keys = append(keys, k)
	}
	return keys
}

func (s *Set[T]) Copy() Set[T] {
	newSet := New[T]()
	for k, _ := range s.m {
		newSet.Add(k)
	}
	return newSet
}
