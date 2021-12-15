package priorityqueue

import (
	"sort"
)

type item[T any] struct {
	priority int
	value    T
}

type PriorityQueue[T any] struct {
	items []item[T]
}

func New[T any]() PriorityQueue[T] {
	return PriorityQueue[T]{}
}

// Push adds an item to the queue.
func (pq *PriorityQueue[T]) Push(priority int, value T) {
	pq.items = append(pq.items, item[T]{priority, value})
}

// Pop returns the item with the lowest priority and removes it from the queue.
func (pq *PriorityQueue[T]) Pop() T {
	// only sort when removing an item, assumption: pop will be called less than push
	sort.Slice(pq.items, func(i, j int) bool {
		return pq.items[i].priority < pq.items[j].priority
	})

	i := pq.items[0]
	pq.items = pq.items[1:]
	return i.value

}

func (pq *PriorityQueue[T]) IsEmpty() bool {
	return len(pq.items) == 0
}
