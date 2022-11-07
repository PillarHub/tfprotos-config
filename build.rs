use std::collections::HashSet;
use std::env::var;
use std::fs::{copy, create_dir_all, remove_dir_all, File};
use std::io::{BufRead, BufReader, Lines, Result as IOResult};
use std::path::Path;

const ENV_CARGO_REGENERATE: &str = "CARGO_REGENERATE";
const PATH_PROTOS_DESTINATION_BASE: &str = "tensorflow";
const PATH_PROTOS_SOURCE_BASE: &str = "submodule-tensorflow";
const PATH_PROTOS_CONFIG: &str = "tensorflow/core/protobuf/config.proto";

fn regenerate_purge_destination() {
    let _ = remove_dir_all(PATH_PROTOS_DESTINATION_BASE);
    let _ = create_dir_all(PATH_PROTOS_DESTINATION_BASE);
}

fn regenerate_readlines<P: AsRef<Path>>(filename: P) -> IOResult<Lines<BufReader<File>>> {
    let file = File::open(filename)?;

    Ok(BufReader::new(file).lines())
}

fn regenerate_get_imports<P: AsRef<Path>>(filename: P) -> Vec<(String, String)> {
    let lines = regenerate_readlines(filename).unwrap();
    let mut result = Vec::new();

    for readline in lines.flatten() {
        if readline.starts_with("import")
            && readline.ends_with(';')
            && readline.contains("tensorflow")
        {
            let destination = readline
                .replace('\"', "")
                .replace(';', "")
                .split(' ')
                .last()
                .unwrap()
                .to_string();
            let source = format!("{PATH_PROTOS_SOURCE_BASE}/{destination}");
            result.push((source, destination));
        }
    }

    result
}

fn regenerate_get_imports_recursively() -> HashSet<(String, String)> {
    let mut imports = HashSet::new();
    imports.insert((
        format!("{PATH_PROTOS_SOURCE_BASE}/{PATH_PROTOS_CONFIG}"),
        PATH_PROTOS_CONFIG.to_string(),
    ));

    loop {
        let import_lines = Vec::from_iter(imports.clone());
        let mut has_changes = false;

        for (source, _) in import_lines {
            let new_imports = regenerate_get_imports(source);

            for new_import in new_imports {
                has_changes |= imports.insert(new_import);
            }
        }

        if !has_changes {
            break;
        }
    }

    imports
}

fn regenerate_copy_protos() {
    let copy_manifest = regenerate_get_imports_recursively();

    for (source, destination) in copy_manifest {
        if let Some(destination_dir) = Path::new(&destination).parent() {
            let _ = create_dir_all(destination_dir);
        }

        let _ = copy(source, destination);
    }
}

fn try_regenerate_protos() {
    let regenerate = if let Ok(cargo_regenerate) = var(ENV_CARGO_REGENERATE) {
        matches!(
            cargo_regenerate.to_lowercase().as_str(),
            "1" | "true" | "yes"
        )
    } else {
        false
    };

    if !regenerate {
        return;
    }

    regenerate_purge_destination();
    regenerate_copy_protos();
}

fn main() {
    try_regenerate_protos();
}
