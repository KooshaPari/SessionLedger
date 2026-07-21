//! First-run “Open corpus…” CTA — corpus file picker (web) or quick-start docs (desktop).

/// Repo-relative quick-start doc path (HELP.md cross-link).
pub const QUICKSTART_CORPUS_DOC: &str = "docs/guides/quick-start/QUICKSTART.md";

/// Public quick-start URL opened when the desktop viewer cannot show a picker.
pub const QUICKSTART_URL: &str =
    "https://github.com/KooshaPari/SessionLedger/blob/main/docs/guides/quick-start/QUICKSTART.md";

/// Stable DOM id for the hidden Forge DB file input (web).
pub const CORPUS_PICKER_INPUT_ID: &str = "sl-corpus-picker-input";

/// localStorage key recording the last picked Forge DB file name (web hint only).
pub const FORGE_DB_HINT_STORAGE_KEY: &str = "sl-viewer-forge-db-hint";

/// Install the web corpus-picker bridge and open the Forge DB file chooser.
#[cfg(feature = "web")]
pub fn trigger_open_corpus() {
    use dioxus::document;

    let script = format!(
        r#"
        (function() {{
          const quickstart = {quickstart:?};
          if (!window.__slCorpusCtaBridge) {{
            window.__slCorpusCtaBridge = true;
            let input = document.getElementById({input_id:?});
            if (!input) {{
              input = document.createElement('input');
              input.type = 'file';
              input.id = {input_id:?};
              input.accept = '.db,.sqlite,.sqlite3';
              input.style.display = 'none';
              input.setAttribute('data-testid', 'corpus-picker-input');
              input.addEventListener('change', () => {{
                const file = input.files && input.files[0];
                if (file) {{
                  window.localStorage.setItem({storage_key:?}, file.name);
                  document.documentElement.dataset.slCorpusSelected = 'true';
                }}
              }});
              document.body.appendChild(input);
            }}
            window.__slOpenCorpusCta = () => {{
              input.click();
            }};
          }}
          if (typeof window.__slOpenCorpusCta === 'function') {{
            window.__slOpenCorpusCta();
            return;
          }}
          window.open(quickstart, '_blank', 'noopener,noreferrer');
        }})();
        "#,
        quickstart = QUICKSTART_URL,
        input_id = CORPUS_PICKER_INPUT_ID,
        storage_key = FORGE_DB_HINT_STORAGE_KEY,
    );
    let _ = document::eval(&script);
}

/// Desktop builds open the quick-start runbook (no native picker in this lane).
#[cfg(all(feature = "desktop", not(target_arch = "wasm32")))]
pub fn trigger_open_corpus() {
    open_quickstart_desktop();
}

/// Headless / test builds: no-op so unit tests stay hermetic.
#[cfg(not(any(feature = "web", all(feature = "desktop", not(target_arch = "wasm32")))))]
pub fn trigger_open_corpus() {}

#[cfg(all(feature = "desktop", not(target_arch = "wasm32")))]
fn open_quickstart_desktop() {
    let url = QUICKSTART_URL;
    let result = if cfg!(target_os = "windows") {
        std::process::Command::new("cmd").args(["/C", "start", "", url]).spawn()
    } else if cfg!(target_os = "macos") {
        std::process::Command::new("open").arg(url).spawn()
    } else {
        std::process::Command::new("xdg-open").arg(url).spawn()
    };
    if let Err(err) = result {
        eprintln!(
            "[sl-viewer] could not open quick-start docs ({err}); see {QUICKSTART_CORPUS_DOC}"
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quickstart_url_points_at_repo_quickstart() {
        assert!(QUICKSTART_URL.contains("KooshaPari/SessionLedger"));
        assert!(QUICKSTART_URL.contains("QUICKSTART.md"));
    }

    #[test]
    fn corpus_picker_dom_ids_are_stable() {
        assert_eq!(CORPUS_PICKER_INPUT_ID, "sl-corpus-picker-input");
        assert_eq!(FORGE_DB_HINT_STORAGE_KEY, "sl-viewer-forge-db-hint");
    }
}
