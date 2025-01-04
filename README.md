# proxy-wasm-ab-testing-header

Sample proxy-wasm plugin that adds HTTP request header for A/B testing.

## build

```sh
$ make build
```

## use in Docker

```sh
$ make docker-compose-up
```

## example

```sh
$ make docker-compose-up

$ curl localhost:10000
OK (control)

$ curl localhost:10000
OK (variation)
```
