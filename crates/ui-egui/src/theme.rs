//! Design tokens (InDesign's four interface brightness levels), fonts and egui style.

use std::sync::Arc;

use egui::{Color32, CornerRadius, FontData, FontDefinitions, FontFamily, FontId, Stroke, TextStyle, Visuals};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Brightness {
    /// Suíte FisiLab: the dark UFRGS theme (`fisilab_brand::palette::NOITE`), the default.
    #[default]
    UfrgsNoite,
    /// Suíte FisiLab: the light UFRGS theme (`fisilab_brand::palette::DIA`).
    UfrgsDia,
    Dark,
    MediumDark,
    MediumLight,
    Light,
    /// Accessibility: black chrome, white text, yellow selection and focus.
    HighContrast,
}

impl Brightness {
    pub const ALL: [Brightness; 7] = [
        Brightness::UfrgsNoite,
        Brightness::UfrgsDia,
        Brightness::Dark,
        Brightness::MediumDark,
        Brightness::MediumLight,
        Brightness::Light,
        Brightness::HighContrast,
    ];
    pub fn label(self) -> &'static str {
        match self {
            Brightness::UfrgsNoite => "UFRGS Noite",
            Brightness::UfrgsDia => "UFRGS Dia",
            Brightness::Dark => "Dark",
            Brightness::MediumDark => "Medium Dark",
            Brightness::MediumLight => "Medium Light",
            Brightness::Light => "Light",
            Brightness::HighContrast => "High Contrast",
        }
    }
    pub fn id(self) -> &'static str {
        match self {
            Brightness::UfrgsNoite => "ufrgsNoite",
            Brightness::UfrgsDia => "ufrgsDia",
            Brightness::Dark => "dark",
            Brightness::MediumDark => "mediumDark",
            Brightness::MediumLight => "mediumLight",
            Brightness::Light => "light",
            Brightness::HighContrast => "highContrast",
        }
    }
    pub fn parse(s: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|b| b.id().eq_ignore_ascii_case(s) || b.label().eq_ignore_ascii_case(s))
    }
}

/// Every colour the UI uses. Widgets never hard-code colours.
#[derive(Clone, Copy, Debug)]
pub struct Tokens {
    pub dark: bool,
    pub app_bar: Color32,
    pub panel: Color32,
    pub panel_darker: Color32,
    pub input: Color32,
    pub input_border: Color32,
    pub divider: Color32,
    pub text: Color32,
    pub text_strong: Color32,
    pub text_dim: Color32,
    pub text_disabled: Color32,
    pub icon: Color32,
    pub hover: Color32,
    pub tool_active: Color32,
    pub accent: Color32,
    pub accent_strong: Color32,
    pub row_selected: Color32,
    pub pasteboard: Color32,
    pub ruler: Color32,
    pub ruler_tick: Color32,
    pub tab_inactive: Color32,
    pub button: Color32,
    pub radius: u8,
    /// 1 pt chrome edges.
    pub border: Color32,
    /// Outline of fields and buttons.
    pub field_border: Color32,
    /// Active tool / toggle well.
    pub well: Color32,
    pub well_rim: Color32,
    pub ruler_text: Color32,
    pub section_divider: Color32,
    /// Panel tab strips, doc-tab bar, dock headers.
    pub tab_strip: Color32,
    /// Smart guides and spacing marks drawn over the page.
    pub smart_guide: Color32,
    /// Measurement labels on the canvas (smart dimensions, gaps).
    pub measure_bg: Color32,
    pub measure_text: Color32,
    /// Links and keyboard focus.
    pub focus: Color32,
    /// egui's selection fill (selected labels, selected text).
    pub selection: Color32,
    /// Popup and window shadow.
    pub shadow: Color32,
}

fn hex(s: u32) -> Color32 {
    Color32::from_rgb((s >> 16) as u8, (s >> 8) as u8, s as u8)
}

/// `a` → `b` by `k` (0..1), opaque.
fn mix(a: Color32, b: Color32, k: f32) -> Color32 {
    let l = |x: u8, y: u8| (f32::from(x) + (f32::from(y) - f32::from(x)) * k).round().clamp(0.0, 255.0) as u8;
    Color32::from_rgb(l(a.r(), b.r()), l(a.g(), b.g()), l(a.b(), b.b()))
}

