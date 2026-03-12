# Project Definition — Node.js + Express API

> Pre-filled preset for a Node.js REST API with Express. Customize for your project.

---

## Project Identity

- **Project Name:** [your-api-name]
- **Description:** [One-sentence description]
- **Project Type:** API service
- **Repository URL:** [https://github.com/user/repo]
- **License:** MIT

## Technology Stack

### Language(s)

| Language | Version | Role |
|----------|---------|------|
| TypeScript | 5.x | Primary |
| Node.js | 20.x LTS | Runtime |

### Framework(s)

| Framework | Version | Purpose |
|-----------|---------|---------|
| Express | 4.x | HTTP framework |

### Key Dependencies

| Dependency | Version | Purpose |
|------------|---------|---------|
| zod | 3.x | Request/response validation |
| prisma | 5.x | ORM / database access |
| pino | 8.x | Structured logging |
| helmet | 7.x | Security headers |
| cors | 2.x | CORS middleware |
| dotenv | 16.x | Environment variable loading |

### Build System & Package Manager

- **Package Manager:** npm (or pnpm)
- **Build Tool:** tsc (TypeScript compiler)
- **Task Runner:** npm scripts

## Repository Structure

```
project/
├── src/
│   ├── index.ts          # Entry point
│   ├── routes/           # Route definitions
│   ├── controllers/      # Request handlers
│   ├── services/         # Business logic
│   ├── models/           # Data models / Prisma schema
│   ├── middleware/        # Express middleware
│   ├── utils/            # Utility functions
│   └── types/            # TypeScript type definitions
├── tests/
│   ├── unit/
│   └── integration/
├── prisma/
│   └── schema.prisma
├── package.json
├── tsconfig.json
└── .env.example
```

### Key Directories

| Directory | Purpose |
|-----------|---------|
| src/routes/ | Express route definitions |
| src/controllers/ | Request handling logic |
| src/services/ | Business logic (framework-agnostic) |
| tests/ | Unit and integration tests |

## Code Conventions

### Naming

| Item | Convention | Example |
|------|-----------|---------|
| Files | kebab-case | user-service.ts |
| Functions/Methods | camelCase | getUserById |
| Classes/Types | PascalCase | UserService |
| Constants | SCREAMING_SNAKE_CASE | MAX_RETRIES |
| Variables | camelCase | currentUser |
| Route paths | kebab-case | /api/user-profiles |

### Code Style

- **Formatter:** prettier
- **Formatter Config:** .prettierrc (semi: true, singleQuote: true, trailingComma: 'all')
- **Max Line Length:** 100
- **Indentation:** 2 spaces
- **Import Order:** node builtins → external packages → internal modules → types

### Patterns & Conventions

- **Error Handling:** Custom error classes extending Error, centralized error middleware, HTTP status codes in responses
- **Logging:** Structured logging with pino (request ID in every log)
- **Configuration:** Environment variables via dotenv, validated at startup with zod
- **API Design:** RESTful, JSON request/response, consistent error format `{ error: { code, message } }`
- **Validation:** zod schemas for request body/params/query validation in middleware

## Testing

### Strategy

- **Unit Tests:** In tests/unit/, mirroring src/ structure
- **Integration Tests:** In tests/integration/, testing full HTTP request/response
- **Test Framework:** Jest with ts-jest
- **Mocking:** jest.mock for module mocking, supertest for HTTP testing
- **Coverage Target:** 80%

### Test Naming Convention

`describe('ServiceName', () => { it('should return user when id exists', ...) })`

## Quality Gates

```bash
# Linter
npx eslint .

# Formatter check
npx prettier --check .

# Type check
npx tsc --noEmit

# Tests
npm test

# Build
npm run build
```

## Documentation

### Required Documentation Files

| File | Purpose |
|------|---------|
| README.md | Project overview, setup, API endpoints |
| CHANGELOG.md | Version history |
| .env.example | Required environment variables |

### Documentation Style

- **Code Comments:** JSDoc for public functions, inline comments for complex logic
- **API Docs:** OpenAPI/Swagger (optional)

## Deployment

- **Target Environment:** Docker / cloud provider
- **CI/CD:** GitHub Actions
- **Branch Strategy:** main + feature branches

## Constraints & Non-Negotiables

- All endpoints must validate input with zod
- All responses must follow the standard response format
- No `any` type without explicit justification
- Environment variables must be validated at startup
- Secrets must never be committed — use .env and .env.example
