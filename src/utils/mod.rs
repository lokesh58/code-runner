use phf::{phf_map, Map};

use crate::models::{LanguageDetails, RunCodeParams, RunCodeResult};

struct LanguageInfo {
    display: &'static str,
}

static LANGUAGE_INFO_MAP: Map<&'static str, LanguageInfo> = phf_map! {
    "c" => LanguageInfo {
        display: "C",
    },
    "cpp" => LanguageInfo {
        display: "C++",
    },
    "js" => LanguageInfo {
        display: "JavaScript",
    },
    "ts" => LanguageInfo {
        display: "TypeScript",
    },
    "py" => LanguageInfo {
        display: "Python",
    },
    "rs" => LanguageInfo {
        display: "Rust",
    },
};

pub fn get_all_languages() -> Vec<LanguageDetails> {
    LANGUAGE_INFO_MAP
        .entries()
        .map(|(lang, info)| LanguageDetails {
            language: lang.to_string(),
            display: info.display.to_string(),
        })
        .collect()
}

pub async fn run_code(params: RunCodeParams) -> RunCodeResult {
    println!(
        "Received params:\nlanguage: {},\ncode: {},\ninput: {}",
        params.language,
        params.code,
        params.input.clone().unwrap_or(String::from("<empty>"))
    );
    RunCodeResult {
        output: String::from("TODO: Run code to get actual output"),
        error: String::from("TODO: Run code to get actual error"),
    }
}
