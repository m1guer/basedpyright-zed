use zed_extension_api as zed;

struct BasedPyrightS;

impl zed::Extension for BasedPyrightS {
    fn new() -> Self {
        Self {}
    }
    fn language_server_command(
        &mut self,
        _language_server_id: &zed_extension_api::LanguageServerId,
        worktree: &zed_extension_api::Worktree,
    ) -> zed_extension_api::Result<zed_extension_api::Command> {
        let path = worktree
            .which("basedpyright")
            .ok_or_else(|| "basedpyright must be installed and avaible in $PATH.".to_string())?;
        Ok(zed::Command {
            command: path,
            args: vec![],
            env: Default::default(),
        })
    }
}

zed::register_extension!(BasedPyrightS);
