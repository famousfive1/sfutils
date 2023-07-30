# SFILS
---
### Simple File Utils
Implementation of some very simple file related utilities.

Currently supports the following operations:
- Diff
- Fuzzy find

----
### Command usage:

`sfils <FILE 1> <OP> <ARG> [options]`

- `FILE 1` is the name of the base file to operate upon
- `OP` is the operation to perform `[d]iff` or `[f]uzzy`
- `ARG` is the argument to the operation, either another file name or a string
- `[options]` are set of options to modify the behaviour of the operation being performed
  - Pass `f` for full file display in diff mode
  - Pass `i` for case-insensitive search in fuzzy mode

---
Made by
\
Tejaswi Hegde
