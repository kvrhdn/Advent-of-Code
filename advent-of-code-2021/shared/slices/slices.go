package slices

func Filter[T any](slice []T, filter func(T) bool) []T {
	filteredSlice := make([]T, 0, len(slice))
	for _, t := range slice {
		if filter(t) {
			filteredSlice = append(filteredSlice, t)
		}
	}
	return filteredSlice
}
