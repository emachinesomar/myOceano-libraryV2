use gray_matter::engine::YAML;
use gray_matter::Matter;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Metadata extracted from YAML frontmatter in Markdown files.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DocumentMetadata {
    pub religion: Option<String>,
    pub book: Option<String>,
    pub chapter: Option<String>,
    pub verse: Option<String>,
    pub title: Option<String>,
    pub author: Option<String>,
    pub language: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
}

/// Parsed document: metadata + body text.
#[derive(Debug, Clone)]
pub struct ParsedDocument {
    pub metadata: DocumentMetadata,
    pub body: String,
}

/// Parse Markdown content string: separate frontmatter from body.
pub fn parse_markdown_content(content: &str) -> Result<ParsedDocument, String> {
    let matter = Matter::<YAML>::new();
    let parsed = matter.parse(content);

    let metadata = if let Some(data) = &parsed.data {
        // Use gray_matter's built-in deserialization into our struct
        data.deserialize::<DocumentMetadata>()
            .unwrap_or_default()
    } else {
        DocumentMetadata::default()
    };

    let body = parsed.content.trim().to_string();

    Ok(ParsedDocument { metadata, body })
}

/// Infer metadata from file path when no frontmatter is present.
/// Handles Baha'i document patterns and generic religion folder structures.
pub fn infer_metadata_from_path(path: &Path) -> DocumentMetadata {
    let mut meta = DocumentMetadata::default();

    let _path_str = path.to_string_lossy().to_lowercase();

    // Pattern 0: Ruhí Institute books — "Libro X Unidad Y_..."
    if let Some(filename) = path.file_stem().and_then(|s| s.to_str()) {
        if let Some(ruhi_meta) = infer_ruhi_metadata(filename) {
            meta.religion = Some("Fe bahá'í".to_string());
            meta.book = Some("Instituto Ruhí".to_string());
            meta.chapter = ruhi_meta.0; // "Libro X"
            meta.title = ruhi_meta.1;   // unit title
            meta.language = Some("Castellano".to_string());
            return meta;
        }
    }

    // Pattern 1: Baha'i filename patterns (Ridván, CUJ, date prefix)
    if let Some(filename) = path.file_stem().and_then(|s| s.to_str()) {
        if let Some(baha_meta) = infer_baha_metadata(filename) {
            meta.religion = Some("Fe bahá'í".to_string());
            meta.book = baha_meta.0; // message type: Ridván, CUJ, etc.
            meta.chapter = baha_meta.1; // year
            meta.title = Some(filename.to_string());
            meta.language = Some("Castellano".to_string());
            return meta;
        }
    }

    // Pattern 2: Baha'i books with known keywords (Cristo, Baha'u'lláh, CIE, etc.)
    if let Some(filename) = path.file_stem().and_then(|s| s.to_str()) {
        let lower = filename.to_lowercase();
        let baha_keywords = [
            // Figures
            "baha", "bahai", "bahá", "baha'u'llah", "bahaullah",
            "cristo", "shoghi", "abdul-baha", "ábdu'l-bahá",
            "the báb", "el báb", "mazal",
            // Institutions
            "cie", "centro internacional", "guardian",
            "casa universal", "justicia universal",
            "casa de justicia", "hands of the cause",
            // Key texts
            "kitáb-i-aqdas", "kitáb-i-íqán", "aqdas", "íqán",
            "gleanings", "compendium", "world order",
            "divine plan", "crusade", "tablets",
            "tablet of ahmad", "lawh-i-akbar",
            // Practices & events
            "naw-ruz", "ridvan", "fast", "feast",
            "holy day", "declaration", "ascension",
            "birth", "martyrdom", "intercalation",
            "badí'", "sacred writings",
            // Community
            "pioneering", "study circle", "devotional",
            "junior youth", "children's class",
            "community building", "growth",
        ];
        if baha_keywords.iter().any(|kw| lower.contains(kw)) {
            meta.religion = Some("Fe bahá'í".to_string());
            meta.book = Some("Libros".to_string());
            meta.title = Some(filename.to_string());
            meta.language = Some("Castellano".to_string());
            return meta;
        }
    }

    // Pattern 2: Detect religion from folder path components
    let components: Vec<&str> = path
        .components()
        .filter_map(|c| c.as_os_str().to_str())
        .collect();

    // Map folder names to religion + book
    let folder_map = [
        ("ruhi",          "Fe bahá'í",  "Instituto Ruhí"),
        ("bahaismo",      "Fe bahá'í",  ""),
        ("bahai",         "Fe bahá'í",  ""),
        ("islam",         "Islam",      "Corán"),
        ("cristianismo",  "Cristianismo", "Biblia"),
        ("judaismo",      "Judaísmo",   "Torá"),
        ("hinduismo",     "Hinduismo",  "Bhagavad Gita"),
        ("budismo",       "Budismo",    "Dhammapada"),
        ("christianity",  "Cristianismo", "Biblia"),
        ("judaism",       "Judaísmo",   "Torá"),
        ("islam",         "Islam",      "Corán"),
    ];

    for (i, component) in components.iter().enumerate() {
        let lower = component.to_lowercase();
        for (folder_name, religion, book) in &folder_map {
            if lower.contains(folder_name) {
                meta.religion = Some(religion.to_string());
                if !book.is_empty() {
                    meta.book = Some(book.to_string());
                } else if i + 1 < components.len() {
                    meta.book = Some(components[i + 1].to_string());
                }
                break;
            }
        }
        if meta.religion.is_some() {
            break;
        }
    }

    // Always set title from filename
    if let Some(filename) = path.file_stem().and_then(|s| s.to_str()) {
        if meta.title.is_none() {
            meta.title = Some(filename.to_string());
        }
    }

    meta
}