/// A premultiplied translucent `top` composited over the opaque `bottom`.
fn over(top: Color32, bottom: Color32) -> Color32 {
    let k = 1.0 - f32::from(top.a()) / 255.0;
    let l = |t: u8, b: u8| (f32::from(t) + f32::from(b) * k).round().clamp(0.0, 255.0) as u8;
    Color32::from_rgb(l(top.r(), bottom.r()), l(top.g(), bottom.g()), l(top.b(), bottom.b()))
}

/// The Suíte FisiLab themes ("UFRGS Noite" / "UFRGS Dia"): every colour from the brand tones;
/// layout values (radius) and document overlays (smart guides) as in `base`.
fn ufrgs(p: &fisilab_brand::palette::Tons, base: Tokens) -> Tokens {
    let night = fisilab_brand::palette::NOITE.fundo;
    let strong = if p.escuro { p.texto_destaque } else { night };
    let field = mix(p.borda, p.texto_fraco, 0.35);
    Tokens {
        dark: p.escuro,
        app_bar: p.painel,
        panel: p.painel,
        panel_darker: mix(p.fundo, p.painel, 0.5),
        input: p.elevado,
        input_border: field,
        divider: p.borda,
        text: p.texto,
        text_strong: strong,
        text_dim: p.texto_fraco,
        text_disabled: mix(p.texto_fraco, p.painel, 0.4),
        icon: mix(p.texto, p.texto_fraco, 0.25),
        hover: p.hover,
        tool_active: p.ativo,
        accent: p.destaque,
        accent_strong: p.destaque,
        row_selected: over(p.selecao, p.painel),
        pasteboard: p.fundo,
        ruler: if p.escuro { p.fundo } else { p.elevado },
        ruler_tick: p.texto_fraco,
        tab_inactive: mix(p.fundo, p.painel, 0.5),
        button: p.painel,
        border: p.borda,
        field_border: field,
        well: if p.escuro { p.fundo } else { p.ativo },
        well_rim: if p.escuro { p.ativo } else { p.borda },
        ruler_text: p.texto,
        section_divider: p.borda,
        tab_strip: mix(p.fundo, p.painel, 0.5),
        measure_bg: Color32::from_rgba_unmultiplied(p.elevado.r(), p.elevado.g(), p.elevado.b(), 230),
        measure_text: p.texto,
        focus: p.foco,
        selection: over(p.selecao, p.painel),
        shadow: Color32::from_rgba_unmultiplied(night.r(), night.g(), night.b(), if p.escuro { 120 } else { 70 }),
        ..base
    }
}

