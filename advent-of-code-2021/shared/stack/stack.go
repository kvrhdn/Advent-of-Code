package stack

type Stack[T any] struct {
	items []T
}

func New[T any]() *Stack[T] {
	return &Stack[T]{}
}

func (s *Stack[T]) Push(item T) {
	s.items = append(s.items, item)
}

func (s *Stack[T]) Pop() T {
	if s.IsEmpty() {
		panic("can not pop item from stack, stack is empty!")
	}
	item := s.items[len(s.items)-1]
	s.items = s.items[0 : len(s.items)-1]
	return item
}

func (s *Stack[T]) IsEmpty() bool {
	return len(s.items) == 0
}
