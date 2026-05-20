#[path = "legacy_core_regression.rs"]
mod core_regression;

#[path = "legacy_history.rs"]
mod history;

#[path = "legacy_namespace_regression.rs"]
mod namespace_regression;

#[path = "legacy_procfs_regression.rs"]
mod procfs_regression;

#[path = "legacy_tree_dirfd_regression.rs"]
mod tree_dirfd_regression;

#[path = "legacy_tree_regression.rs"]
mod tree_regression;

pub fn run_active_once_bridge() {
    crate::fs::vfs::legacy_tree::active_once();
}

pub fn run_history_bus() -> Option<&'static str> {
    history::run_history_bus()
}
