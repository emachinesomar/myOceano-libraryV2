# Estado del Sistema — Ocean Library v2

> Descripción técnica de lo que el sistema hace actualmente.
> Versión: 0.1.0 | Última actualización: 2026-07-10

---

## 🏗️ Stack tecnológico

### Backend (Rust)
| Componente | Tecnología | Versión |
|------------|-----------|---------|
| Runtime | Tauri | v2 |
| Base de datos | SQLite + FTS5 | rusqlite 0.31 |
| Tokenizador FTS | unicode61 | remove_diacritics=2 |
| Extracción PDF (primario) | pdf-extract | 0.7 |
| Extracción PDF (fallback) | PyMuPDF via Python | fitz 1.28 |
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
│   │   ├── lib.rs           # 12 comandos Tauri + tree builder
│   │   ├── db.rs            # SQLite CRUD, FTS5, búsquedas
│   │   ├── parser.rs        # Inferencia de metadata (4 patrones)
│   │   └── indexer.rs       # Escaneo + indexación + fallback PyMuPDF
│   ├── scripts/
│   │   └── pymupdf_extract.py # Fallback Python para PDFs corruptos
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
│   │   │   ├── EditMetadataModal.svelte # Editor de metadata (3 modos)
│   │   │   └── Toast.svelte            # Notificaciones
│   │   ├── stores/
│   │   │   ├── document.ts             # Estado del documento seleccionado
│   │   │   └── toast.ts               # Estado de notificaciones
│   │   ├── tauri.ts                    # Wrappers para 12 comandos Tauri
│   │   ├── types.ts                    # Tipos TypeScript (TreeNode con religion)
│   │   └── utils.ts                    # Helper cn()
│   └── routes/
│       ├── +layout.svelte              # Layout principal (sidebar + topbar)
│       └── +page.svelte                # Página principal
├── ESTADO_SISTEMA.md
├── SOLICITUDES_USUARIO.md
└── samples/                            # Documentos de prueba
```

---

## 🔧 Comandos Tauri disponibles (12)

| Comando | Descripción | Parámetros |
|---------|-------------|------------|
| `index_directory` | Indexa una carpeta completa (con fallback PyMuPDF) | `path: String` |
| `search_documents` | Búsqueda FTS5 + LIKE en paths | `query: String, limit: i64` |
| `get_document_tree` | Obtiene árbol jerárquico con `religion` en book nodes | — |
| `read_document` | Lee el contenido completo de un documento | `path: String` |
| `clear_index` | Borra todo (DROP + CREATE para FTS5 externo) | — |
| `delete_document` | Elimina un documento (FTS → content → metadata → files) | `path: String` |
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
    ├── LB-El_Kitab-i-Aqdas.pdf
    └── [otros documentos Baha'íes sueltos]
```

### Tipos de nodo
| Tipo | Icono | Descripción |
|------|-------|-------------|
| religion | 🌍 | Primer nivel (Fe bahá'í, Islam, etc.) |
| book | 📘 | Segundo nivel (Ridván, CUJ, Libros, etc.) |
| chapter | 📁 | Solo Instituto Ruhí — agrupa por Libro X |
| document | 📄 | Hoja individual del PDF |

---

## 🎨 Interfaz de usuario

### Layout (sidebar-03)
- **Sidebar izquierda:** Árbol de navegación con conteo de documentos y botón de borrar DB
- **Top bar:** Buscador con Ctrl+K, toggle de modo oscuro
- **Área principal:** Lector de documentos con Markdown renderizado

### Componentes
- **TreeItem:** Nodo recursivo con expand/collapse, iconos por tipo, hover para editar/eliminar
- **EditMetadataModal:** Modal con 3 modos (religión→bulk rename, libro→bulk rename, documento→full form)
- **SearchCommand:** Búsqueda manual con botón o Enter, muestra snippet + párrafo via UNION query
- **DocumentViewer:** Renderiza Markdown con `marked`, scroll-to-top
- **Toast:** Notificaciones emergentes de éxito/error

### Atajos de teclado
- `Ctrl+K` → Abrir buscador
- `Escape` → Cerrar modales/buscador

---

## 🧪 Tests (18 total — todos pasando)

### Rust (18 tests)
| Módulo | Tests | Lo que cubren |
|--------|-------|---------------|
| Parser | 7 | Inferencia: Ridván, CUJ, Ruhí, carpetas, keywords |
| Parser | 1 | `test_infer_all_sample_files` — todos los PDFs de samples |
| Parser | 1 | `test_infer_baha_book_libros` — 40+ keywords Baha'íes |
| DB | 1 | Insert + search + tree |
| DB | 1 | FTS5 sanity check |
| DB | 1 | extract_paragraph |
| DB | 6 | sanitize_fts_query (varios casos) |

### Cobertura
- ✅ Inferencia de metadata para 45+ archivos de prueba
- ✅ Búsqueda FTS5 con sanitización (comillas, AND/OR/NOT)
- ✅ CRUD completo de documentos + bulk update
- ✅ Árbol jerárquico con 4 tipos de nodo
- ✅ PDFs corruptos → fallback PyMuPDF

---

## 📊 Datos de prueba actuales

| Categoría | Cantidad | Ejemplo |
|-----------|----------|---------|
| Instituto Ruhí | 31 archivos | Libro 1 a Libro 14 (múltiples unidades) |
| Ridván | 5 archivos | 2022-2026 |
| CUJ | 5 archivos | 2010-2026 |
| Mensaje a conferencia | 1 archivo | 2005 |
| Libros (otros) | 2 archivos | Cristo y Bahá'u'lláh, Kitáb-i-Aqdas |
| Sin clasificar | 2 archivos | Fechas ambiguas |
| **Total** | **~46 archivos** | |

---

## 📝 Notas técnicas importantes

### FTS5 externo no soporta DELETE
- `DELETE FROM documents_fts` no funciona con tablas de contenido externo
- `clear_all()` usa DROP TABLE + recreación del schema completo
- `delete_document()` elimina en orden: FTS rebuild → content → metadata → files

### Fallback PyMuPDF
- `pdf_extract` falla en PDFs con tabla cross-reference corrupta
- El indexer intenta `pdf-extract` primero, si falla llama a `scripts/pymupdf_extract.py` via Python
- Si Python no está disponible o PyMuPDF no está instalado, indexa con body vacío
- Requiere: `pip install PyMuPDF`

### `confirm()` en Tauri
- `window.confirm()` SÍ funciona en Tauri webview (contrario a lo que se documentó antes)
- Se usa para confirmar eliminación de documentos

---

## 🚀 Cómo ejecutar

```bash
# Instalar dependencias
pnpm install
pip install PyMuPDF   # Para fallback de PDFs corruptos

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
