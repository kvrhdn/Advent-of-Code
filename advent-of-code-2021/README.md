# Advent of Code 2021

My solutions for [Advent of Code 2021](https://adventofcode.com/2021).

To run these solutions you'll need Go 1.18, which is not available at the time of writing.
Instead build the latest version of Go using gotip:

```
go install golang.org/dl/gotip@latest
gotip download
```

Use the `gotip` command instead of `go`

### Tracing

These solutions have tracing built-in from the start.
You can send traces to a local tracing backend ([Grafana Tempo](https://github.com/grafana/tempo)) or to [Grafana Cloud Traces](https://grafana.com/). 

Tweak [.env](.env) to control the tracing exporter.

To run Grafana and Tempo locally:

```
cd o11y/
docker-compose up -d
```

Visit Grafana and start searching for traces at [localhost:3000/explore](http://localhost:3000/explore).
