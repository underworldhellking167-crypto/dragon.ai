# Dragon.AI

Terminal-Based Universal AI Runtime

## Concept

Dragon.AI is a lightweight terminal-only AI engine that separates model architecture from parameters.
The engine ships once. Parameter files are provided by the user and streamed from disk at runtime.

## Folder structure

- `dragon` — CLI binary
- `architectures/` — built-in model designs
- `tokenizers/` — tokenizer assets
- `params/` — user-provided parameter files
- `config.yml` — configuration

## Quick start

```bash
cargo run -- run
```

## CLI examples

```bash
dragon run --params ./params/llama-70b.params
dragon scan
dragon ask "What is quantum computing?" --params ./params/p.bin
```
