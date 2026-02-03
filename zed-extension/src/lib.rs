use zed_extension_api as zed;

struct DocgraphExtension {
    cached_binary_path: Option<String>,
}

impl zed::Extension for DocgraphExtension {
    fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        let binary_name = "docgraph";

        // Try to find the binary path
        let binary_path = if let Some(path) = &self.cached_binary_path {
            path.clone()
        } else {
            // "docgraph" command should correspond to the binary in the path.
            // On mac/linux, we can try to use `which` or simply assume it's in PATH.
            // Zed's `which` allows us to search for the binary in the environment.
            if let Some(path) = worktree.which(binary_name) {
                self.cached_binary_path = Some(path.clone());
                path
            } else {
                return Err(format!("{} not found in PATH", binary_name).into());
            }
        };

        Ok(zed::Command {
            command: binary_path,
            args: vec!["lsp".to_string()],
            env: Default::default(),
        })
    }
}

zed::register_extension!(DocgraphExtension);
