//! Suíte FisiLab and project links (Help menu, About, start screen, CLI, MCP).
//!
//! FisiLab Layout is the Suíte FisiLab (FisiLab — LARFIC · UFRGS, with the CTA) build of DesignCraft.
//! The upstream community links were replaced by the suite's; the original project keeps its
//! credit link. The URLs mirror `fisilab_brand` (which this layer can't depend on: it uses egui).

use serde_json::{Value, json};

/// The app's slug in the Suíte FisiLab.
pub const APP: &str = "fisilab-layout";
/// The Suíte FisiLab page (source code, installers, credits).
pub const SUITE: &str = "https://github.com/mrcandrade/larfic/tree/main/suite-fisilab";
/// The manifesto for free technologies.
pub const MANIFESTO: &str = "https://github.com/mrcandrade/larfic/blob/main/suite-fisilab/MANIFESTO.md";
/// The original project (DesignCraft, by the ArtCraft Team and contributors).
pub const UPSTREAM: &str = "https://github.com/storytold/designcraft";
/// FisiLab Layout's source repository.
pub const GITHUB: &str = "https://github.com/mrcandrade/fisilab-layout";
/// Where to report bugs and request features.
pub const ISSUES: &str = "https://github.com/mrcandrade/fisilab-layout/issues";

/// Every link key, in display order.
pub const KEYS: [&str; 5] = ["suite", "manifesto", "upstream", "github", "issues"];

/// All links as JSON (the `app.links` command).
pub fn all() -> Value {
    json!({
        "suite": SUITE,
        "manifesto": MANIFESTO,
        "upstream": UPSTREAM,
        "github": GITHUB,
        "issues": ISSUES,
    })
}

/// A link by key (`suite`, `manifesto`, `upstream`, `github`, `issues`).
pub fn get(key: &str) -> Option<&'static str> {
    Some(match key {
        "suite" => SUITE,
        "manifesto" => MANIFESTO,
        "upstream" => UPSTREAM,
        "github" => GITHUB,
        "issues" => ISSUES,
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn links_follow_the_app_slug() {
        assert!(GITHUB.ends_with(&format!("/mrcandrade/{APP}")));
        assert!(ISSUES.starts_with(GITHUB));
        assert!(UPSTREAM.ends_with("/storytold/designcraft"));
        for k in KEYS {
            assert!(get(k).is_some_and(|u| u.starts_with("https://")), "{k}");
            assert_eq!(all()[k], get(k).unwrap());
        }
        assert_eq!(all().as_object().map(|o| o.len()), Some(KEYS.len()));
    }
}
