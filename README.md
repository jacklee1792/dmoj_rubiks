# dmoj_rubiks

2-phase Rubik's cube solver. 3rd AC on the site!

Some notable things:
- No `unsafe` usage
- No SIMD intrinsics
- No cube transformations based on lookup tables

**Coordinate description:**
- CO: corner orientation of 7 corners (phase 1)
  - 2187 values, 291 conjugacy classes (DRUD-preserving + non-mirroring symmetries)
- EO: edge orientation (FB) of 11 edges (phase 1)
  - 2048 values, 336 conjugacy classes (DRUD-preserving + double-rotation symmetries)
- ESlice: location of E-slice edges (phase 1)
  - 495 values
- CP: corner permutation (phase 2)
  - 40320 values, 2768 conjugacy classes (DRUD-preserving symmetries)
- EP: edge permutation of non-E-slice edges (phase 2)
  - 40320 values, 2768 conjugacy classes (DRUD-preserving symmetries)
- ESliceEP: permutation of E-slice edges (phase 2)
  - 24 values

**Pruning:**
- Phase 1: max(CO+Eslice, EO+ESlice)
- Phase 2: max(CP+EsliceEP, EP+EsliceEP)
