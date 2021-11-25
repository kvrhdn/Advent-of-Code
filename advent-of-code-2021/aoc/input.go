package aoc

import (
	"context"
	"errors"
	"fmt"
	"io"
	"io/ioutil"
	"net/http"
	"os"
	"strings"

	"go.opentelemetry.io/otel"
)

func readInput(ctx context.Context, day int) (string, error) {
	return readInputFrom(ctx, "", day)
}

func readInputFrom(ctx context.Context, path string, day int) (string, error) {
	path = fmt.Sprintf("%s./input/%02d/day%02d.txt", path, year, day)

	if _, err := os.Stat(path); err != nil {
		if errors.Is(err, os.ErrNotExist) {
			err = downloadInput(ctx, day, path)
			if err != nil {
				return "", fmt.Errorf("downloading input failed: %w", err)
			}
		} else {
			return "", err
		}
	}

	bytes, err := ioutil.ReadFile(path)
	return strings.TrimSpace(string(bytes)), err
}

func downloadInput(ctx context.Context, day int, dest string) error {
	ctx, span := otel.Tracer("").Start(ctx, fmt.Sprintf("downloading input day %d", day))
	defer span.End()

	fmt.Printf("Input for day %d does not exist, downloading...\n\n", day)

	session := os.Getenv("AOC_SESSION")
	if session == "" {
		return fmt.Errorf("set AOC_SESSION environment variable")
	}
	cookie := &http.Cookie{
		Name:  "session",
		Value: session,
	}

	req, err := http.NewRequestWithContext(ctx, "GET", fmt.Sprintf("https://adventofcode.com/%d/day/%d/input", year, day), nil)
	if err != nil {
		return err
	}

	req.AddCookie(cookie)

	client := &http.Client{}
	res, err := client.Do(req)
	if err != nil {
		return err
	}
	defer res.Body.Close()

	inputBytes, err := io.ReadAll(res.Body)
	if err != nil {
		return err
	}

	if res.StatusCode != http.StatusOK {
		return fmt.Errorf("request failed (%d): %s", res.StatusCode, string(inputBytes))
	}

	err = os.WriteFile(dest, inputBytes, 0666)
	if err != nil {
		return err
	}
	return nil
}
