package circular

// Buffer is a circular array.
type Buffer struct {
	data []int
}

func New(initial []int) Buffer {
	return Buffer{
		data: initial,
	}
}

func (b *Buffer) wrap(index int) int {
	if len(b.data) == 0 {
		return 0
	}
	return index % len(b.data)
}

func (b *Buffer) Get(index int) int {
	return b.data[b.wrap(index)]
}

func (b *Buffer) InsertAfter(index int, value int) (newIndex int) {
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

func (b *Buffer) Find(value int) (index int) {
	for i, v := range b.data {
		if v == value {
			return i
		}
	}
	return -1
}
