# Project Definition

> Fill this template once per project. It is the single source of technology-specific information that all prompts and AI interactions reference.

---

## Project Identity

- **Project Name:** [e.g., my-awesome-app]
- **Description:** [One-sentence description of what the project does]
- **Project Type:** [library | web app | CLI | API service | mobile app | data pipeline | other]
- **Repository URL:** [e.g., https://github.com/user/repo]
- **License:** [e.g., MIT, Apache-2.0, proprietary]

## Technology Stack

### Language(s)

| Language | Version | Role |
|----------|---------|------|
| [e.g., TypeScript] | [e.g., 5.x] | [Primary / Secondary / Scripting] |

### Framework(s)

| Framework | Version | Purpose |
|-----------|---------|---------|
| [e.g., Next.js] | [e.g., 14.x] | [Web framework] |

### Key Dependencies

| Dependency | Version | Purpose |
|------------|---------|---------|
| [e.g., Prisma] | [e.g., 5.x] | [ORM / database access] |

### Build System & Package Manager

- **Package Manager:** [e.g., npm, pnpm, cargo, pip, go modules]
- **Build Tool:** [e.g., Vite, webpack, cargo, setuptools, go build]
- **Task Runner:** [e.g., npm scripts, Makefile, just, none]

## Repository Structure

```
[Describe or paste the top-level directory structure]
project/
├── src/
├── tests/
├── docs/
└── ...
```

### Key Directories

| Directory | Purpose |
|-----------|---------|
| [e.g., src/] | [Source code] |
| [e.g., tests/] | [Test files] |

## Code Conventions

### Naming

| Item | Convention | Example |
|------|-----------|---------|
| Files | [e.g., kebab-case] | [e.g., user-service.ts] |
| Functions/Methods | [e.g., camelCase] | [e.g., getUserById] |
| Classes/Types | [e.g., PascalCase] | [e.g., UserService] |
| Constants | [e.g., SCREAMING_SNAKE_CASE] | [e.g., MAX_RETRIES] |
| Variables | [e.g., camelCase] | [e.g., currentUser] |

### Code Style

- **Formatter:** [e.g., prettier, black, rustfmt]
- **Formatter Config:** [e.g., .prettierrc, pyproject.toml, default]
- **Max Line Length:** [e.g., 100, 120, no limit]
- **Indentation:** [e.g., 2 spaces, 4 spaces, tabs]
- **Import Order:** [Describe convention, e.g., stdlib → external → internal]

### Patterns & Conventions

- **Error Handling:** [Describe pattern, e.g., Result types, try/catch with custom errors, error boundaries]
- **Logging:** [e.g., structured logging with pino, console.log, log crate]
- **Configuration:** [e.g., environment variables, config files, dotenv]
- **API Design:** [e.g., RESTful, GraphQL, RPC — describe patterns used]
- **State Management:** [e.g., Redux, Zustand, Context API — for frontend projects]
- **Other:** [Any project-specific patterns worth noting]

## Testing

### Strategy

- **Unit Tests:** [e.g., co-located with source, in tests/ dir, both]
- **Integration Tests:** [e.g., separate directory, docker-compose, test database]
- **E2E Tests:** [e.g., Playwright, Cypress, none]
- **Test Framework:** [e.g., Jest, pytest, cargo test, go test]
- **Mocking:** [e.g., jest.mock, mockito, unittest.mock, none]
- **Coverage Target:** [e.g., 80%, no formal target]

### Test Naming Convention

[e.g., `test_should_return_user_when_id_exists`, `it('should return 404 for missing user')`]

## Quality Gates

> These commands must ALL pass before work is considered complete.

```bash
# Linter
[e.g., npm run lint]

# Formatter check
[e.g., npm run format:check]

# Type check
[e.g., npx tsc --noEmit]

# Tests
[e.g., npm test]

# Build
[e.g., npm run build]
```

## Documentation

### Required Documentation Files

| File | Purpose |
|------|---------|
| [e.g., README.md] | [Project overview, setup, usage] |
| [e.g., CHANGELOG.md] | [Version history] |
| [e.g., API.md] | [API documentation] |

### Documentation Style

- **Code Comments:** [e.g., JSDoc, docstrings, rustdoc, minimal]
- **Doc Examples:** [e.g., runnable, no_run, pseudocode]

## Deployment

- **Target Environment:** [e.g., Vercel, AWS Lambda, Docker, crates.io, npm]
- **CI/CD:** [e.g., GitHub Actions, GitLab CI, none]
- **Branch Strategy:** [e.g., main + feature branches, trunk-based, gitflow]

## Constraints & Non-Negotiables

> List any hard constraints the AI must always respect.

- [e.g., No unsafe code without justification]
- [e.g., All public APIs must have documentation]
- [e.g., No new dependencies without discussion]
- [e.g., Must support Node 18+]
- [e.g., Must remain backward compatible with v2.x API]
