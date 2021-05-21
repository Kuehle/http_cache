# Simple HTTP Cache

Exposes to endpoints:

1. `POST /{key}` which takes a value (utf8) in the body of the request and caches it.
1. `GET /{key}` which returns a previously set value or a 404 error.

## Running it

`cargo run` runs the server.

## Configuration

The Env is used to configure the server with the following options:

- TIMEOUT=[timeout in seconds] until a value is not available any more
- HOST=[hostname/ip] to bind to
- PORT=[port number] to bind to

## Testing

Start the server with `TIMEOUT=1 cargo run` to reduce the timeout. The Tests retry the route after 2s.
Run the tests with `./test.sh` to test the contract of the API.
