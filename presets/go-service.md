# Project Definition — Go Backend Service

> Pre-filled preset for a Go backend service. Customize for your project.

---

## Project Identity

- **Project Name:** [your-service-name]
- **Description:** [One-sentence description]
- **Project Type:** API service
- **Repository URL:** [https://github.com/user/repo]
- **License:** MIT

## Technology Stack

### Language(s)

| Language | Version | Role |
|----------|---------|------|
| Go | 1.22+ | Primary |

### Framework(s)

| Framework | Version | Purpose |
|-----------|---------|---------|
| net/http (stdlib) | - | HTTP server |
| chi (optional) | 5.x | HTTP router |

### Key Dependencies

| Dependency | Version | Purpose |
|------------|---------|---------|
| slog (stdlib) | - | Structured logging |
| database/sql (stdlib) | - | Database access |
| pgx | 5.x | PostgreSQL driver (if needed) |
| testify | 1.x | Test assertions and mocking |
| golangci-lint | latest | Linter aggregator |

### Build System & Package Manager

- **Package Manager:** go modules
- **Build Tool:** go build
- **Task Runner:** Makefile or just

## Repository Structure

```
project/
├── cmd/
│   └── server/
│       └── main.go           # Entry point
├── internal/
│   ├── handler/              # HTTP handlers
│   ├── service/              # Business logic
│   ├── repository/           # Data access
│   ├── model/                # Domain types
│   └── config/               # Configuration
├── pkg/                      # Public library code (if any)
├── tests/
│   └── integration/
├── migrations/
├── go.mod
├── go.sum
├── Makefile
└── Dockerfile
```

### Key Directories

| Directory | Purpose |
|-----------|---------|
| cmd/ | Application entry points |
| internal/ | Private application code |
| internal/handler/ | HTTP request handlers |
| internal/service/ | Business logic |
| internal/repository/ | Database access |

## Code Conventions

### Naming

| Item | Convention | Example |
|------|-----------|---------|
| Files | snake_case | user_service.go |
| Functions (exported) | PascalCase | GetUserByID |
| Functions (unexported) | camelCase | getUserByID |
| Types/Structs | PascalCase | UserService |
| Constants (exported) | PascalCase | MaxRetries |
| Constants (unexported) | camelCase | maxRetries |
| Interfaces | PascalCase, -er suffix | UserRepository, Reader |
| Packages | lowercase, single word | handler, service |

### Code Style

- **Formatter:** gofmt / goimports
- **Max Line Length:** No formal limit (use judgment)
- **Indentation:** Tabs (gofmt default)
- **Import Order:** stdlib → external → internal (enforced by goimports)

### Patterns & Conventions

- **Error Handling:** Return errors explicitly, wrap with `fmt.Errorf("context: %w", err)`, custom error types for domain errors
- **Logging:** slog (structured logging), pass logger via context or dependency injection
- **Configuration:** Environment variables parsed at startup, validated in config package
- **API Design:** RESTful JSON, handler → service → repository layering
- **Dependency Injection:** Constructor injection (NewService(repo, logger)), interfaces for testability

## Testing

### Strategy

- **Unit Tests:** Co-located with source (file_test.go)
- **Integration Tests:** In tests/integration/ or with build tags
- **Test Framework:** testing (stdlib) + testify for assertions
- **Mocking:** Interfaces + mock implementations (or testify/mock)
- **Coverage Target:** 70%+ (measured with go test -cover)

### Test Naming Convention

`TestServiceName_MethodName_Scenario` — e.g., `TestUserService_GetByID_ReturnsUser`

## Quality Gates

```bash
# Linter
golangci-lint run ./...

# Formatter check
gofmt -l .

# Vet
go vet ./...

# Tests
go test ./... -race -cover

# Build
go build ./...
```

## Documentation

### Required Documentation Files

| File | Purpose |
|------|---------|
| README.md | Project overview, setup, API endpoints |
| CHANGELOG.md | Version history |

### Documentation Style

- **Code Comments:** Godoc format — comment on exported types and functions
- **Package Comments:** Package-level comment in doc.go or first file

## Deployment

- **Target Environment:** Docker / Kubernetes / cloud provider
- **CI/CD:** GitHub Actions
- **Branch Strategy:** main + feature branches

## Constraints & Non-Negotiables

- All errors must be handled explicitly — no ignored error returns
- All exported types and functions must have Godoc comments
- No package-level mutable state (global vars)
- Interfaces defined by the consumer, not the implementer
- Context must be passed through the call chain (first parameter)
- Race detector must pass (`go test -race`)
