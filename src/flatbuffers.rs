use std::fs;
use zed::settings::LspSettings;
use zed_extension_api::{self as zed, LanguageServerId, Result};

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
}

zed::register_extension!(FlatBuffersExtension);
