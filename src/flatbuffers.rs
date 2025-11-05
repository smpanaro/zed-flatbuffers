use std::fs;
use zed::settings::LspSettings;
use zed_extension_api::{
    self as zed, CodeLabel, CodeLabelSpan, LanguageServerId, Result,
    lsp::{Completion, CompletionKind, Symbol, SymbolKind},
};

macro_rules! log {
    // ($($arg:tt)*) => { println!("[flatbuffers-extension] {}", format!($($arg)*)); }; // uncomment for local debugging
    ($($arg:tt)*) => {}; // no-op
}

const GENERIC_ERROR_MESSAGE: &str = "You can download a release manually or build from source here: https://github.com/smpanaro/flatbuffers-language-server\nOr open an issue here: https://github.com/smpanaro/zed-flatbuffers/issues";

struct FlatBuffersExtension {
    cached_binary_path: Option<String>,
}

impl FlatBuffersExtension {
    fn language_server_binary_path(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<String> {
        log!("searching for binary");
        // From $PATH
        if let Some(path) = worktree.which("flatbuffers-language-server") {
            log!("found binary in PATH: {}", path);
            return Ok(path);
        }

        // From settings: {"lsp": {"flatbuffers-language-server": {"binary": {"path": "/path/to/it"}}}
        // NOTE: It seems like when this is set, Zed does not even call the extension.
        if let Some(path) = LspSettings::for_worktree("flatbuffers-language-server", worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.binary)
            .and_then(|b| b.path)
        {
            log!("found binary in settings: {}", path);
            return Ok(path);
        }

        // From previous download (in this run of the editor?)
        if let Some(path) = &self.cached_binary_path {
            if fs::metadata(path).map_or(false, |stat| stat.is_file()) {
                log!("found cached binary: {}", path);
                return Ok(path.clone());
            }
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );
        let release = zed::latest_github_release(
            "smpanaro/flatbuffers-language-server",
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;

        log!("found release: {:?}", release);

        let (platform, arch) = zed::current_platform();
        let file_kind = match platform {
            zed::Os::Windows => zed::DownloadedFileType::Zip,
            zed::Os::Mac | zed::Os::Linux => zed::DownloadedFileType::GzipTar,
        };
        let asset_name = format!(
            "flatbuffers-language-server-{version}-{arch}-{os}{ext}",
            version = release.version,
            arch = match arch {
                zed::Architecture::Aarch64 => "aarch64",
                zed::Architecture::X86 => "x86",
                zed::Architecture::X8664 => "x86_64",
            },
            os = match platform {
                zed::Os::Mac => "apple-darwin",
                zed::Os::Linux => "unknown-linux-gnu",
                zed::Os::Windows => "pc-windows-msvc",
            },
            ext = match file_kind {
                zed::DownloadedFileType::Zip => ".zip",
                zed::DownloadedFileType::GzipTar => ".tar.gz",
                zed::DownloadedFileType::Gzip => ".gz",
                zed::DownloadedFileType::Uncompressed => "",
            }
        );

        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .ok_or_else(|| {
                format!(
                    "no asset found matching {:?}\n\n{GENERIC_ERROR_MESSAGE}",
                    asset_name
                )
            })?;

        let version_dir_prefix = "flatbuffers-language-server-";
        let version_dir = format!("{version_dir_prefix}{}", release.version);
        fs::create_dir_all(&version_dir).map_err(|err| {
            format!("failed to create directory '{version_dir}': {err}\n\n{GENERIC_ERROR_MESSAGE}")
        })?;
        let binary_path = format!("{version_dir}/flatbuffers-language-server");

        if !fs::metadata(&binary_path).map_or(false, |stat| stat.is_file()) {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            zed::download_file(&asset.download_url, &version_dir, file_kind)
                .map_err(|e| format!("failed to download file: {e}\n\n{GENERIC_ERROR_MESSAGE}"))?;

            zed::make_file_executable(&binary_path)?;

            // Remove old versions
            let entries = fs::read_dir(".").map_err(|e| {
                format!("failed to list working directory {e}\n\n{GENERIC_ERROR_MESSAGE}")
            })?;
            for entry in entries {
                let entry = entry.map_err(|e| {
                    format!("failed to load directory entry {e}\n\n{GENERIC_ERROR_MESSAGE}")
                })?;
                if entry.file_name().to_str() != Some(&version_dir)
                    && entry
                        .file_name()
                        .to_string_lossy()
                        .starts_with(version_dir_prefix)
                {
                    log!(
                        "removing old version of flatbuffer-language-server: {}",
                        entry.path().display()
                    );
                    fs::remove_dir_all(entry.path()).ok();
                }
            }
        }

        self.cached_binary_path = Some(binary_path.clone());
        Ok(binary_path)
    }
}

impl zed::Extension for FlatBuffersExtension {
    fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        log!("starting flatbuffers extension");
        Ok(zed::Command {
            command: self.language_server_binary_path(language_server_id, worktree)?,
            args: vec![],
            env: Default::default(),
        })
    }

    /// Returns the label for the given completion.
    fn label_for_completion(
        &self,
        _language_server_id: &LanguageServerId,
        completion: Completion,
    ) -> Option<CodeLabel> {
        let name = completion.label;
        let kind = completion.kind?;

        let (code, filter_range, display_range) = match kind {
            CompletionKind::Class => {
                let code = format!("table {name} {{}}");
                let filter_range = 6..6 + name.len();
                let display_range = 0..filter_range.end;
                (code, filter_range, display_range)
            }
            CompletionKind::Struct => {
                let code = format!("struct {name} {{}}");
                let filter_range = 7..7 + name.len();
                let display_range = 0..filter_range.end;
                (code, filter_range, display_range)
            }
            CompletionKind::Enum => {
                // Note: v0.0.1 of the language serve incorrectly reports unions as enums.
                let code = format!("enum {name} : byte {{}}");
                let filter_range = 5..5 + name.len();
                let display_range = 0..filter_range.end;
                (code, filter_range, display_range)
            }
            CompletionKind::Interface => {
                let code = format!("union {name} {{}}");
                let filter_range = 6..6 + name.len();
                let display_range = 0..filter_range.end;
                (code, filter_range, display_range)
            }
            CompletionKind::Keyword => {
                let code = format!("table F {{ f: {name}; }}");
                let filter_range = 13..13 + name.len();
                let display_range = filter_range.clone();
                (code, filter_range, display_range)
            }
            CompletionKind::Module => {
                let code = format!("namespace {name};");
                let filter_range = 10..10 + name.len();
                let display_range = 0..filter_range.end;
                (code, filter_range, display_range)
            }
            CompletionKind::Property => {
                return Some(CodeLabel {
                    code: name.clone(),
                    spans: vec![CodeLabelSpan::literal(
                        name.clone(),
                        Some("attribute".to_string()),
                    )],
                    filter_range: (0..name.len()).into(),
                });
            }
            _ => return None,
        };

        let label_desc = completion
            .label_details
            .and_then(|ld| ld.description)
            .map(|desc| format!("({desc})"))
            .unwrap_or_default();

        Some(CodeLabel {
            code,
            spans: vec![
                CodeLabelSpan::code_range(display_range),
                CodeLabelSpan::literal(" ", None),
                CodeLabelSpan::literal(label_desc, Some("comment".to_string())), // Second param here comes from highlights.scm
            ],
            filter_range: filter_range.into(),
        })
    }

    /// Returns the label for the given symbol.
    fn label_for_symbol(
        &self,
        _language_server_id: &LanguageServerId,
        symbol: Symbol,
    ) -> Option<CodeLabel> {
        let name = &symbol.name;

        let (code, filter_range, display_range) = match symbol.kind {
            SymbolKind::Class => {
                let code = format!("table {name} {{}}");
                let filter_range = 6..6 + name.len();
                let display_range = 0..filter_range.end;
                (code, filter_range, display_range)
            }
            SymbolKind::Struct => {
                let code = format!("struct {name} {{}}");
                let filter_range = 7..7 + name.len();
                let display_range = 0..filter_range.end;
                (code, filter_range, display_range)
            }
            SymbolKind::Enum => {
                let code = format!("enum {name} : byte {{}}");
                let filter_range = 5..5 + name.len();
                let display_range = 0..filter_range.end;
                (code, filter_range, display_range)
            }
            SymbolKind::Interface => {
                let code = format!("union {name} {{}}");
                let filter_range = 6..6 + name.len();
                let display_range = 0..filter_range.end;
                (code, filter_range, display_range)
            }
            SymbolKind::Object => {
                let code = format!("rpc_service {name} {{ Read(Req): Resp; }}");
                let filter_range = 12..12 + name.len();
                let display_range = 0..filter_range.end;
                (code, filter_range, display_range)
            }
            _ => return None,
        };

        Some(CodeLabel {
            code,
            spans: vec![CodeLabelSpan::code_range(display_range)],
            filter_range: filter_range.into(),
        })
    }
}

zed::register_extension!(FlatBuffersExtension);
