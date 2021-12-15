package linkedlist

type Node[T any] struct {
	next  *Node[T]
	value T
}

func New[T any](value T) *Node[T] {
	return &Node[T]{
		next:  nil,
		value: value,
	}
}

func (n *Node[T]) Value() T {
	return n.value
}

func (n *Node[T]) Next() *Node[T] {
	return n.next
}

// InsertAfter insert an element after this node and returns the new node.
func (n *Node[T]) InsertAfter(value T) *Node[T] {
	next := n.next
	n.next = &Node[T]{
		next:  next,
		value: value,
	}
	return n.next
}