/// Infer Ruhí Institute metadata from a filename.
/// Returns (book_number, unit_title) if recognized, None otherwise.
///
/// Patterns:
/// - "Libro 10 Unidad 1_ Acompañarse..." → ("Libro 10", "Acompañarse...")
/// - "Libro 1 Reflexiones..." → ("Libro 1", "Reflexiones...")
/// - "Libro 3 Grado 3 Unidad 1_..." → ("Libro 3", "Grado 3 Unidad 1...")
fn infer_ruhi_metadata(filename: &str) -> Option<(Option<String>, Option<String>)> {
    let lower = filename.to_lowercase();

    // Must contain "libro" to be a Ruhí book
    if !lower.contains("libro") {
        return None;
    }

    // Extract book number: "Libro X"
    let re_book = Regex::new(r"(?i)libro\s+(\d+)").ok()?;
    let book_num = re_book.captures(filename)?.get(1)?.as_str();

    let chapter = Some(format!("Libro {}", book_num));

    // Extract unit/grade info after book number for the title
    // Patterns: "Unidad X_", "Grado X Unidad Y_", or just the rest
    let re_unit = Regex::new(r"(?i)libro\s+\d+\s*(.*)").ok()?;
    let title_suffix = if let Some(caps) = re_unit.captures(filename) {
        let rest = caps.get(1)?.as_str().trim();
        // Clean up: remove "_ocr.pdf" suffix and leading separators
        let cleaned = rest
            .trim_start_matches('_')
            .trim_start_matches(' ')
            .replace("_ocr", "")
            .replace("_", " ")
            .trim()
            .to_string();
        if cleaned.is_empty() {
            None
        } else {
            Some(cleaned)
        }
    } else {
        None
    };

    Some((chapter, title_suffix))
}

