---
"@biomejs/biome": patch
---

Fixed formatting of SCSS bracketed lists so trailing line comments stay before the closing bracket.

```diff
- $items:[ -$gap // item
- ];
+ $items: [-$gap // item
+ ];
```
