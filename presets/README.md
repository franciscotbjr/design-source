# Presets

Pre-filled Project Definitions for common technology stacks. Copy a preset as your starting point instead of filling the Project Definition template from scratch.

## How to Use

1. Find the preset that matches your stack (or is closest to it)
2. Copy the preset file to your project as your Project Definition
3. Customize the copied file to match your specific project (name, repo URL, dependencies, etc.)
4. Use it with the prompts in `prompts/`

## Available Presets

| Preset | Stack | Best For |
|--------|-------|----------|
| [rust-library.md](rust-library.md) | Rust + Cargo + clippy/fmt | Rust libraries and crates |
| [node-express-api.md](node-express-api.md) | Node.js + Express + Jest + ESLint | REST API backends |
| [python-fastapi.md](python-fastapi.md) | Python + FastAPI + pytest + ruff | Python web APIs and services |
| [react-webapp.md](react-webapp.md) | React + TypeScript + Vite + Vitest | Frontend web applications |
| [go-service.md](go-service.md) | Go + standard library + go vet/test | Go backend services |

## Contributing a Preset

If you have a stack that's not covered, contributions are welcome. A good preset should:

1. Follow the same structure as `templates/project/project-definition.md`
2. Fill in realistic, production-ready values (not toy examples)
3. Include the most common quality gates for that stack
4. Document conventions that are idiomatic for the language/framework