impl Tokens {
    pub fn for_brightness(b: Brightness) -> Self {
        let dark = Tokens {
            dark: true,
            app_bar: hex(0x262626),
            panel: hex(0x323232),
            panel_darker: hex(0x282828),
            input: hex(0x1f1f1f),
            input_border: hex(0x4a4a4a),
            divider: hex(0x1e1e1e),
            text: hex(0xd8d8d8),
            text_strong: hex(0xf2f2f2),
            text_dim: hex(0xa3a3a3),
            text_disabled: hex(0x6a6a6a),
            icon: hex(0xc8c8c8),
            hover: hex(0x424242),
            tool_active: hex(0x1b1b1b),
            accent: hex(0x378ef0),
            accent_strong: hex(0x1473e6),
            row_selected: hex(0x2d4f7c),
            pasteboard: hex(0x232323),
            ruler: hex(0x2e2e2e),
            ruler_tick: hex(0x9a9a9a),
            tab_inactive: hex(0x262626),
            button: hex(0x404040),
            radius: 1,
            border: hex(0x1e1e1e),
            field_border: hex(0x5a5a5a),
            well: hex(0x1b1b1b),
            well_rim: hex(0x3a3a3a),
            ruler_text: hex(0xd8d8d8),
            section_divider: hex(0x282828),
            tab_strip: hex(0x282828),
            smart_guide: hex(0x00c853),
            measure_bg: Color32::from_rgba_unmultiplied(70, 70, 70, 230),
            measure_text: Color32::WHITE,
            focus: hex(0x378ef0),
            selection: hex(0x1473e6),
            shadow: Color32::from_black_alpha(90),
        };
        match b {
            Brightness::UfrgsNoite => ufrgs(&fisilab_brand::palette::NOITE, dark),
            Brightness::UfrgsDia => ufrgs(&fisilab_brand::palette::DIA, dark),
            Brightness::Dark => dark,
            // Measured from InDesign 2026 (plan/indesign/11-observed-ui.md §1).
            Brightness::MediumDark => Tokens {
                app_bar: hex(0x535353),
                panel: hex(0x535353),
                panel_darker: hex(0x424242),
                input: hex(0x454545),
                input_border: hex(0x747474),
                divider: hex(0x4b4b4b),
                text: hex(0xffffff),
                text_strong: hex(0xffffff),
                text_dim: hex(0xb0b0b0),
                text_disabled: hex(0x8a8a8a),
                icon: hex(0xc2c2c2),
                hover: hex(0x606060),
                tool_active: hex(0x303030),
                row_selected: hex(0x4a6a92),
                pasteboard: hex(0x5e5e5e),
                ruler: hex(0x1f1f1f),
                ruler_tick: hex(0x858585),
                tab_inactive: hex(0x424242),
                button: hex(0x535353),
                border: hex(0x383838),
                field_border: hex(0x747474),
                well: hex(0x303030),
                well_rim: hex(0x565656),
                ruler_text: hex(0xffffff),
                section_divider: hex(0x4b4b4b),
                tab_strip: hex(0x424242),
                ..dark
            },
            Brightness::MediumLight => Tokens {
                dark: false,
                app_bar: hex(0xa8a8a8),
                panel: hex(0xb8b8b8),
                panel_darker: hex(0xaaaaaa),
                input: hex(0xd6d6d6),
                input_border: hex(0x8c8c8c),
                divider: hex(0x9c9c9c),
                text: hex(0x1e1e1e),
                text_strong: hex(0x000000),
                text_dim: hex(0x3c3c3c),
                text_disabled: hex(0x7a7a7a),
                icon: hex(0x2a2a2a),
                hover: hex(0xc8c8c8),
                tool_active: hex(0x9a9a9a),
                row_selected: hex(0x8eb2e0),
                pasteboard: hex(0xa0a0a0),
                ruler: hex(0xc0c0c0),
                ruler_tick: hex(0x3a3a3a),
                tab_inactive: hex(0xa6a6a6),
                button: hex(0xb8b8b8),
                border: hex(0x8c8c8c),
                field_border: hex(0x7a7a7a),
                well: hex(0x9a9a9a),
                well_rim: hex(0x8a8a8a),
                ruler_text: hex(0x1e1e1e),
                section_divider: hex(0x9c9c9c),
                tab_strip: hex(0xa6a6a6),
                ..dark
            },
            Brightness::Light => Tokens {
                dark: false,
                app_bar: hex(0xe4e4e4),
                panel: hex(0xf0f0f0),
                panel_darker: hex(0xe2e2e2),
                input: hex(0xffffff),
                input_border: hex(0xb4b4b4),
                divider: hex(0xd0d0d0),
                text: hex(0x1e1e1e),
                text_strong: hex(0x000000),
                text_dim: hex(0x555555),
                text_disabled: hex(0x9a9a9a),
                icon: hex(0x2c2c2c),
                hover: hex(0xdddddd),
                tool_active: hex(0xcfcfcf),
                row_selected: hex(0xc4dbf7),
                pasteboard: hex(0xd4d4d4),
                ruler: hex(0xf2f2f2),
                ruler_tick: hex(0x4a4a4a),
                tab_inactive: hex(0xdcdcdc),
                button: hex(0xf0f0f0),
                border: hex(0xc4c4c4),
                field_border: hex(0xa8a8a8),
                well: hex(0xcfcfcf),
                well_rim: hex(0xbdbdbd),
                ruler_text: hex(0x1e1e1e),
                section_divider: hex(0xd0d0d0),
                tab_strip: hex(0xe2e2e2),
                ..dark
            },
            Brightness::HighContrast => Tokens {
                app_bar: hex(0x000000),
                panel: hex(0x000000),
                panel_darker: hex(0x000000),
                input: hex(0x000000),
                input_border: hex(0xffffff),
                divider: hex(0xffffff),
                text: hex(0xffffff),
                text_strong: hex(0xffffff),
                text_dim: hex(0xe6e6e6),
                text_disabled: hex(0xa0a0a0),
                icon: hex(0xffffff),
                hover: hex(0x3d3d00),
                tool_active: hex(0x5c5c00),
                accent: hex(0xffeb3b),
                accent_strong: hex(0xffd600),
                focus: hex(0xffeb3b),
                selection: hex(0xffd600),
                row_selected: hex(0x5c5c00),
                pasteboard: hex(0x1a1a1a),
                ruler: hex(0x000000),
                ruler_tick: hex(0xffffff),
                tab_inactive: hex(0x000000),
                button: hex(0x000000),
                border: hex(0xffffff),
                field_border: hex(0xffffff),
                well: hex(0x5c5c00),
                well_rim: hex(0xffeb3b),
                ruler_text: hex(0xffffff),
                section_divider: hex(0xffffff),
                tab_strip: hex(0x000000),
                ..dark
            },
        }
    }
}

