package aoc

import (
	"context"
	"fmt"
	"testing"
	"time"

	"github.com/stretchr/testify/assert"
	"go.opentelemetry.io/otel"
)

type Day interface {
	run(ctx context.Context, day, part int) error

	VerifySolution(t *testing.T, day int, part1, part2 string)
}

type dayImpl[T any] struct {
	generator    func(ctx context.Context, input string) (T, error)
	part1, part2 func(ctx context.Context, input T) (string, error)
}

func NewDay(part1, part2 func(ctx context.Context, input string) (string, error)) Day {
	passthrough := func(ctx context.Context, input string) (string, error) {
		return input, nil
	}
	return NewDayGen(passthrough, part1, part2)
}

func NewDayGen[T any](generator func(ctx context.Context, input string) (T, error), part1, part2 func(ctx context.Context, input T) (string, error)) Day {
	return &dayImpl[T]{generator, part1, part2}
}

func (s *dayImpl[T]) run(ctx context.Context, day, part int) error {
	ctx, span := otel.Tracer("").Start(ctx, fmt.Sprintf("day %d", day))
	defer span.End()

	input, err := readInput(ctx, day)
	if err != nil {
		return fmt.Errorf("could not read input for day %d: %w", day, err)
	}

	fmt.Printf("Day %d\n", day)

	gen, err := s.runGenerator(ctx, input)
	if err != nil {
		return err
	}

	if part == 0 || part == 1 {
		err := s.runSolver(ctx, 1, gen)
		if err != nil {
			return err
		}
	}

	if part == 0 || part == 2 {
		err := s.runSolver(ctx, 2, gen)
		if err != nil {
			return err
		}
	}

	return nil
}

func (s *dayImpl[T]) runGenerator(ctx context.Context, input string) (T, error) {
	ctx, span := otel.Tracer("").Start(ctx, "generator")
	defer span.End()

	start := time.Now()
	defer func() {
		fmt.Printf("- Generator [%s]\n", time.Since(start))
	}()

	return s.generator(ctx, input)
}

func (s *dayImpl[T]) runSolver(ctx context.Context, part int, input T) error {
	ctx, span := otel.Tracer("").Start(ctx, fmt.Sprintf("part %d", part))
	defer span.End()

	var solver func(ctx context.Context, input T) (string, error)
	switch part {
	case 1:
		solver = s.part1
	case 2:
		solver = s.part2
	}
	if solver == nil {
		return nil
	}

	start := time.Now()
	var output string
	defer func() {
		fmt.Printf("- Part %d    [%s]\n", part, time.Since(start))
		fmt.Printf("%s\n", output)
	}()

	output, err := solver(ctx, input)
	return err
}

func (s *dayImpl[T]) VerifySolution(t *testing.T, day int, part1, part2 string) {
	ctx := context.Background()

	input, err := readInputFrom(ctx, "../", day)
	assert.NoError(t, err, "Day %d, reading input", day)

	gen, err := s.generator(ctx, input)
	assert.NoError(t, err, "Day %d, generating input", day)

	if s.part1 != nil {
		output, err := s.part1(ctx, gen)
		assert.NoError(t, err, "Day %d, part 2", day)
		assert.Equal(t, part1, output, "Day %d, part 2", day)
	}
	if s.part2 != nil {
		output, err := s.part2(ctx, gen)
		assert.NoError(t, err, "Day %d, part 2", day)
		assert.Equal(t, part2, output, "Day %d, part 2", day)
	}
}
