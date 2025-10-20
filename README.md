# zed-flatbuffers
[Zed](https://zed.dev) editor extension to add language support for [FlatBuffers](https://flatbuffers.dev)

# Features
- Syntax highlighting
- Language Server support (go to definition, hovers, diagnostics etc)
- Outlines for navigating between types

<img width="1028" alt="screenshot of a flatbuffer file in zed" src="https://github.com/user-attachments/assets/4fa8f97c-f72d-4007-a6ae-ff1f1ff26f1c" />

# Language Server

The extension will use the first `flatbuffers-language-server` binary it finds in:

1. Zed settings.json (see below)
1. Your system PATH
1. The latest GitHub [release](https://github.com/smpanaro/flatbuffers-language-server/releases)

Example `settings.json`:

```json
{
  "lsp": {
    "flatbuffers-language-server": {
      "binary": {
        "path": "/path/to/flatbuffers-language-server/target/debug/flatbuffers-language-server",
        "arguments": []
      }
    }
  }
}
```

# Syntax Highlighting
Tree-Sitter Grammar: [smpanaro/tree-sitter-flatbuffers](https://github.com/smpanaro/tree-sitter-flatbuffers)