/// Install the UI fonts for the interface language `lang` (it orders the CJK fallbacks).
pub fn install_fonts(ctx: &egui::Context, lang: &str) {
    ctx.set_fonts(font_definitions(designcraft_fonts::CRAFT_FONTS, lang));
}

/// The UI fonts: the app's own, then the craft-fonts faces from `craft` (empty without
/// `CRAFT_FONTS_DIR`) as fallbacks at the end of every family: Japanese (BIZ UDPGothic first) and
/// Simplified Chinese, in the interface language's order, then Arabic. The `arabic` families
/// put the Arabic face first for right-to-left runs. egui has no system-font discovery, so
/// without craft-fonts CJK and Arabic UI text has no glyphs.
fn font_definitions(craft: &'static [designcraft_fonts::CraftFont], lang: &str) -> FontDefinitions {
    let mut fonts = FontDefinitions::default();
    let add = |fonts: &mut FontDefinitions, name: &str, data: &'static [u8]| {
        fonts.font_data.insert(name.into(), Arc::new(FontData::from_static(data)));
    };
    add(&mut fonts, "ui", include_bytes!("../../../assets/fonts/SourceSans3-Regular.ttf"));
    add(&mut fonts, "ui-semibold", include_bytes!("../../../assets/fonts/SourceSans3-Semibold.ttf"));
    add(&mut fonts, "mono", include_bytes!("../../../assets/fonts/JetBrainsMono-Regular.ttf"));
    fonts.families.entry(FontFamily::Proportional).or_default().insert(0, "ui".into());
    fonts.families.entry(FontFamily::Monospace).or_default().insert(0, "mono".into());
    fonts.families.insert(FontFamily::Name("semibold".into()), vec!["ui-semibold".into(), "ui".into()]);
    let pick = |script: &str| -> Vec<&designcraft_fonts::CraftFont> { craft.iter().filter(|f| f.scripts.contains(&script)).collect() };
    let mut japanese = pick("Jpan");
    // BIZ UDPGothic (the UI face) first, Regular before Bold; the semibold family prefers Bold.
    japanese.sort_by_key(|f| (f.family != "BIZ UDPGothic", f.style != "Regular"));
    let chinese = pick("Hans");
    let arabic = pick("Arab");
    let name = |f: &designcraft_fonts::CraftFont| format!("craft-{}-{}", f.family, f.style);
    for f in japanese.iter().chain(&chinese).chain(&arabic) {
        add(&mut fonts, &name(f), f.bytes);
    }
    // Han characters take the forms of the interface language: Chinese faces first for zh.
    let zh = lang.starts_with("zh");
    for (family, stack) in fonts.families.iter_mut() {
        let bold = *family == FontFamily::Name("semibold".into());
        let mut ja = japanese.clone();
        if bold {
            ja.sort_by_key(|f| (f.family != "BIZ UDPGothic", f.style != "Bold"));
        }
        let (first, second) = if zh { (&chinese, &ja) } else { (&ja, &chinese) };
        stack.extend(first.iter().chain(second).chain(&arabic).map(|f| name(f)));
    }
    // Arabic runs (rtl.rs): the Arabic face first, so letters and spaces shape as one run.
    for family in ["arabic", "arabic-semibold"] {
        let ui = if family == "arabic" { "ui" } else { "ui-semibold" };
        let stack = arabic.iter().map(|f| name(f)).chain([ui.to_string(), "ui".to_string()]).chain(japanese.iter().chain(&chinese).map(|f| name(f)));
        let mut seen = std::collections::HashSet::new();
        fonts.families.insert(FontFamily::Name(family.into()), stack.filter(|n| seen.insert(n.clone())).collect());
    }
    fonts
}

