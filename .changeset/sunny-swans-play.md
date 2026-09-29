---
"@biomejs/biome": patch
---

Fixed formatting of one-element SCSS comma lists so they retain their list identity.

```diff
- $items:( item , );
+ $items: (item,);
```
