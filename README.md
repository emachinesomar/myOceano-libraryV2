# 🌊 Ocean Library V2

A modern library application built with **SvelteKit**, **Tauri**, and **TailwindCSS**. Designed for managing and discovering ocean-related resources with a beautiful, responsive interface.

## 🚀 Tech Stack

- **Frontend**: SvelteKit 2.16 + Svelte 5.33
- **Desktop**: Tauri 2.5 (Rust-based native wrapper)
- **Styling**: TailwindCSS 3.4 + Autoprefixer
- **Utilities**: clsx, tailwindcss-merge, tailwindcss-animate
- **Markdown**: Marked 18.0
- **Runtime**: Node.js with pnpm

## 📋 Prerequisites

- **Node.js** (v18+)
- **pnpm** (recommended) or npm
- **Rust** (only if running desktop with Tauri)

## 🔧 Installation

```bash
# Install dependencies
pnpm install
```

The project uses `pnpm-lock.yaml` for deterministic dependency resolution.

## ▶️ Development

### Web Mode (SvelteKit)
```bash
npm run dev
```
Opens at `http://localhost:5173` by default.

### Desktop Mode (Tauri)
Requires Rust toolchain. First time setup:
```bash
rustup update
```

Then run:
```bash
npm run tauri dev
```

## 🏗️ Building

### For Web
```bash
npm run build
```
Creates static output in `.svelte-kit/` ready for deployment.

### For Desktop (Tauri)
```bash
npm run tauri build
```
Outputs native binaries for macOS, Windows, or Linux.

## 👁️ Preview

To preview the production build locally:
```bash
npm run preview
```

## 📁 Project Structure

```
.
├── src/                    # Svelte components & pages
├── src-tauri/              # Tauri configuration (Rust backend)
├── static/                 # Static assets
├── svelte.config.js        # SvelteKit config
├── vite.config.ts          # Vite bundler config
├── tailwind.config.ts      # TailwindCSS config
├── tsconfig.json           # TypeScript config
├── postcss.config.js       # PostCSS config
└── pnpm-lock.yaml         # Dependency lock file
```

## 📦 Key Dependencies

| Package | Purpose |
|---------|---------|
| `@tauri-apps/api` | Tauri JS API for desktop integration |
| `@tauri-apps/plugin-fs` | File system operations |
| `@tauri-apps/plugin-dialog` | Native dialogs (open/save files) |
| `marked` | Markdown parsing & rendering |
| `@sveltejs/adapter-static` | Static site adapter for production |

## 🤝 Contributing

1. Create a feature branch
2. Make changes
3. Test with `npm run dev`
4. Build to verify: `npm run build`

## 📝 License

See LICENSE file for details.

---

**Quick Start**:
```bash
pnpm install && npm run dev
```

**Need Tauri desktop app?**:
```bash
npm run tauri dev
```
