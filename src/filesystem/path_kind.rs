#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PathKind {
    File,
    Directory,
    Symlink,
    Other,
}
