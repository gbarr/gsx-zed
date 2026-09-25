use std::fs;
use zed::settings::LspSettings;
use zed_extension_api::{self as zed, LanguageServerId, Result};

struct GSXExtension;

impl zed::Extension for GSXExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let path = worktree
            .which("gsx")
            .ok_or_else(|| "Cannot find command `gsx` in the PATH.".to_string())?;

        Ok(zed::Command {
            command: path,
            args: vec!["lsp".to_string()],
            env: Default::default(),
        })
    }
}

zed::register_extension!(GSXExtension);
