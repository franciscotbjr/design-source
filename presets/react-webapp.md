# Project Definition — React Web Application

> Pre-filled preset for a React + TypeScript web application. Customize for your project.

---

## Project Identity

- **Project Name:** [your-app-name]
- **Description:** [One-sentence description]
- **Project Type:** web app
- **Repository URL:** [https://github.com/user/repo]
- **License:** MIT

## Technology Stack

### Language(s)

| Language | Version | Role |
|----------|---------|------|
| TypeScript | 5.x | Primary |

### Framework(s)

| Framework | Version | Purpose |
|-----------|---------|---------|
| React | 18.x / 19.x | UI framework |
| Vite | 5.x | Build tool and dev server |

### Key Dependencies

| Dependency | Version | Purpose |
|------------|---------|---------|
| react-router | 6.x | Client-side routing |
| tailwindcss | 3.x / 4.x | Utility-first CSS |
| tanstack/react-query | 5.x | Server state management |
| zustand | 4.x | Client state management |
| zod | 3.x | Runtime validation |
| lucide-react | latest | Icon library |

### Build System & Package Manager

- **Package Manager:** npm (or pnpm)
- **Build Tool:** Vite
- **Task Runner:** npm scripts

## Repository Structure

```
project/
├── src/
│   ├── main.tsx              # Entry point
│   ├── App.tsx               # Root component
│   ├── components/           # Reusable UI components
│   │   ├── ui/               # Base UI components (buttons, inputs, etc.)
│   │   └── [feature]/        # Feature-specific components
│   ├── pages/                # Route page components
│   ├── hooks/                # Custom React hooks
│   ├── services/             # API client and external service wrappers
│   ├── stores/               # Zustand stores
│   ├── types/                # Shared TypeScript types
│   ├── utils/                # Utility functions
│   └── lib/                  # Third-party library configs
├── public/
├── tests/
│   ├── unit/
│   └── e2e/
├── package.json
├── tsconfig.json
├── vite.config.ts
└── tailwind.config.ts
```

### Key Directories

| Directory | Purpose |
|-----------|---------|
| src/components/ | Reusable UI components |
| src/pages/ | Top-level route components |
| src/hooks/ | Custom React hooks |
| src/services/ | API communication layer |

## Code Conventions

### Naming

| Item | Convention | Example |
|------|-----------|---------|
| Component files | PascalCase.tsx | UserProfile.tsx |
| Hook files | camelCase.ts | useUserProfile.ts |
| Utility files | camelCase.ts | formatDate.ts |
| Components | PascalCase | UserProfile |
| Functions/Hooks | camelCase | useUserProfile |
| Constants | SCREAMING_SNAKE_CASE | API_BASE_URL |
| CSS classes | Tailwind utilities | className="flex items-center" |

### Code Style

- **Formatter:** prettier
- **Formatter Config:** .prettierrc (semi: true, singleQuote: true, jsxSingleQuote: false)
- **Max Line Length:** 100
- **Indentation:** 2 spaces
- **Import Order:** react → external packages → internal modules → types → styles

### Patterns & Conventions

- **Error Handling:** Error boundaries for React errors, try/catch in async operations, toast notifications for user-facing errors
- **State Management:** Server state via React Query, client state via Zustand, local state via useState
- **API Layer:** Centralized API client in services/, React Query for data fetching
- **Component Design:** Composition over inheritance, props for customization, children for slot content
- **Styling:** Tailwind CSS utility classes, no CSS modules or styled-components

## Testing

### Strategy

- **Unit Tests:** Vitest for logic, React Testing Library for components
- **E2E Tests:** Playwright (optional)
- **Test Framework:** Vitest
- **Mocking:** vi.mock for modules, MSW for API mocking
- **Coverage Target:** No formal target; test user-visible behavior

### Test Naming Convention

`describe('UserProfile', () => { it('renders user name', ...) })`

## Quality Gates

```bash
# Linter
npx eslint .

# Formatter check
npx prettier --check .

# Type check
npx tsc --noEmit

# Tests
npx vitest run

# Build
npx vite build
```

## Documentation

### Required Documentation Files

| File | Purpose |
|------|---------|
| README.md | Project overview, setup, dev workflow |
| CHANGELOG.md | Version history |

### Documentation Style

- **Code Comments:** JSDoc for utility functions and hooks, inline comments for complex logic
- **Component Docs:** Props documented via TypeScript interfaces

## Deployment

- **Target Environment:** Vercel / Netlify / static hosting
- **CI/CD:** GitHub Actions
- **Branch Strategy:** main + feature branches

## Constraints & Non-Negotiables

- No `any` type without explicit justification
- All components must be accessible (keyboard navigable, proper ARIA)
- No direct API calls outside of services/ layer
- All user inputs must be validated before submission
- Bundle size must be monitored — no unnecessary large dependencies
