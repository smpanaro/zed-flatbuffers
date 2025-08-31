use zed_extension_api as zed;

struct FlatBuffersExtension;

impl zed::Extension for FlatBuffersExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        // TODO: Download pre-compiled binaries.
        Ok(zed::Command {
            command: "flatbuffers-language-server".to_string(),
            args: vec![],
            env: Default::default(),
        })
    }
}

zed::register_extension!(FlatBuffersExtension);
