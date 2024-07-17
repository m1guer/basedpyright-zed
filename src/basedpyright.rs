use zed_extension_api as zed;

struct BasedPyright;

impl zed::Extension for BasedPyright {
    fn new() -> Self {
        Self {}
    }
    fn language_server_command(
        &mut self,
        _language_server_id: &zed_extension_api::LanguageServerId,
        worktree: &zed_extension_api::Worktree,
    ) -> zed_extension_api::Result<zed_extension_api::Command> {
        let path = worktree
            .which("basedpyright-langserver")
            .ok_or_else(|| "basedpyright must be installed and avaible in $PATH.".to_string())?;
        Ok(zed::Command {
            command: path,
            args: vec!["--stdio".to_string(), Default::default()],
            env: Default::default(),
        })
    }
}

zed::register_extension!(BasedPyright);