/// Infer Baha'i metadata from a filename.
/// Returns (message_type, year) if recognized, None otherwise.
///
/// Patterns:
/// - "Mensaje de Ridván 183 (2026) (CAST)" → (Ridván, 2026)
/// - "051227 Mensaje a conferencia consejeros CAST" → (CUJ, 2005)
/// - "101228 CUJ, carta a CCs..." → (CUJ, 2010)
fn infer_baha_metadata(filename: &str) -> Option<(Option<String>, Option<String>)> {
    let lower = filename.to_lowercase();

    // Pattern 1: "Mensaje de Ridván ... (YYYY)"
    if lower.contains("ridvan") || lower.contains("ridván") {
        let re = Regex::new(r"\((\d{4})\)").ok()?;
        let year = re.captures(filename)?.get(1)?.as_str().to_string();
        return Some((Some("Ridván".to_string()), Some(year)));
    }

    // Pattern 2: "CUJ" in filename — extract year from 6-digit date prefix
    if lower.contains("cuj") {
        let re = Regex::new(r"^(\d{2})\d{4}\s").ok()?;
        if let Some(caps) = re.captures(filename) {
            let yy: u32 = caps.get(1)?.as_str().parse().ok()?;
            let year = if yy <= 50 {
                2000 + yy
            } else {
                1900 + yy
            };
            return Some((Some("CUJ".to_string()), Some(year.to_string())));
        }
        // CUJ without date prefix — try year in parentheses
        let re_year = Regex::new(r"\((\d{4})\)").ok()?;
        if let Some(caps) = re_year.captures(filename) {
            let year = caps.get(1)?.as_str().to_string();
            return Some((Some("CUJ".to_string()), Some(year)));
        }
    }

    // Pattern 3: 6-digit date prefix without CUJ keyword
    let re = Regex::new(r"^(\d{2})\d{4}\s+(.+?)\s+(CAST|ES|EN|AR)\b").ok()?;
    if let Some(caps) = re.captures(filename) {
        let yy: u32 = caps.get(1)?.as_str().parse().ok()?;
        let year = if yy <= 50 {
            2000 + yy
        } else {
            1900 + yy
        };
        let desc = caps.get(2)?.as_str().to_string();
        return Some((Some(desc), Some(year.to_string())));
    }

    None
}

