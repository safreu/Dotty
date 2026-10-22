use crate::cli::{InitArgs, Result};
/// Init will not overwrite an existing directory
/// Must be relative to home
/// NOT ALLOWED:
/// - existing Directories
/// - symlinks
pub fn execute(args: InitArgs) -> Result<()> {
    let path = args.path;

    path.is_symlink();
    /// Init:
    /// 1. Create config.toml in  -> Will contain .config [.config/dotty/{config.toml}]
    /// 2. Create backups dir in .local/share -> Will contain backup_dir [.local/share/{backup.toml, backups/}]
    /// 3. Create dotfiles dir -> Will contain manifest.toml
    todo!()
}
