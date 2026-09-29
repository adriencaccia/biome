---
"@biomejs/biome": patch
---

Improved recovery from malformed SCSS expressions such as `$x: [1 + $];` so surrounding declarations keep their structure.
