.PHONY: tidy install-gotip

tidy:
	gotip fmt ./...
	gotip mod tidy

install-gotip:
	go install golang.org/dl/gotip@latest
	gotip download
