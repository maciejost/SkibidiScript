#![allow(clippy::print_stdout)]
use std::{
    fs::{self, OpenOptions},
    io::{self},
    path::{Path, PathBuf},
};

use oxc_allocator::Allocator;
use oxc_codegen::CodeGenerator;
use oxc_parser::Parser;
use oxc_semantic::SemanticBuilder;
use oxc_span::SourceType;
use oxc_transformer::{EnvOptions, HelperLoaderMode, TransformOptions, Transformer};
use pico_args::Arguments;

fn create_js_file(file_path: &Path) -> io::Result<PathBuf> {
    let new_file_path = file_path.with_extension("js");

    OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&new_file_path)?;

    Ok(new_file_path)
}

pub fn main(input_path: &PathBuf) {
    let mut args = Arguments::from_env();
    let targets: Option<String> = args.opt_value_from_str("--targets").unwrap_or(None);
    let target: Option<String> = args.opt_value_from_str("--target").unwrap_or(None);

    let source_text =
        std::fs::read_to_string(input_path).unwrap_or_else(|err| panic!("Path not found.\n{err}"));
    let allocator = Allocator::default();
    let source_type = SourceType::from_path(input_path).unwrap();

    let ret = Parser::new(&allocator, &source_text, source_type).parse();

    if !ret.errors.is_empty() {
        println!("Parser Errors:");
        for error in ret.errors {
            let error = error.with_source_code(source_text.clone());
            println!("{error:?}");
        }
    }

    let mut program = ret.program;

    let ret = SemanticBuilder::new()
        .with_excess_capacity(2.0)
        .build(&program);

    if !ret.errors.is_empty() {
        println!("Semantic Errors:");
        for error in ret.errors {
            let error = error.with_source_code(source_text.clone());
            println!("{error:?}");
        }
    }

    let (symbols, scopes) = ret.semantic.into_symbol_table_and_scope_tree();

    let mut transform_options = if let Some(query) = &targets {
        TransformOptions {
            env: EnvOptions::from_browserslist_query(query).unwrap(),
            ..TransformOptions::default()
        }
    } else if let Some(target) = &target {
        TransformOptions::from_target(target).unwrap()
    } else {
        TransformOptions::enable_all()
    };

    transform_options.helper_loader.mode = HelperLoaderMode::External;

    let ret = Transformer::new(&allocator, input_path, &transform_options)
        .build_with_symbols_and_scopes(symbols, scopes, &mut program);

    if !ret.errors.is_empty() {
        println!("Transformer Errors:");
        for error in ret.errors {
            let error = error.with_source_code(source_text.clone());
            println!("{error:?}");
        }
    }

    let new_file = create_js_file(input_path).unwrap();

    let printed = CodeGenerator::new().build(&program).code;

    fs::write(&new_file, printed).unwrap_or_else(|err| panic!("Failed to write to file.\n{err}"));
}