/// Merge inferred metadata with frontmatter metadata.
pub fn merge_metadata(frontmatter: DocumentMetadata, inferred: DocumentMetadata) -> DocumentMetadata {
    DocumentMetadata {
        religion: frontmatter.religion.or(inferred.religion),
        book: frontmatter.book.or(inferred.book),
        chapter: frontmatter.chapter.or(inferred.chapter),
        verse: frontmatter.verse.or(inferred.verse),
        title: frontmatter.title.or(inferred.title),
        author: frontmatter.author.or(inferred.author),
        language: frontmatter.language.or(inferred.language),
        tags: if frontmatter.tags.is_empty() {
            inferred.tags
        } else {
            frontmatter.tags
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_markdown_with_frontmatter() {
        let content = r#"---
religion: Islam
book: Quran
chapter: "2"
verse: "255"
title: Ayat al-Kursi
author: ""
language: ar
tags:
  - throne
  - verse
---

بِسْمِ ٱللَّهِ ٱلرَّحْمَـٰنِ ٱلرَّحِيمِ

ٱللَّهُ لَآ إِلَـٰهَ إِلاَّ هُوَ ٱلْحَىُّ ٱلْقَيُّومُ..."#;

        let doc = parse_markdown_content(content).unwrap();
        assert_eq!(doc.metadata.religion.as_deref(), Some("Islam"));
        assert_eq!(doc.metadata.book.as_deref(), Some("Quran"));
        assert_eq!(doc.metadata.chapter.as_deref(), Some("2"));
        assert_eq!(doc.metadata.verse.as_deref(), Some("255"));
        assert!(doc.body.contains("ٱللَّهُ"));
        assert!(!doc.body.contains("---"));
    }

    #[test]
    fn test_parse_markdown_without_frontmatter() {
        let content = "# Salmo 23\n\nEl Señor es mi pastor...";
        let doc = parse_markdown_content(content).unwrap();
        assert!(doc.metadata.religion.is_none());
        assert!(doc.body.contains("El Señor es mi pastor"));
    }

    #[test]
    fn test_infer_baha_ridvan() {
        let path = Path::new("samples/Mensaje de Ridván 183 (2026) (CAST).pdf");
        let meta = infer_metadata_from_path(path);
        assert_eq!(meta.religion.as_deref(), Some("Fe bahá'í"));
        assert_eq!(meta.book.as_deref(), Some("Ridván"));
        assert_eq!(meta.chapter.as_deref(), Some("2026"));
    }

    #[test]
    fn test_infer_baha_date_prefix_without_cuj() {
        let path = Path::new("samples/051227 Mensaje a conferencia consejeros CAST.pdf");
        let meta = infer_metadata_from_path(path);
        assert_eq!(meta.religion.as_deref(), Some("Fe bahá'í"));
        assert_eq!(meta.book.as_deref(), Some("Mensaje a conferencia consejeros"));
        assert_eq!(meta.chapter.as_deref(), Some("2005"));
    }

    #[test]
    fn test_infer_baha_cuj_keyword() {
        let path = Path::new("samples/101228 CUJ, carta a CCs delineando nuevo Plan de 5 Años CAST.pdf");
        let meta = infer_metadata_from_path(path);
        assert_eq!(meta.religion.as_deref(), Some("Fe bahá'í"));
        assert_eq!(meta.book.as_deref(), Some("CUJ"));
        assert_eq!(meta.chapter.as_deref(), Some("2010"));
    }

    #[test]
    fn test_infer_ruhi_book() {
        let path = Path::new("samples/ruhi/Libro 10 Unidad 1_ Acompañarse unos a otros en el sendero del servicio_ocr.pdf");
        let meta = infer_metadata_from_path(path);
        assert_eq!(meta.religion.as_deref(), Some("Fe bahá'í"));
        assert_eq!(meta.book.as_deref(), Some("Instituto Ruhí"));
        assert_eq!(meta.chapter.as_deref(), Some("Libro 10"));
        assert!(meta.title.unwrap().contains("Acompañarse"));
    }

    #[test]
    fn test_infer_ruhi_book_simple() {
        let path = Path::new("samples/ruhi/Libro 1 Reflexiones sobre la vida del espíritu_ocr.pdf");
        let meta = infer_metadata_from_path(path);
        assert_eq!(meta.religion.as_deref(), Some("Fe bahá'í"));
        assert_eq!(meta.book.as_deref(), Some("Instituto Ruhí"));
        assert_eq!(meta.chapter.as_deref(), Some("Libro 1"));
        assert!(meta.title.unwrap().contains("Reflexiones"));
    }

    #[test]
    fn test_infer_baha_book_libros() {
        let path = Path::new("samples/LO-George-Townshend_Cristo_y_Bahaullah.pdf");
        let meta = infer_metadata_from_path(path);
        assert_eq!(meta.religion.as_deref(), Some("Fe bahá'í"));
        assert_eq!(meta.book.as_deref(), Some("Libros"));
        assert!(meta.title.unwrap().contains("Cristo"));
    }

    #[test]
    fn test_infer_from_folder_path() {
        let path = Path::new("library/islam/quran/chapter1.md");
        let meta = infer_metadata_from_path(path);
        assert_eq!(meta.religion.as_deref(), Some("Islam"));
        assert_eq!(meta.book.as_deref(), Some("Corán"));
    }

    #[test]
    fn test_infer_all_sample_files() {
        let samples_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("samples");

        if !samples_dir.exists() {
            return; // Skip if samples not present
        }

        // Files that MUST be classified as Fe bahá'í
        let must_classify = [
            "Mensaje de Ridvan 179",
            "Mensaje de Ridvan 180",
            "CUJ",
            "Libro 1",
            "Libro 10",
            "Libro 11",
            "Libro 12",
            "Libro 13",
            "Libro 14",
            "Cristo_y_Bahaullah",
        ];

        let mut checked = 0;
        let mut unclassified = Vec::new();
        for entry in walkdir::WalkDir::new(&samples_dir)
            .follow_links(true)
            .into_iter()
            .filter_entry(|e| {
                let name = e.file_name().to_string_lossy().to_lowercase();
                !name.starts_with('.') && name != "node_modules"
            })
        {
            let entry = entry.unwrap();
            if entry.file_type().is_file() {
                let path = entry.path();
                let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
                if ext == "pdf" || ext == "md" {
                    let meta = infer_metadata_from_path(path);
                    let filename = path.file_name().unwrap().to_string_lossy();

                    if must_classify.iter().any(|pat| filename.contains(pat)) {
                        assert!(
                            meta.religion.is_some(),
                            "Expected religion for: {}",
                            filename
                        );
                        assert_eq!(
                            meta.religion.as_deref(),
                            Some("Fe bahá'í"),
                            "Wrong religion for: {}",
                            filename
                        );
                    }

                    if meta.religion.is_none() {
                        unclassified.push(filename.to_string());
                    }
                    checked += 1;
                }
            }
        }
        assert!(checked > 0, "No sample files found to check");
        // Allow up to 3 unclassified files (ambiguous date-only names, etc.)
        assert!(
            unclassified.len() <= 3,
            "Too many unclassified files: {:?}",
            unclassified
        );
    }
}
