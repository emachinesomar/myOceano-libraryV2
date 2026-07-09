# Estado del Sistema — Ocean Library v2

> Descripción técnica de lo que el sistema hace actualmente.
> Versión: 0.1.0 | Última actualización: 2026-07-09

---

## 🏗️ Stack tecnológico

### Backend (Rust)
| Componente | Tecnología | Versión |
|------------|-----------|---------|
| Runtime | Tauri | v2 |
| Base de datos | SQLite + FTS5 | rusqlite 0.31 |
| Tokenizador FTS | unicode61 | remove_diacritics=2 |
| Extracción PDF | pdf-extract | 0.7 |
| Frontmatter | gray_matter | 0.2 |
| Walkdir | walkdir | 2 |
| Async runtime | tokio | 1.x |

### Frontend (Svelte)
| Componente | Tecnología | Versión |
|------------|-----------|---------|
| Framework | Svelte 5 | 5.33+ |
| Router | SvelteKit | 2.16+ |
| Build | Vite | 6.3+ |
| CSS | Tailwind CSS | 3.4+ |
| Markdown | marked | 18.x |
| Adapter | adapter-static | 3.x |

---

## 📁 Estructura del proyecto

```
myOceano-library/
├── src-tauri/
│   ├── src/
│   │   ├── main.rs          # Entry point
│   │   ├── lib.rs           # 11 comandos Tauri + tree builder
│   │   ├── db.rs            # SQLite CRUD, FTS5, búsquedas
│   │   ├── parser.rs        # Inferencia de metadata (4 patrones)
│   │   └── indexer.rs       # Escaneo + indexación de archivos
│   ├── migrations/
│   │   └── 001_init.sql     # Schema FTS5
│   └── capabilities/
│       └── default.json     # Permisos Tauri
├── src/
│   ├── lib/
│   │   ├── components/
│   │   │   ├── AppSidebar.svelte       # Sidebar con árbol
│   │   │   ├── DocumentTree.svelte     # Árbol jerárquico
│   │   │   ├── TreeItem.svelte         # Nodo recursivo del árbol
│   │   │   ├── DocumentViewer.svelte   # Lector de documentos
│   │   │   ├── SearchCommand.svelte    # Búsqueda manual
│   │   │   ├── EditMetadataModal.svelte # Editor de metadata
│   │   │   └── Toast.svelte            # Notificaciones
│   │   ├── stores/
│   │   │   ├── document.ts             # Estado del documento seleccionado
│   │   │   └── toast.ts               # Estado de notificaciones
│   │   ├── tauri.ts                    # Wrappers para comandos Tauri
│   │   ├── types.ts                    # Tipos TypeScript
│   │   └── utils.ts                    # Helper cn()
│   └── routes/
│       ├── +layout.svelte              # Layout principal (sidebar + topbar)
│       └── +page.svelte                # Página principal
└── samples/                            # Documentos de prueba
```

---

## 🔧 Comandos Tauri disponibles

| Comando | Descripción | Parámetros |
|---------|-------------|------------|
| `index_directory` | Indexa una carpeta completa | `path: String` |
| `search_documents` | Búsqueda FTS5 + LIKE en paths | `query: String, limit: i64` |
| `get_document_tree` | Obtiene árbol jerárquico | — |
| `read_document` | Lee el contenido completo | `path: String` |
| `clear_index` | Borra todo y recrea tablas | — |
| `delete_document` | Elimina un documento | `path: String` |
| `get_index_stats` | Estadísticas del índice | — |
| `get_fts_stats` | Stats del índice FTS5 | — |
| `update_document_metadata` | Actualiza metadata individual | `path, religion, book, chapter, title, author, language` |
| `get_document_metadata` | Obtiene metadata de un doc | `path: String` |
| `update_religion_bulk` | Renombra religión en lote | `old_religion, new_religion` |
| `update_book_bulk` | Renombra libro en lote | `religion, old_book, new_book` |

---

## 🗄️ Schema de base de datos

### Tabla `files`
```sql
id          INTEGER PRIMARY KEY AUTOINCREMENT
path        TEXT NOT NULL UNIQUE
filename    TEXT NOT NULL
extension   TEXT NOT NULL
size_bytes  INTEGER NOT NULL
mtime       TEXT NOT NULL
indexed_at  TEXT NOT NULL DEFAULT (datetime('now'))
```

### Tabla `document_metadata`
```sql
id          INTEGER PRIMARY KEY AUTOINCREMENT
file_id     INTEGER NOT NULL UNIQUE (FK → files.id)
religion    TEXT
book        TEXT
chapter     TEXT
verse       TEXT
title       TEXT
author      TEXT
language    TEXT
tags        TEXT (JSON array)
```

### Tabla virtual `documents_fts` (FTS5)
```sql
path, title, author, religion, book, body
tokenizer: unicode61 remove_diacritics 2
content: documents_content
```

### Tabla `documents_content` (contenido externo FTS5)
```sql
rowid  INTEGER PRIMARY KEY
path   TEXT NOT NULL
title  TEXT
author TEXT
religion TEXT
book   TEXT
body   TEXT NOT NULL
```

