use std::fs;
use zed_extension_api::{
    self as zed, set_language_server_installation_status as set_install_status,
    settings::LspSettings, LanguageServerId, LanguageServerInstallationStatus as Status, Result,
};

struct TyBinary {
    path: String,
    args: Option<Vec<String>>,
}

struct TyExtension {
    cached_binary_path: Option<String>,
}

impl TyExtension {
    fn language_server_binary(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<TyBinary> {
        let settings = LspSettings::for_worktree(language_server_id.as_ref(), worktree);
        let binary = settings.ok().and_then(|settings| settings.binary);
        let args = binary.as_ref().and_then(|binary| binary.arguments.clone());
        let path = binary
            .and_then(|binary| binary.path)
            .or_else(|| worktree.which("ty"))
            .unwrap_or(self.zed_managed_binary_path(language_server_id)?);
        Ok(TyBinary { path, args })
    }

    fn zed_managed_binary_path(&mut self, language_server_id: &LanguageServerId) -> Result<String> {
        if let Some(path) = &self.cached_binary_path {
            if fs::metadata(path).is_ok_and(|stat| stat.is_file()) {
                return Ok(path.clone());
            }
        }

        set_install_status(language_server_id, &Status::CheckingForUpdate);
        let release = zed::latest_github_release(
            "astral-sh/ty",
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: true,
            },
        )?;

        let (platform, architecture) = zed::current_platform();
        let arch = match architecture {
            zed::Architecture::Aarch64 => "aarch64",
            zed::Architecture::X86 => "i686",
            zed::Architecture::X8664 => "x86_64",
        };
        let os = match platform {
            zed::Os::Mac => "apple-darwin",
            zed::Os::Linux => "unknown-linux-gnu",
            zed::Os::Windows => "pc-windows-msvc",
        };
        let suffix = match platform {
            zed::Os::Windows => "zip",
            _ => "tar.gz",
        };
        let asset_stem = format!("ty-{arch}-{os}");
        let asset_name = format!("ty-{arch}-{os}.{suffix}");

        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .ok_or_else(|| format!("no asset found matching {asset_name:?}"))?;

        let version_dir = format!("ty-{}", release.version);
        let binary_path = match platform {
            zed::Os::Windows => format!("{version_dir}/ty.exe"),
            _ => format!("{version_dir}/{asset_stem}/ty"),
        };

        if !fs::metadata(&binary_path).is_ok_and(|stat| stat.is_file()) {
            set_install_status(language_server_id, &Status::Downloading);
            let file_kind = match platform {
                zed::Os::Windows => zed::DownloadedFileType::Zip,
                _ => zed::DownloadedFileType::GzipTar,
            };
            zed::download_file(&asset.download_url, &version_dir, file_kind)
                .map_err(|e| format!("failed to download file: {e}"))?;

            let entries =
                fs::read_dir(".").map_err(|e| format!("failed to list working directory {e}"))?;
            for entry in entries {
                let entry = entry.map_err(|e| format!("failed to load directory entry {e}"))?;
                if entry.file_name().to_str() != Some(&version_dir) {
                    fs::remove_dir_all(entry.path()).ok();
                }
            }
        }

        self.cached_binary_path = Some(binary_path.clone());
        Ok(binary_path)
    }
}

impl zed::Extension for TyExtension {
    fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed_extension_api::LanguageServerId,
        worktree: &zed_extension_api::Worktree,
    ) -> zed_extension_api::Result<zed_extension_api::Command> {
        let ty_binary = self.language_server_binary(language_server_id, worktree)?;
        Ok(zed::Command {
            command: ty_binary.path,
            args: ty_binary.args.unwrap_or_else(|| vec!["server".into()]),
            env: vec![],
        })
    }

    fn language_server_initialization_options(
        &mut self,
        server_id: &zed_extension_api::LanguageServerId,
        worktree: &zed_extension_api::Worktree,
    ) -> zed_extension_api::Result<Option<zed_extension_api::serde_json::Value>> {
        let settings = LspSettings::for_worktree(server_id.as_ref(), worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.initialization_options.clone())
            .unwrap_or_default();
        Ok(Some(settings))
    }

    fn language_server_workspace_configuration(
        &mut self,
        server_id: &zed_extension_api::LanguageServerId,
        worktree: &zed_extension_api::Worktree,
    ) -> zed_extension_api::Result<Option<zed_extension_api::serde_json::Value>> {
        let settings = LspSettings::for_worktree(server_id.as_ref(), worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.settings.clone())
            .unwrap_or_default();
        Ok(Some(settings))
    }
}

zed::register_extension!(TyExtension);