pub fn semibold(size: f32) -> FontId {
    FontId::new(size, FontFamily::Name("semibold".into()))
}

pub fn apply(ctx: &egui::Context, t: &Tokens) {
    ctx.data_mut(|d| d.insert_temp(egui::Id::NULL, *t));
    let text_styles: std::collections::BTreeMap<TextStyle, FontId> = [
        (TextStyle::Small, FontId::proportional(10.0)),
        (TextStyle::Body, FontId::proportional(11.0)),
        (TextStyle::Button, FontId::proportional(11.0)),
        (TextStyle::Heading, FontId::new(15.0, FontFamily::Name("semibold".into()))),
        (TextStyle::Monospace, FontId::monospace(11.5)),
    ]
    .into();
    let mut v = if t.dark { Visuals::dark() } else { Visuals::light() };
    let r = CornerRadius::same(t.radius);
    v.panel_fill = t.panel;
    v.window_fill = t.panel;
    v.extreme_bg_color = t.input;
    v.faint_bg_color = t.panel_darker;
    v.window_stroke = Stroke::new(1.0, t.divider);
    v.window_corner_radius = CornerRadius::same(4);
    v.menu_corner_radius = CornerRadius::same(4);
    v.selection.bg_fill = t.selection;
    v.selection.stroke = Stroke::new(1.0, t.text_strong);
    v.hyperlink_color = t.focus;
    v.override_text_color = Some(t.text);
    craft_ui::theme::WidgetPalette { noninteractive: t.panel, inactive: t.button, hovered: t.hover, active: t.tool_active, open: t.hover }.apply_to(
        &mut v.widgets,
        r,
        Stroke::new(1.0, t.text),
    );
    v.widgets.noninteractive.bg_stroke = Stroke::new(1.0, t.divider);
    v.widgets.inactive.bg_stroke = Stroke::new(1.0, t.field_border);
    v.widgets.hovered.bg_stroke = Stroke::new(1.0, t.field_border);
    v.widgets.active.bg_stroke = Stroke::new(1.0, t.field_border);
    // InDesign buttons are outline-only.
    v.widgets.inactive.weak_bg_fill = Color32::TRANSPARENT;
    v.widgets.inactive.bg_fill = t.input;
    v.popup_shadow = egui::epaint::Shadow { offset: [0, 4], blur: 12, spread: 0, color: t.shadow };
    ctx.set_visuals(v);
    ctx.global_style_mut(|s| {
        s.text_styles = text_styles.clone();
        s.spacing.item_spacing = egui::vec2(6.0, 4.0);
        s.spacing.button_padding = egui::vec2(6.0, 2.0);
        s.spacing.interact_size = egui::vec2(20.0, 21.0);
        s.spacing.menu_margin = egui::Margin::same(4);
        s.animation_time = 0.06;
    });
}

impl Tokens {
    /// Tokens stored by [`apply`].
    pub fn get(ctx: &egui::Context) -> Tokens {
        ctx.data(|d| d.get_temp::<Tokens>(egui::Id::NULL)).unwrap_or_else(|| Tokens::for_brightness(Brightness::default()))
    }
}

#[cfg(test)]
mod japanese_font_tests {
    fn families() -> [egui::FontFamily; 3] {
        [egui::FontFamily::Proportional, egui::FontFamily::Monospace, egui::FontFamily::Name("semibold".into())]
    }