---

## 🔍 Sistema de inferencia de metadata

### Patrón 0: Instituto Ruhí
- **Patrón:** `Libro X Unidad Y_...` o `Libro X_...`
- **Resultado:** `religion="Fe bahá'í"`, `book="Instituto Ruhí"`, `chapter="Libro X"`, `title=unidad`
- **Ejemplo:** `Libro 10 Unidad 1_ Acompañarse..._ocr.pdf`

### Patrón 1: Ridván / CUJ / prefijo de fecha
- **Ridván:** `Mensaje de Ridván XXX (YYYY) (CAST)` → `book="Ridván"`, `chapter=YYYY`
- **CUJ:** `YYMMDD CUJ...` → `book="CUJ"`, `chapter=año`
- **Fecha:** `YYMMDD Descripción (CAST)` → `book=descripción`, `chapter=año`

### Patrón 2: Palabras clave Baha'íes (40+ keywords)
- Personajes: baha, bahai, bahá, baha'u'llah, cristo, shoghi, abdul-baha, el báb, mazal
- Instituciones: cie, centro internacional, guardian, casa universal, justicia universal
- Textos: kitáb-i-aqdas, kitáb-i-íqán, gleanings, compendium, world order
- Prácticas: naw-ruz, ridvan, fast, feast, holy day, declaration
- Comunidad: pioneering, study circle, devotional, junior youth
- **Resultado:** `religion="Fe bahá'í"`, `book="Libros"`

### Patrón 3: Carpetas conocidas
- `ruhi/` → Fe bahá'í / Instituto Ruhí
- `islam/` → Islam / Corán
- `cristianismo/` → Cristianismo / Biblia
- `judaismo/` → Judaísmo / Torá
- `hinduismo/` → Hinduismo / Bhagavad Gita
- `budismo/` → Budismo / Dhammapada

---

## 🌳 Estructura del árbol

```
Religión
├── Instituto Ruhí (solo Fe bahá'í)
│   ├── Libro 1
│   │   ├── [documento1.pdf]
│   │   └── [documento2.pdf]
│   ├── Libro 10
│   │   ├── Libro 10 Unidad 1_...
│   │   └── Libro 10 Unidad 2_...
│   └── ...
├── Ridván
│   ├── Mensaje de Ridván 183 (2026) (CAST).pdf
│   └── ...
├── CUJ
│   ├── 101228 CUJ, carta a CCs...
│   └── ...
├── Mensaje a conferencia consejeros
│   └── 051227 Mensaje a conferencia consejeros CAST.pdf
└── Libros
    ├── LO-George-Townshend_Cristo_y_Bahaullah.pdf
    └── [otros documentos Baha'íes sueltos]
```

---

## 🎨 Interfaz de usuario

### Layout (sidebar-03)
- **Sidebar izquierda:** Árbol de navegación con conteo de documentos
- **Top bar:** Buscador con Ctrl+K, toggle de modo oscuro
- **Área principal:** Lector de documentos con Markdown renderizado

### Componentes
- **TreeItem:** Nodo recursivo con expand/collapse, hover para editar/eliminar
- **EditMetadataModal:** Modal con 3 modos (religión/libro/documento), presets por religión
- **SearchCommand:** Búsqueda manual con botón o Enter, muestra snippet + párrafo
- **DocumentViewer:** Renderiza Markdown con `marked`, scroll-to-top, snippet preview
- **Toast:** Notificaciones emergentes de éxito/error

### Atajos de teclado
- `Ctrl+K` → Abrir buscador
- `Escape` → Cerrar modales/buscador

---

## 🧪 Tests (18 total)

### Rust (17 tests)
- 7× Parser: inferencia de metadata (Ridván, CUJ, Ruhí, carpetas, todos los samples)
- 7× DB: sanitización de queries FTS5
- 1× DB: insert + search
- 1× DB: extract_paragraph
- 1× Parser: todos los archivos de samples

### Cobertura
- ✅ Inferencia de metadata para 42 archivos de prueba
- ✅ Búsqueda FTS5 con sanitización
- ✅ CRUD completo de documentos
- ✅ Árbol jerárquico con 3 niveles

---

## 📊 Datos de prueba actuales

| Categoría | Cantidad | Ejemplo |
|-----------|----------|---------|
| Instituto Ruhí | 31 archivos | Libro 1 a Libro 14 |
| Ridván | 5 archivos | 2022-2026 |
| CUJ | 5 archivos | 2010-2026 |
| Mensaje a conferencia | 1 archivo | 2005 |
| Libros (otros) | 1 archivo | Cristo y Baha'u'lláh |
| Sin clasificar | 2 archivos | Fechas ambiguas |
| **Total** | **45 archivos** | |

---

## 🚀 Cómo ejecutar

```bash
# Instalar dependencias
pnpm install

# Desarrollo
pnpm tauri dev

# Build producción
pnpm tauri build

# Tests Rust
cd src-tauri && cargo test

# Limpiar DB (si hay corrupción)
Remove-Item "$env:APPDATA\com.ocean.library.v2\ocean_library.db" -Force
# Luego re-indexar desde la app
```
