# Skill Registry — Ocean Library v2

## Project Context

| Field | Value |
|-------|-------|
| Project | ocean-library-v2 |
| Path | C:\Users\Omar\Documents\Dev\myOceano-library |
| Git | https://github.com/emachinesomar/myOceano-libraryV2.git |
| Stack | Tauri v2 (Rust) + Svelte 5 (Runes) + SQLite FTS5 + Tailwind CSS 3 |
| Language | TypeScript (strict) + Rust (2021 edition) |
| Build | Vite 6 + Tauri CLI 2 |
| Adapter | @sveltejs/adapter-static |

## Installed Skills (from ~/.config/opencode/skills/)

| Skill | Trigger | Relevance |
|-------|---------|-----------|
| typescript | TypeScript code — types, interfaces, generics | HIGH — all frontend code |
| tailwind-4 | Tailwind CSS styling — cn(), theme variables | HIGH — all .svelte components |
| sdd-apply | SDD implementation phase | MEDIUM — when implementing tasks |
| sdd-verify | SDD verification phase | MEDIUM — when verifying changes |
| sdd-explore | SDD exploration phase | MEDIUM — when exploring ideas |
| sdd-spec | SDD specification phase | MEDIUM — when writing specs |
| sdd-tasks | SDD task planning | MEDIUM — when breaking down work |
| sdd-design | SDD technical design | MEDIUM — when designing architecture |
| sdd-propose | SDD change proposals | MEDIUM — when proposing changes |
| sdd-archive | SDD archiving | LOW — when closing changes |
| skill-creator | Creating new skills | LOW — for skill development |
| skill-registry | Updating this registry | LOW — after skill changes |

## Compact Rules

### typescript
- Use `strict: true` (already configured in tsconfig.json).
- Prefer `interface` for object shapes, `type` for unions/intersections.
- Use `satisfies` operator for type-safe object literals.
- Avoid `any` — use `unknown` and narrow with type guards.
- Use template literal types for string patterns.

### tailwind-4
- Use `cn()` helper from `$lib/utils` (clsx + tailwind-merge).
- Use CSS variables for theming (e.g., `hsl(var(--primary))`).
- Prefer `@apply` sparingly — inline Tailwind classes are preferred.
- Use `tailwindcss-animate` for animations.

## User Skills (code context triggers)

| File Pattern | Matching Skills |
|-------------|-----------------|
| `*.ts` | typescript |
| `*.svelte` | typescript, tailwind-4 |
| `*.rs` | (none — Rust has no dedicated skill yet) |
| `*.sql` | (none) |