    fn ctx_with(craft: &'static [designcraft_fonts::CraftFont]) -> egui::Context {
        let ctx = egui::Context::default();
        ctx.set_fonts(super::font_definitions(craft, "ja"));
        let mut output = ctx.run_ui(egui::RawInput::default(), |_| {});
        output.textures_delta.clear();
        ctx
    }

    #[test]
    fn japanese_ui_glyphs_are_available_in_every_family() {
        if designcraft_fonts::CRAFT_FONTS.is_empty() {
            eprintln!("skipped: built without craft-fonts (set CRAFT_FONTS_DIR to a checkout)");
            return;
        }
        let ctx = ctx_with(designcraft_fonts::CRAFT_FONTS);
        ctx.fonts_mut(|fonts| {
            for family in families() {
                let font = egui::FontId::new(13.0, family);
                // Real glyphs (no tofu) for every character.
                for ch in "日本語の文字縦書き横書き組み方向ルビ圏点".chars() {
                    assert!(fonts.has_glyph(&font, ch), "missing {ch} in {font:?}");
                }
            }
        });
        // The UI face is BIZ UDPGothic, ahead of the Mincho faces.
        let defs = super::font_definitions(designcraft_fonts::CRAFT_FONTS, "ja");
        let stack = &defs.families[&egui::FontFamily::Proportional];
        assert_eq!(stack.iter().find(|n| n.starts_with("craft-")).map(String::as_str), Some("craft-BIZ UDPGothic-Regular"));
    }

    #[test]
    fn ui_works_without_craft_fonts() {
        let ctx = ctx_with(&[]);
        ctx.fonts_mut(|fonts| {
            // (egui's has_glyph reports false for characters of the face that also supplies the
            // replacement glyph, as Source Sans Semibold does in the semibold stack.)
            let body = egui::FontId::new(13.0, egui::FontFamily::Proportional);
            assert!(fonts.has_glyphs(&body, "DesignCraft"));
            for family in families() {
                let font = egui::FontId::new(13.0, family);
                let galley = fonts.layout_no_wrap("日本語 DesignCraft".into(), font, egui::Color32::WHITE);
                assert!(galley.size().x > 0.0);
            }
        });
        // And the real installer works with whatever this build has.
        super::install_fonts(&egui::Context::default(), "ja");
    }
}

#[cfg(test)]
mod ukrainian_font_tests {
    #[test]
    fn every_catalog_cyrillic_glyph_exists_in_the_bundled_ui_fonts() {
        // Check the actual installed face data, without craft-fonts or system fonts.
        // egui's has_glyph can report false for the face providing its replacement glyph.
        let definitions = super::font_definitions(&[], "uk");
        let db = designcraft_fonts::FontDb::with_font_dirs(Vec::new());
        db.set_system_fallback(false);
        let characters: std::collections::HashSet<_> = crate::i18n::ukrainian_catalog()
            .iter()
            .flat_map(|text| text.chars())
            .chain("ҐґЄєІіЇї".chars())
            .filter(|c| matches!(*c, '\u{0400}'..='\u{052f}'))
            .collect();
        for (name, family, style) in
            [("ui", "Source Sans 3", "Regular"), ("ui-semibold", "Source Sans 3", "Semibold"), ("mono", "JetBrains Mono", "Regular")]
        {
            let data = &definitions.font_data[name].font;
            let face = db.face(family, style);
            assert_eq!(face.data(), data.as_ref(), "configured UI face {name}");
            for character in &characters {
                assert_ne!(face.glyph_for(*character), 0, "missing {character} in {name}");
            }
        }
    }
}

#[cfg(test)]
mod ufrgs_theme_tests {
    use super::*;

    fn luminance(c: Color32) -> f32 {
        let f = |x: u8| {
            let s = f32::from(x) / 255.0;
            if s <= 0.04045 { s / 12.92 } else { ((s + 0.055) / 1.055).powf(2.4) }
        };
        0.2126 * f(c.r()) + 0.7152 * f(c.g()) + 0.0722 * f(c.b())
    }

