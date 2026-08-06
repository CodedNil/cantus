use std::{env, path::PathBuf, process::ExitCode};

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("isthmus: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), String> {
    let mut arguments = env::args_os().skip(1);
    if arguments.next().as_deref() != Some("build".as_ref()) {
        return Err(String::from("usage: isthmus build [package]"));
    }
    let start = arguments.next().map_or_else(
        || env::current_dir().map_err(|error| error.to_string()),
        |path| Ok(PathBuf::from(path)),
    )?;
    if arguments.next().is_some() {
        return Err(String::from("usage: isthmus build [package]"));
    }

    let crate_dir = isthmus_build::find_shader_crate(&start)?;
    let (output, changed) = isthmus_build::build_shader(&crate_dir)?;
    if changed {
        println!("wrote {}", output.display());
    } else {
        println!("{} is up to date", output.display());
    }
    Ok(())
}
