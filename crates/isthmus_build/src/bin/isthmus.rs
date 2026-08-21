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
    let (Some(command), Some(package), None) = (arguments.next(), arguments.next(), arguments.next()) else {
        return Err(String::from("usage: isthmus build <package>"));
    };
    if command != "build" {
        return Err(String::from("usage: isthmus build <package>"));
    }

    let (output, changed) = isthmus_build::compiler::build_shader(&PathBuf::from(package))?;
    if changed {
        println!("wrote {}", output.display());
    } else {
        println!("{} is up to date", output.display());
    }
    Ok(())
}
