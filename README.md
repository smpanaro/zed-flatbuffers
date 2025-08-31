# zed-flatbuffers
[Zed](https://zed.dev) editor extension to add language support for [FlatBuffers](https://flatbuffers.dev)

# Features
- Syntax highlighting
- Outlines for navigating between types
- Experimental Language Server Protocol support (go to definition etc)

<img width="1028" alt="screenshot of a flatbuffer file in zed" src="https://github.com/user-attachments/assets/b250f291-9bf4-4519-b044-36ea968004fa">

# Language Server

The extension expects the `flatbuffers-language-server` binary to be available in your system PATH or configured through Zed settings.

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
