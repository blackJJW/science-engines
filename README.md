# science-engines

A work-in-progress project to build a set of foundational science engines:
Math -> Physics -> Chemistry -> Biology => Applied Engineering

## Status

- Early development
- Currently working on `engine-core` (ODE + integreators + tests)

## Repository structures

- `crates/engine-core/` — core library (in progress)
- `docker-compose.yml` / `Dockerfile` — dev environment
- `Makefile` — common commands (`make test`, `make fmt`, ...)

## Quick start

### Using Docker (recommended)

```bash
make up
make test
```

### Enter the dev container

```bash
make sh
```

## Roadmap (high-level)

- Math engine foundations (ODE, linear algebra, optimization)
- Physics engine (simulation + visulaization)
- GPU backends and real-time execution
- Docs site (GitHub Pages)

## License

- TBD (will be added later)