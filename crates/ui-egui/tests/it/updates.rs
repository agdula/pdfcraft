//! Central management suppresses both startup and explicit in-app update checks.
use egui_kittest::{Harness, kittest::Queryable};
use pdfcraft_ui_egui::PdfCraftApp;
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

#[test]
fn centrally_managed_updates_never_contact_a_source_or_show_a_popup() {
    for answer in [Ok(Some("v99.0.0")), Ok(None), Err("offline")] {
        let calls = Arc::new(AtomicUsize::new(0));
        let counted = calls.clone();
        let mut h = Harness::builder().with_size(egui::vec2(1200.0, 800.0)).build_eframe(move |_cc| {
            let mut app = PdfCraftApp::new();
            app.set_option("language", "en").unwrap();
            app.restore(r#"{"check_updates_at_start":true}"#);
            app.set_update_source(Arc::new(move || {
                counted.fetch_add(1, Ordering::SeqCst);
                answer
                    .map(|tag| {
                        tag.map(|tag| craft_update::Release { tag: tag.into(), version: craft_update::parse_version(tag), ..Default::default() })
                    })
                    .map_err(str::to_string)
            }));
            app.set_startup_update_check(true);
            app.check_for_updates();
            app
        });
        h.run_steps(8);
        std::thread::sleep(std::time::Duration::from_millis(50));
        h.run_steps(4);
        assert_eq!(calls.load(Ordering::SeqCst), 0);
        for label in ["is available", "Checking for updates", "Could not check for updates", "Check for updates"] {
            assert!(h.query_by_label_contains(label).is_none(), "unexpected update UI: {label}");
        }
    }
}