    fn contrast(a: Color32, b: Color32) -> f32 {
        let (x, y) = (luminance(a), luminance(b));
        (x.max(y) + 0.05) / (x.min(y) + 0.05)
    }

    #[test]
    fn ufrgs_noite_is_the_default_and_both_ufrgs_themes_come_first() {
        assert_eq!(Brightness::default(), Brightness::UfrgsNoite);
        assert_eq!(&Brightness::ALL[..2], &[Brightness::UfrgsNoite, Brightness::UfrgsDia]);
        assert_eq!(crate::UiState::default().brightness, Brightness::UfrgsNoite);
        // The upstream themes stay available.
        for b in [Brightness::Dark, Brightness::MediumDark, Brightness::MediumLight, Brightness::Light, Brightness::HighContrast] {
            assert!(Brightness::ALL.contains(&b), "{b:?}");
        }
    }

    #[test]
    fn every_theme_parses_by_id_and_label() {
        for b in Brightness::ALL {
            assert_eq!(Brightness::parse(b.id()), Some(b));
            assert_eq!(Brightness::parse(b.label()), Some(b));
        }
        assert_eq!(Brightness::parse("ufrgsNoite"), Some(Brightness::UfrgsNoite));
        assert_eq!(Brightness::parse("UFRGS Dia"), Some(Brightness::UfrgsDia));
        let ids: std::collections::HashSet<_> = Brightness::ALL.iter().map(|b| b.id()).collect();
        assert_eq!(ids.len(), Brightness::ALL.len(), "unique ids");
    }

    #[test]
    fn ufrgs_themes_use_the_brand_tones() {
        use fisilab_brand::palette::{DIA, NOITE};
        for (b, p) in [(Brightness::UfrgsNoite, &NOITE), (Brightness::UfrgsDia, &DIA)] {
            let t = Tokens::for_brightness(b);
            assert_eq!(t.dark, p.escuro, "{b:?}");
            assert_eq!((t.panel, t.pasteboard, t.input, t.hover), (p.painel, p.fundo, p.elevado, p.hover), "{b:?}");
            assert_eq!((t.text, t.text_dim, t.accent, t.focus), (p.texto, p.texto_fraco, p.destaque, p.foco), "{b:?}");
            // Layout is the upstream default's, only the colours change.
            assert_eq!(t.radius, Tokens::for_brightness(Brightness::MediumDark).radius);
        }
    }

    #[test]
    fn ufrgs_themes_keep_text_readable() {
        for b in [Brightness::UfrgsNoite, Brightness::UfrgsDia] {
            let t = Tokens::for_brightness(b);
            for bg in [t.panel, t.panel_darker, t.input, t.hover, t.row_selected, t.selection, t.pasteboard, t.app_bar] {
                assert!(contrast(t.text, bg) >= 4.5, "{b:?}: text on {bg:?} is {:.2}", contrast(t.text, bg));
                assert!(contrast(t.text_strong, bg) >= 4.5, "{b:?}: strong text on {bg:?}");
            }
            for bg in [t.panel, t.input] {
                assert!(contrast(t.text_dim, bg) >= 3.0, "{b:?}: dim text on {bg:?}");
                assert!(contrast(t.focus, bg) >= 3.0, "{b:?}: links on {bg:?}");
            }
            // White labels on the primary (accent) buttons.
            assert!(contrast(Color32::WHITE, t.accent_strong) >= 4.5, "{b:?}: button label");
            assert!(contrast(t.ruler_text, t.ruler) >= 4.5, "{b:?}: ruler numbers");
        }
    }

    #[test]
    fn applying_a_ufrgs_theme_sets_egui_visuals() {
        let ctx = egui::Context::default();
        let t = Tokens::for_brightness(Brightness::UfrgsNoite);
        apply(&ctx, &t);
        let v = ctx.global_style().visuals.clone();
        assert!(v.dark_mode);
        assert_eq!(v.panel_fill, t.panel);
        assert_eq!(v.hyperlink_color, t.focus);
        assert_eq!(v.selection.bg_fill, t.selection);
        assert_eq!(Tokens::get(&ctx).panel, t.panel);
    }
}
