#!/usr/bin/env python3
"""Extract text from a PDF using PyMuPDF. Used as fallback when pdf-extract fails."""
import sys
import io
import fitz  # PyMuPDF

# Force UTF-8 output regardless of system locale
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8", errors="replace")
sys.stderr = io.TextIOWrapper(sys.stderr.buffer, encoding="utf-8", errors="replace")

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
