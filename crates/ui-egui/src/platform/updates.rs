//! Application updates are managed centrally by the owner.
//!
//! Keep the host entry points as no-ops for compatibility, but never construct an
//! updater, contact a release service, display an update dialog, or self-replace
//! the executable. This preserves reviewed local customizations across updates.

use crate::PdfCraftApp;

/// Compatibility hook for embedders and offline regression tests. Never called.
#[cfg(not(target_arch = "wasm32"))]
pub type UpdateSource = craft_update::updater::CheckSource;

impl PdfCraftApp {
    /// Compatibility entry point; the central manager owns update checks.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn set_startup_update_check(&mut self, _on: bool) {}

    #[cfg(not(target_arch = "wasm32"))]
    pub fn set_update_source(&mut self, _source: UpdateSource) {}

    /// Compatibility entry point; no network, update prompt or self-update.
    pub fn check_for_updates(&mut self) {}
}
