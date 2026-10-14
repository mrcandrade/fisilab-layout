//! Help › About FisiLayout: tabs About · Contributors · Models. About shows the app icon, version,
//! the credit to the original project (DesignCraft) and the Suíte FisiLab links; Contributors and
//! Models are the compiled-in credits of the upstream project (`crate::credits`).
//!
//! Suíte FisiLab: the upstream ArtCraft mark, Discord buttons and sibling-app list were removed
//! (the ArtCraft name and logos are trademarks, not covered by the open-source licence). The
//! suite's own About window is `fisilab_brand::about` (Help › Sobre a Suíte FisiLab…).

use designcraft_engine::links;
use egui::{Color32, Rect, Sense, pos2, vec2};
use serde_json::json;

use crate::DesignApp;
use crate::theme::{Tokens, semibold};

/// The About window's tabs, in `UiState::about_tab` order (`help.about {tab}` names them in lowercase).
pub const ABOUT_TABS: [&str; 3] = ["About", "Contributors", "Models"];

/// The FisiLayout app icon (Suíte FisiLab), decoded once and cached.
const APP_ICON_PNG: &[u8] = include_bytes!("../../../assets/fisilab/fisilayout-128.png");

/// Paint the FisiLayout icon into `rect` (square). Nothing is painted if it can't be decoded.
pub fn paint_app_icon(ui: &egui::Ui, rect: Rect) {
    let ctx = ui.ctx();
    let key = egui::Id::new("fisilayout_app_icon");
    let cached: Option<Option<egui::TextureHandle>> = ctx.data(|d| d.get_temp(key));
    let tex = cached.unwrap_or_else(|| {
        let t = designcraft_images::decode_rgba(APP_ICON_PNG).map(|img| {
            let size = [img.width() as usize, img.height() as usize];
            let ci = egui::ColorImage::from_rgba_unmultiplied(size, img.as_raw());
            ctx.load_texture("fisilayout_app_icon", ci, egui::TextureOptions::LINEAR)
        });
        ctx.data_mut(|d| d.insert_temp(key, t.clone()));
        t
    });
    let Some(tex) = tex else { return };
    let side = rect.width().min(rect.height());
    let r = Rect::from_center_size(rect.center(), vec2(side, side));
    ui.painter().image(tex.id(), r, Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)), Color32::WHITE);
}

/// A text link that opens `url`. Returns true when clicked.
fn link(ui: &mut egui::Ui, text: &str, url: &str, color: Color32) -> bool {
    let resp = ui.add(egui::Label::new(egui::RichText::new(text).size(13.0).color(color).underline()).sense(Sense::click()));
    let resp = resp.on_hover_text(url).on_hover_cursor(egui::CursorIcon::PointingHand);
    resp.clicked()
}

/// The Suíte FisiLab links (label, `links` key), shown in the About window and on the start screen.
pub const SUITE_LINKS: [(&str, &str); 4] = [
    ("Site da Suíte FisiLab", "suite"),
    ("Manifesto", "manifesto"),
    ("Projeto original (DesignCraft)", "upstream"),
    ("Reportar um problema", "issues"),
];

