// Copyright: Ankitects Pty Ltd and contributors
// License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html

use std::env;
use std::fmt::Write;
use std::path::PathBuf;

use anki_io::create_dir_all;
use anki_io::write_file_if_changed;
use anyhow::Result;
use inflections::Inflect;
use itertools::Itertools;

use crate::extract::Module;
use crate::extract::Variable;
use crate::extract::VariableKind;

pub fn write_py_interface(modules: &[Module]) -> Result<()> {
    let mut out = header();

    render_methods(modules, &mut out);
    render_legacy_enum(modules, &mut out);

    if let Ok(path) = env::var("STRINGS_PY") {
        let path = PathBuf::from(path);
        create_dir_all(path.parent().unwrap())?;
        write_file_if_changed(path, out)?;
    }

    Ok(())
}

fn render_legacy_enum(modules: &[Module], out: &mut String) {
    out.push_str("class LegacyTranslationEnum:\n");
    for (mod_idx, module) in modules.iter().enumerate() {
        for (str_idx, translation) in module.translations.iter().enumerate() {
            let upper = translation.key.replace('-', "_").to_upper_case();
            writeln!(out, r#"    {upper} = ({mod_idx}, {str_idx})"#).unwrap();
        }
    }
}

fn render_methods(modules: &[Module], out: &mut String) {
    for (mod_idx, module) in modules.iter().enumerate() {
        for (str_idx, translation) in module.translations.iter().enumerate() {
            let text = &translation.text;
            let key = &translation.key;
            let func_name = key.replace('-', "_").to_snake_case();
            let arg_types = get_arg_types(&translation.variables);
            let args = get_args(&translation.variables);
            writeln!(
                out,
                r#"
    def {func_name}(self, {arg_types}) -> str:
        r''' {text} '''
        return self._translate({mod_idx}, {str_idx}, {{{args}}})
"#,
            )
            .unwrap();
        }
    }
}

fn header() -> String {
    "# Copyright: Ankitects Pty Ltd and contributors
# License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html

# This file is automatically generated from the *.ftl files.

from __future__ import annotations

from typing import Union

FluentVariable = Union[str, int, float]

class GeneratedTranslations:
    def _translate(self, module: int, translation: int, args: dict) -> str:
        raise Exception('not implemented')

"
    .to_string()
}

fn get_arg_types(args: &[Variable]) -> String {
    let args = args
        .iter()
        .map(|arg| format!("{}: {}", arg.name.to_snake_case(), arg_kind(&arg.kind)))
        .join(", ");
    if args.is_empty() {
        "".into()
    } else {
        args
    }
}

fn get_args(args: &[Variable]) -> String {
    args.iter()
        .map(|arg| format!("\"{}\": {}", arg.name, arg.name.to_snake_case()))
        .join(", ")
}

fn arg_kind(kind: &VariableKind) -> &str {
    match kind {
        VariableKind::Int => "int",
        VariableKind::Float => "float",
        VariableKind::String => "str",
        VariableKind::Any => "FluentVariable",
    }
}
