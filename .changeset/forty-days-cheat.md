---
"@biomejs/biome": patch
---

Fixed formatting of SCSS bracketed lists so trailing line comments stay before the closing bracket.

For a comment written inside the list, the formatted output changed as follows:

```diff
- $single: [-$gap]; // single item
+ $single: [-$gap // single item
+ ];
```