pub fn show(app: &mut DesignApp, ctx: &egui::Context) {
    if !app.ui.about {
        return;
    }
    let t = Tokens::get(ctx);
    let mut open: Option<&'static str> = None;
    let resp = egui::Modal::new(egui::Id::new("about")).show(ctx, |ui| {
        ui.set_width(if app.ui.about_tab == 0 { 460.0 } else { 640.0 });
        ui.horizontal(|ui| {
            for (i, l) in ABOUT_TABS.iter().enumerate() {
                let i = i as u8;
                if ui.selectable_label(app.ui.about_tab == i, crate::rtl::widget(ui, crate::i18n::tr(&app.ui.language, l))).clicked() {
                    app.ui.about_tab = i;
                }
            }
        });
        ui.separator();
        if app.ui.about_tab != 0 {
            let h = 400.0_f32.min(ctx.content_rect().height() * 0.7).max(160.0);
            ui.allocate_ui_with_layout(vec2(ui.available_width(), h), egui::Layout::top_down(egui::Align::Min), |ui| {
                ui.set_min_height(h);
                if app.ui.about_tab == 1 {
                    crate::credits::contributors_ui(ui);
                } else {
                    crate::credits::models_ui(ui);
                }
            });
            ui.separator();
            ui.vertical_centered(|ui| {
                if ui.button(crate::rtl::widget(ui, crate::i18n::tr(&app.ui.language, "  Close  "))).clicked() {
                    app.ui.about = false;
                }
            });
            return;
        }
        ui.vertical_centered(|ui| {
            ui.add_space(8.0);
            let (r, _) = ui.allocate_exact_size(vec2(72.0, 72.0), Sense::hover());
            paint_app_icon(ui, r);
            ui.add_space(6.0);
            ui.label(crate::rtl::widget(
                ui,
                egui::RichText::new(crate::i18n::tr(&app.ui.language, "FisiLayout")).font(semibold(26.0)).color(t.text_strong),
            ));
            crate::rtl::label(
                ui,
                egui::RichText::new(format!("{} {}", crate::i18n::tr(&app.ui.language, "Version"), env!("CARGO_PKG_VERSION")))
                    .size(12.0)
                    .color(t.text_dim),
            );
            ui.add_space(4.0);
            ui.label(crate::rtl::widget(
                ui,
                egui::RichText::new(crate::i18n::tr(&app.ui.language, "Page layout for print and screen — fast, open and scriptable."))
                    .size(13.0)
                    .color(t.text),
            ));
            ui.label(crate::rtl::widget(
                ui,
                egui::RichText::new(format!("{} · {}", fisilab_brand::SUITE, fisilab_brand::LAB)).size(12.0).color(t.text_dim),
            ));
            ui.label(crate::rtl::widget(
                ui,
                egui::RichText::new(format!("Em parceria com o {}. {}", fisilab_brand::PARTNER, fisilab_brand::AUDIENCE))
                    .size(11.0)
                    .color(t.text_dim),
            ));
            ui.add_space(8.0);
            ui.label(crate::rtl::widget(
                ui,
                egui::RichText::new(
                    "Baseado no DesignCraft, da ArtCraft Team e contribuidores (MIT OR Apache-2.0). Os nomes e marcas do projeto \
                     original pertencem aos seus titulares; esta versão não é afiliada nem endossada por eles.",
                )
                .size(11.0)
                .color(t.text_dim),
            ));
            ui.add_space(14.0);
            let label = egui::RichText::new(fisilab_brand::ABOUT_MENU_LABEL.trim_end_matches('…')).size(14.0).color(Color32::WHITE);
            if ui.add(egui::Button::new(label).fill(t.accent_strong).corner_radius(18.0).min_size(vec2(260.0, 36.0))).clicked() {
                open = Some("fisilab");
            }
            ui.add_space(12.0);
            ui.horizontal_wrapped(|ui| {
                for (text, key) in SUITE_LINKS {
                    if link(ui, text, links::get(key).unwrap_or_default(), t.focus) {
                        open = Some(key);
                    }
                    ui.add_space(6.0);
                }
            });
            ui.add_space(12.0);
            ui.label(crate::rtl::widget(
                ui,
                egui::RichText::new(crate::i18n::tr(&app.ui.language, "Open source. No telemetry.")).size(11.0).color(t.text_dim),
            ));
            ui.add_space(8.0);
            if ui.button(crate::rtl::widget(ui, crate::i18n::tr(&app.ui.language, "  Close  "))).clicked() {
                app.ui.about = false;
            }
        });
    });
    if resp.should_close() {
        app.ui.about = false;
    }
    if let Some(k) = open {
        if k == "fisilab" {
            // The modal would cover the suite's window: close it first.
            app.ui.about = false;
        }
        let _ = app.run(&format!("help.{k}"), json!({}));
    }
}
