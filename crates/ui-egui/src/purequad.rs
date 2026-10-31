//! PureQuad build (fork patch): the product name and the links that replace the upstream
//! community links. Kept in one file so rebases on upstream GridCraft stay small.

/// The name shown in the window title, the About box and the installers.
pub const APP_NAME: &str = "PureQuad Sheets";
/// PureQuad Cloud, opened by the title-bar button, Help › PureQuad Cloud and the Backstage.
pub const CLOUD_URL: &str = "https://cloud.purequad.com/fisiere";
/// Where employees find help and the other PureQuad apps.
pub const HELP_URL: &str = "https://cloud.purequad.com/aplicatii";
/// Source of the fork.
pub const SOURCE_URL: &str = "https://github.com/harababura/gridcraft-cloud";

/// Show PureQuad Cloud news (a file opened, saved or not saved) in the toast.
pub fn cloud_status(app: &mut crate::SheetApp, message: String) {
    app.toast = Some((message, crate::clock::now_ms()));
}

/// Show the PureQuad Cloud window (drawn by the app, `apps/gridcraft/src/cloud_links.rs`).
pub fn browse(ctx: &egui::Context) {
    purequad_cloud::requests::request(purequad_cloud::requests::Request::Browse);
    ctx.request_repaint();
}

/// Save the active workbook (when it has a file) and offer to put a copy in PureQuad Cloud;
/// from then on every save of that copy goes to the cloud.
pub fn save_to_cloud(app: &mut crate::SheetApp, ctx: &egui::Context) {
    if !app.commit_edit(0, 0, false, false) {
        return;
    }
    let path = app.session.active().and_then(|d| d.path.clone());
    if path.is_some() && app.session.active().is_some_and(|d| d.is_dirty()) {
        app.run_or_alert("file.save", serde_json::json!({}));
    }
    purequad_cloud::requests::save_copy(path.map(std::path::PathBuf::from));
    ctx.request_repaint();
}
