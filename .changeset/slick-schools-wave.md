---
"@biomejs/biome": patch
---

Fixed parsing of SCSS bracketed lists containing unary operands, such as `[-$gap, +$gap, not false]`, while preserving keyword case and escaped operator spellings.
