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

    // Try Baha'i filename patterns first
    if let Some(filename) = path.file_stem().and_then(|s| s.to_str()) {
        if let Some(baha_meta) = infer_baha_metadata(filename) {
            meta.religion = Some("Bahaismo".to_string());
            meta.book = baha_meta.0; // message type: Ridván, CUJ, etc.
            meta.chapter = baha_meta.1; // year
            meta.title = Some(filename.to_string());
            meta.language = Some("Castellano".to_string());
            return meta;
        }
    }

    // Fallback: detect religion from folder path components
    let components: Vec<&str> = path
        .components()
        .filter_map(|c| c.as_os_str().to_str())
        .collect();

    let religion_names = [
        "islam", "cristianismo", "judaismo", "hinduismo", "budismo",
        "bahaismo", "sintoismo", "zoroastrismo", "jainismo", "sijismo",
        "christianity", "judaism", "hinduism", "buddhism",
        "christian", "jewish", "muslim", "buddhist", "hindu",
    ];

    for (i, component) in components.iter().enumerate() {
        let lower = component.to_lowercase();
        for name in &religion_names {
            if lower.contains(name.trim()) {
                meta.religion = Some(component.to_string());
                if i + 1 < components.len() {
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
        assert_eq!(meta.religion.as_deref(), Some("Bahaismo"));
        assert_eq!(meta.book.as_deref(), Some("Ridván"));
        assert_eq!(meta.chapter.as_deref(), Some("2026"));
    }

    #[test]
    fn test_infer_baha_date_prefix_without_cuj() {
        let path = Path::new("samples/051227 Mensaje a conferencia consejeros CAST.pdf");
        let meta = infer_metadata_from_path(path);
        assert_eq!(meta.religion.as_deref(), Some("Bahaismo"));
        assert_eq!(meta.book.as_deref(), Some("Mensaje a conferencia consejeros"));
        assert_eq!(meta.chapter.as_deref(), Some("2005"));
    }

    #[test]
    fn test_infer_baha_cuj_keyword() {
        let path = Path::new("samples/101228 CUJ, carta a CCs delineando nuevo Plan de 5 Años CAST.pdf");
        let meta = infer_metadata_from_path(path);
        assert_eq!(meta.religion.as_deref(), Some("Bahaismo"));
        assert_eq!(meta.book.as_deref(), Some("CUJ"));
        assert_eq!(meta.chapter.as_deref(), Some("2010"));
    }

    #[test]
    fn test_infer_from_folder_path() {
        let path = Path::new("library/islam/quran/chapter1.md");
        let meta = infer_metadata_from_path(path);
        assert_eq!(meta.religion.as_deref(), Some("islam"));
        assert_eq!(meta.book.as_deref(), Some("quran"));
    }
}
