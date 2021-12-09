package main

import (
	"context"
	"fmt"
	"os"

	"github.com/joho/godotenv"
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
	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/exporters/otlp/otlptrace/otlptracegrpc"
	"go.opentelemetry.io/otel/sdk/resource"
	sdktrace "go.opentelemetry.io/otel/sdk/trace"
	semconv "go.opentelemetry.io/otel/semconv/v1.7.0"
	"google.golang.org/grpc/credentials"
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
			1: day01.Solution,
			2: day02.Solution,
			3: day03.Solution,
			4: day04.Solution,
			5: day05.Solution,
			6: day06.Solution,
			7: day07.Solution,
			8: day08.Solution,
			9: day09.Solution,
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
