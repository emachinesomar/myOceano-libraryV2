#!/usr/bin/env python3
"""Extract text from a PDF using PyMuPDF. Used as fallback when pdf-extract fails."""
import sys
import fitz  # PyMuPDF

def extract_text(pdf_path: str) -> str:
    doc = fitz.open(pdf_path)
    parts = []
    for page in doc:
        text = page.get_text()
        if text.strip():
            parts.append(text)
    doc.close()
    return "\n".join(parts)

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: python pymupdf_extract.py <pdf_path>", file=sys.stderr)
        sys.exit(1)
    try:
        text = extract_text(sys.argv[1])
        sys.stdout.write(text)
    except Exception as e:
        print(f"ERROR: {e}", file=sys.stderr)
        sys.exit(1)
