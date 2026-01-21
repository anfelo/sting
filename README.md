# Sting

![Sting](sting.jpg)

> Bilbo's blade that glows blue when enemies are near. Detects problems in your Nx monorepo.

A fast CLI for static analysis of Nx TypeScript projects.

## Why Sting?

- **Fast** - Faster alternative to `nx affected`
- **Static analysis** - Finds issues invisible to linters
- **AI-friendly** - Designed for use with AI tools to reduce context

## Installation

```sh
cargo install sting
```

## Commands

```sh
sting query-all <path>     # List all entities
sting query <path> <name>  # Find specific entity
sting unused <path>        # Find unused entities
sting graph <path>         # Output dependency graph as JSON
sting affected <path>      # List affected files (git-based)
```

## Status

Experimental - APIs may change.

## License

MIT
