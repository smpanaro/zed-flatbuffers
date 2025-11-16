## Unreleased

### Language Server
- Add syntax highlighting for Project Symbols. Try with: `project symbols: toggle` or ⌘T.
- Add syntax highlighting for completions.

### Grammar
- Update to new grammar version. Enabling:
  - Fuller Outline Panel with types, enum values, and file-level declarations.
  - Proper syntax highlighting of namespaces.
  - Highlighting builtin types (int, string, etc.) distinct from user-defined types, if the theme supports it.
  - Support vim % for quotes.
- Automatically close brackets, quotes, and comments.
- Support text object vim motions. e.g. `]]` to move between types; `ac` in visual mode to select the surrounding declaration
- TODO/FIXME/etc highlighting in comments when the [Comment extension](https://github.com/thedadams/zed-comment) is installed.

## 0.0.2 - October 19, 2025

Add Language Server support via [flatbuffers-language-server](https://github.com/smpanaro/flatbuffers-language-server).

## 0.0.1 -  September 24, 2024

Initial release. Adds syntax highlighting for FlatBuffers schema files.
