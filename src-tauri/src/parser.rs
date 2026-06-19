use gray_matter::engine::YAML;
use gray_matter::Matter;
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
pub fn infer_metadata_from_path(path: &Path) -> DocumentMetadata {
    let components: Vec<&str> = path
        .components()
        .filter_map(|c| c.as_os_str().to_str())
        .collect();

    let mut meta = DocumentMetadata::default();

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

    if let Some(filename) = path.file_stem().and_then(|s| s.to_str()) {
        if meta.title.is_none() {
            meta.title = Some(filename.to_string());
        }
    }

    meta
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
}
