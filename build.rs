use prost_build::compile_protos;
use std::collections::HashSet;
use std::env::var as os_var;
use std::fs::{copy, create_dir_all, read_to_string, remove_dir_all, remove_file, write, File};
use std::io::{BufRead, BufReader, Lines, Result};
use std::path::Path;

const ENV_OUT: &str = "OUT_DIR";
const PATH_PROTOS_CONFIG: &str = "tensorflow/core/protobuf/config.proto";
const PATH_PROTOS_DESTINATION_BASE: &str = "src/protos";
const PATH_PROTOS_DESTINATION_RESULT: &str = "src/tensorflow.rs";
const PATH_PROTOS_NAME_RESULT: &str = "tensorflow.rs";
const PATH_PROTOS_SOURCE_BASE: &str = "submodule-tensorflow";

fn regenerate_purge_destination() {
    let _ = remove_dir_all(PATH_PROTOS_DESTINATION_BASE);
    let _ = create_dir_all(PATH_PROTOS_DESTINATION_BASE);
}

fn regenerate_readlines<P: AsRef<Path>>(filename: P) -> Result<Lines<BufReader<File>>> {
    let file = File::open(filename)?;

    Ok(BufReader::new(file).lines())
}

fn regenerate_get_imports<P: AsRef<Path>>(filename: P) -> Result<Vec<(String, String)>> {
    let lines = regenerate_readlines(filename)?;
    let mut result = Vec::new();

    for readline in lines.flatten() {
        if readline.starts_with("import")
            && readline.ends_with(';')
            && readline.contains("tensorflow")
        {
            let import_file = readline
                .replace(['\"', ';'], "")
                .split(' ')
                .last()
                .unwrap()
                .to_string();
            let source = format!("{PATH_PROTOS_SOURCE_BASE}/{import_file}");
            let destination = format!("{PATH_PROTOS_DESTINATION_BASE}/{import_file}");
            result.push((source, destination));
        }
    }

    Ok(result)
}

fn regenerate_get_imports_recursively() -> Result<HashSet<(String, String)>> {
    let mut imports = HashSet::new();
    imports.insert((
        format!("{PATH_PROTOS_SOURCE_BASE}/{PATH_PROTOS_CONFIG}"),
        format!("{PATH_PROTOS_DESTINATION_BASE}/{PATH_PROTOS_CONFIG}"),
    ));

    loop {
        let import_lines = Vec::from_iter(imports.clone());
        let mut has_changes = false;

        for (source, _) in import_lines {
            let new_imports = regenerate_get_imports(source)?;

            for new_import in new_imports {
                has_changes |= imports.insert(new_import);
            }
        }

        if !has_changes {
            break;
        }
    }

    Ok(imports)
}

fn regenerate_copy_protos() -> Result<()> {
    let copy_manifest = regenerate_get_imports_recursively()?;

    for (source, destination) in copy_manifest {
        if let Some(destination_dir) = Path::new(&destination).parent() {
            create_dir_all(destination_dir)?;
        }

        copy(source, destination)?;
    }

    Ok(())
}

fn regenerate_build() -> Result<()> {
    let config_file = [format!(
        "{PATH_PROTOS_DESTINATION_BASE}/{PATH_PROTOS_CONFIG}"
    )];
    compile_protos(&config_file, &[PATH_PROTOS_DESTINATION_BASE])?;

    Ok(())
}

fn regenerate_copy() -> Result<()> {
    let source_result = format!("{}/{}", os_var(ENV_OUT).unwrap(), PATH_PROTOS_NAME_RESULT);
    let _ = remove_file(PATH_PROTOS_DESTINATION_RESULT);
    let generated_content = read_to_string(source_result)?;
    write(PATH_PROTOS_DESTINATION_RESULT, generated_content)?;

    Ok(())
}

fn main() -> Result<()> {
    regenerate_purge_destination();
    regenerate_copy_protos()?;
    regenerate_build()?;
    regenerate_copy()?;

    Ok(())
}
