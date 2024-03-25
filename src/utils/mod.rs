use std::{
    fs::File,
    io::Write,
    process::{Command, Stdio},
};

use actix_web::{error, Error};
use phf::{phf_map, Map};
use tempfile::tempdir;

use crate::models::{LanguageDetails, RunCodeParams, RunCodeResult};

struct LanguageInfo {
    display: &'static str,
    code_filename: &'static str,
    compile_cmd: Option<&'static str>,
    execute_cmd: &'static str,
}

static LANGUAGE_INFO_MAP: Map<&'static str, LanguageInfo> = phf_map! {
    "c" => LanguageInfo {
        display: "C",
        code_filename: "main.c",
        compile_cmd: Some("gcc -o out main.c"),
        execute_cmd: "./out",
    },
    "cpp" => LanguageInfo {
        display: "C++",
        code_filename: "main.cpp",
        compile_cmd: Some("g++ -o out main.cpp"),
        execute_cmd: "./out",
    },
    "js" => LanguageInfo {
        display: "JavaScript",
        code_filename: "index.js",
        compile_cmd: None,
        execute_cmd: "node index.js",
    },
    "ts" => LanguageInfo {
        display: "TypeScript",
        code_filename: "index.ts",
        compile_cmd: None,
        execute_cmd: "ts-node index.ts",
    },
    "py" => LanguageInfo {
        display: "Python",
        code_filename: "main.py",
        compile_cmd: None,
        execute_cmd: "python main.py",
    },
    "rs" => LanguageInfo {
        display: "Rust",
        code_filename: "main.rs",
        compile_cmd: Some("rustc -o out main.rs"),
        execute_cmd: "./out",
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

pub async fn run_code(params: RunCodeParams) -> Result<RunCodeResult, Error> {
    let Some(lang_info) = LANGUAGE_INFO_MAP.get(params.language.as_str()) else {
        return Err(error::ErrorBadRequest("Unsupported or Invalid Language"));
    };

    let temp_dir = tempdir()?;
    let mut code_file = File::create(temp_dir.path().join(lang_info.code_filename))?;
    code_file.write_all(params.code.as_bytes())?;
    let mut input_file = File::create(temp_dir.path().join("input.txt"))?;
    if let Some(input_text) = params.input {
        input_file.write_all(input_text.as_bytes())?;
    }

    let mut out = String::new();
    let mut err = String::new();

    if let Some(compile_cmd) = lang_info.compile_cmd {
        let cmd_tokens: Vec<&str> = compile_cmd.split(' ').collect();
        let output = Command::new(cmd_tokens[0])
            .args(&cmd_tokens[1..])
            .current_dir(temp_dir.path())
            .output()?;

        out.push_str(String::from_utf8_lossy(&output.stdout).to_string().as_str());
        err.push_str(String::from_utf8_lossy(&output.stderr).to_string().as_str());
    }

    if err.len() == 0 {
        let input_process = Command::new("cat")
            .arg("input.txt")
            .current_dir(temp_dir.path())
            .stdout(Stdio::piped())
            .spawn()?;

        let cmd_tokens: Vec<&str> = lang_info.execute_cmd.split(' ').collect();
        let output = Command::new(cmd_tokens[0])
            .args(&cmd_tokens[1..])
            .current_dir(temp_dir.path())
            .stdin(Stdio::from(input_process.stdout.unwrap()))
            .output()?;

        out.push_str(String::from_utf8_lossy(&output.stdout).to_string().as_str());
        err.push_str(String::from_utf8_lossy(&output.stderr).to_string().as_str());
    }

    Ok(RunCodeResult {
        output: out,
        error: err,
    })
}
