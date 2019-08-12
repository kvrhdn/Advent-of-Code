package circular

type CircularBuffer struct {
	data []int
}

func NewBuffer() CircularBuffer {
	return CircularBuffer{}
}

func (b *CircularBuffer) Get(index int) int {
	return b.data[b.wrap(index)]
}

func (b *CircularBuffer) InsertAfter(index int, value int) (newIndex int) {
	splitIndex := b.wrap(index) + 1

	if splitIndex > len(b.data) {
		b.data = append(b.data, value)
		return splitIndex
	}

	// ensure there is enough space
	b.data = append(b.data, 0)

	// move everything after splitIndex
	copy(b.data[splitIndex+1:], b.data[splitIndex:])

	// insert value
	b.data[splitIndex] = value

	return splitIndex
}

func (b *CircularBuffer) Find(value int) (index int) {
	for i, v := range b.data {
		if v == value {
			return i
		}
	}
	return -1
}

func (b *CircularBuffer) wrap(index int) int {
	if len(b.data) == 0 {
		return 0
	}
	return index % len(b.data)
}
