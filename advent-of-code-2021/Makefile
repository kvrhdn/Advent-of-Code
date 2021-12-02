.PHONY: run test tidy install-gotip

run:
	gotip run .

test:
	gotip test ./...

tidy:
	gotip fmt ./...
	gotip mod tidy

install-gotip:
	go install golang.org/dl/gotip@latest
	gotip download
