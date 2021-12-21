package main

import (
	"context"
	"fmt"
	"os"

	"github.com/joho/godotenv"
	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/exporters/otlp/otlptrace/otlptracegrpc"
	"go.opentelemetry.io/otel/sdk/resource"
	sdktrace "go.opentelemetry.io/otel/sdk/trace"
	semconv "go.opentelemetry.io/otel/semconv/v1.7.0"
	"google.golang.org/grpc/credentials"

	"github.com/kvrhdn/advent-of-code/advent-of-code-2021/aoc"
	"github.com/kvrhdn/advent-of-code/advent-of-code-2021/day01"
	"github.com/kvrhdn/advent-of-code/advent-of-code-2021/day02"
	"github.com/kvrhdn/advent-of-code/advent-of-code-2021/day03"
	"github.com/kvrhdn/advent-of-code/advent-of-code-2021/day04"
	"github.com/kvrhdn/advent-of-code/advent-of-code-2021/day05"
	"github.com/kvrhdn/advent-of-code/advent-of-code-2021/day06"
	"github.com/kvrhdn/advent-of-code/advent-of-code-2021/day07"
	"github.com/kvrhdn/advent-of-code/advent-of-code-2021/day08"
	"github.com/kvrhdn/advent-of-code/advent-of-code-2021/day09"
	"github.com/kvrhdn/advent-of-code/advent-of-code-2021/day10"
	"github.com/kvrhdn/advent-of-code/advent-of-code-2021/day11"
	"github.com/kvrhdn/advent-of-code/advent-of-code-2021/day12"
	"github.com/kvrhdn/advent-of-code/advent-of-code-2021/day13"
	"github.com/kvrhdn/advent-of-code/advent-of-code-2021/day14"
	"github.com/kvrhdn/advent-of-code/advent-of-code-2021/day15"
	"github.com/kvrhdn/advent-of-code/advent-of-code-2021/day16"
	"github.com/kvrhdn/advent-of-code/advent-of-code-2021/day17"
	"github.com/kvrhdn/advent-of-code/advent-of-code-2021/day18"
	"github.com/kvrhdn/advent-of-code/advent-of-code-2021/day19"
	"github.com/kvrhdn/advent-of-code/advent-of-code-2021/day20"
	"github.com/kvrhdn/advent-of-code/advent-of-code-2021/day21"
)

func main() {
	_ = godotenv.Load("./.env")

	ctx := context.Background()

	shutdown, err := setupTracing(ctx)
	if err != nil {
		_, _ = fmt.Fprintf(os.Stderr, "Error: %s\n", err)
		return
	}
	defer shutdown()

	ctx, span := otel.Tracer("").Start(ctx, "main")
	defer span.End()

	config := aoc.Configuration{
		Days: map[int]aoc.Day{
			1:  day01.Solution,
			2:  day02.Solution,
			3:  day03.Solution,
			4:  day04.Solution,
			5:  day05.Solution,
			6:  day06.Solution,
			7:  day07.Solution,
			8:  day08.Solution,
			9:  day09.Solution,
			10: day10.Solution,
			11: day11.Solution,
			12: day12.Solution,
			13: day13.Solution,
			14: day14.Solution,
			15: day15.Solution,
			16: day16.Solution,
			17: day17.Solution,
			18: day18.Solution,
			19: day19.Solution,
			20: day20.Solution,
			21: day21.Solution,
		},
	}
	err = config.Run(ctx)
	if err != nil {
		_, _ = fmt.Fprintf(os.Stderr, "Error: %s\n", err)
		return
	}
}

func setupTracing(ctx context.Context) (func(), error) {
	// Remove WithTLSCredentials() when sending to an insecure backend
	// https://github.com/open-telemetry/opentelemetry-go/issues/1584#issuecomment-978486223
	exporter, err := otlptracegrpc.New(ctx, otlptracegrpc.WithTLSCredentials(credentials.NewClientTLSFromCert(nil, "")))
	if err != nil {
		return nil, err
	}

	tp := sdktrace.NewTracerProvider(
		sdktrace.WithBatcher(exporter),
		sdktrace.WithResource(resource.NewWithAttributes(
			semconv.SchemaURL,
			semconv.ServiceNameKey.String("aoc"),
		)),
	)
	otel.SetTracerProvider(tp)

	otel.SetErrorHandler(otel.ErrorHandlerFunc(func(err error) {
		_, _ = fmt.Fprintf(os.Stderr, "OTel error: %s\n", err)
	}))

	return func() {
		_ = tp.Shutdown(ctx)
		_ = exporter.Shutdown(ctx)
	}, nil
}
