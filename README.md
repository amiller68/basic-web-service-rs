# basic-web-service

Simple basic web service with axum.

Comes with:

- `axum` for the web server.
- example `api`, `app` (state + config), `health`, and http `server`.

Easily extensible to use cases like:

- Serving static files or a HATEOAS API.
- Serving a (RESTful) API.
- Implementing a database application
- etc.

## Requirements

- cargo + rustc
- cargo-watch
- cmake + libclang-dev
- docker (or podman)

## Usage

Build
```sh
make
```

Test
```bash
make test
```

Dev 
```bash
make watch
```

Run 
```bash
make run
```

Clean
```bash
make clean
```

Format
```bash
make fmt
```

Check 
```bash
make Check
```
