# Solicitudes del Usuario — Ocean Library v2

> Documento de peticiones y funcionalidades pendientes para futuras versiones.
> Última actualización: 2026-07-10

---

## 📋 Pendientes por implementar

### 🔍 Búsqueda avanzada
- [ ] Filtros por religión, libro, autor, idioma en la búsqueda
- [ ] Búsqueda por rango de fechas
- [ ] Búsqueda dentro de un libro específico (scope filtering)
- [ ] Autocompletado de términos de búsqueda
- [ ] Historial de búsquedas recientes
- [ ] Exportar resultados de búsqueda
- [ ] **Highlight de matches en el documento** — al abrir un doc desde search, scrollear al párrafo y resaltar todas las ocurrencias de la query
- [ ] **Toolbar de búsqueda inline** — Ctrl+F dentro del visor con navegación entre resultados

### 📖 Navegación del árbol
- [ ] Colapsar/expandir todos los nodos de una vez
- [ ] Ordenar nodos alfabéticamente o por fecha
- [ ] Filtros en el árbol (mostrar solo una religión/libro)
- [ ] Contador total de documentos en la barra lateral
- [ ] Búsqueda dentro del árbol (filtrar nodos)

### ✏️ Edición de metadata
- [ ] Editar metadata en lote (selección múltiple)
- [ ] Agregar tag personalizado a documentos
- [ ] Editor de YAML frontmatter inline
- [ ] Historial de cambios de metadata
- [ ] Deshacer último cambio de metadata

### 📄 Visualización de documentos
- [ ] **Renderer de PDF completo** — integrar PDF.js para ver el PDF original con imágenes, tablas, formato
- [ ] **Renderizado de páginas como imágenes** — usar PyMuPDF para convertir páginas a PNG y mostrarlas inline
- [ ] **Zoom** en el visor de documentos
- [ ] **Selección de texto + copia** en el PDF renderizado
- [ ] Modo lectura con fuente personalizable (tamaño, familia)
- [ ] Modo oscuro/claro para el lector
- [ ] Navegación por capítulos dentro del documento
- [ ] Compartir fragmento de texto
- [ ] Imprimir documento

### 🖍️ Anotaciones y marcado (nuevo)
- [ ] **Highlight de texto** — seleccionar y resaltar pasajes con color
- [ ] **Notas al margen** — agregar notas personales a páginas/paragraphos específicos
- [ ] **Subrayado y tachado** — herramientas de marcado de texto
- [ ] **Dibujo libre** — rayar/dibujar sobre el PDF (como en visores de PDF con lápiz)
- [ ] **Persistencia de anotaciones** — guardar en SQLite (página, tipo, coordenadas, contenido, color)
- [ ] **Toggle de anotaciones** — mostrar/ocultar todas las anotaciones
- [ ] **Exportar PDF con anotaciones** — PDF anotado para compartir

### 📁 Gestión de documentos
- [ ] Agregar documentos individuales (no solo carpetas)
- [ ] Mover documentos entre carpetas
- [ ] Duplicar documento
- [ ] Verificar integridad del índice (reparar archivos faltantes)
- [ ] Backup/restore de la base de datos
- [ ] Historial de versiones de documentos

### 🏷️ Organización
- [ ] Sistema de etiquetas (tags) jerárquico
- [ ] Colecciones/playlist de documentos
- [ ] Favoritos / marcadores
- [ ] Lectura reciente
- [ ] Notas personales por documento

### 🌐 Multi-idioma
- [ ] Interfaz en español, inglés, árabe, persa
- [ ] Detección automática de idioma del documento
- [ ] Soporte para documentos en hebreo/sánscrito (RTL)
- [ ] OCR para PDFs escaneados en múltiples idiomas

### 🖥️ Multi-plataforma
- [ ] Compilar para macOS
- [ ] Compilar para Linux (AppImage, .deb)
- [ ] Sincronización entre dispositivos (vía nube o red local)
- [ ] Versión web (acceso desde navegador)
- [ ] App móvil (iOS/Android) — lectura básica

### ⚙️ Configuración
- [ ] Configuración de carpeta de índices (múltiples fuentes)
- [ ] Temas personalizables
- [ ] Atajos de teclado configurables
- [ ] Idioma de la interfaz seleccionable
- [ ] Tamaño de fuente del lector

### 🔧 Avanzado
- [ ] Índice automático al detectar cambios en la carpeta (watcher)
- [ ] Exportar a PDF/EPUB
- [ ] Integración con citation managers (Zotero, Mendeley)
- [ ] API REST para acceso externo
- [ ] Plugin system para procesadores de documentos personalizados
- [ ] Soporte para más formatos: DOCX, EPUB, FB2

---

## 🐛 Bugs conocidos
- [ ] Los archivos con solo fecha en el nombre ("4 de enero de 2026", "4 ene 2022") no se clasifican automáticamente
- [ ] DB corrupta si se interrumpe la indexación a mitad
- [ ] PDFs con cross-reference table corrupta requieren `pip install PyMuPDF` para indexarse

### Fixed
- ~~`window.confirm()` bloqueado en Tauri webview~~ — ✅ **Funciona bien en Tauri webview**, se usa para confirmar eliminación

---

## 💡 Ideas futuras
- Modo presentación para estudios grupales
- Integración con calendario Baha'i (días santos, ayuno, etc.)
- Estadísticas de lectura (tiempo, documentos leídos, progreso)
- Modo offline-first con sincronización posterior
- IA para resúmenes automáticos de documentos
- Detección de duplicados por contenido (no solo nombre)

---

## 🎯 Prioridades discutidas (sesión 2026-07-10)

1. **Short-term:** Highlight de matches de búsqueda en el documento (scroll + `<mark>`)
2. **Medium-term:** Visor de PDF completo (PDF.js o render vía PyMuPDF)
3. **Long-term:** Anotaciones (highlights, notas, dibujo) sobre el PDF
