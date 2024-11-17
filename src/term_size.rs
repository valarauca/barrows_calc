use std::process::Command;

pub struct TerminalSize {
    pub lines: u32,
    pub columns: u32,
}

impl TerminalSize {
    pub fn from_values(lines: u32, columns: u32) -> Self {
        Self {
            lines: lines,
            columns: columns
        }
    }

    pub fn new() -> Self {
        let columns = run_cmd(&["tput", "col"]).unwrap_or_else(|| 80u32);
        let lines = run_cmd(&["tput", "lines"]).unwrap_or_else(|| 24u32);

        Self {
            lines: lines,
            columns: columns,
        }
    }
}


fn run_cmd(arg: &[&str]) -> Option<u32> {
    get_output(
        Command::new(&arg[0])
            .args( arg[1..].iter())
    )
}

fn get_output(arg: &mut Command) -> Option<u32> {
    let output = match arg.output() {
        Ok(output) => output,
        Err(_) => return None,
    };
    if !output.status.success() {
        return None;
    }
    let stdout = match String::from_utf8(output.stdout) {
        Ok(output) => output,
        Err(_) => return None,
    };
    match u32::from_str_radix(stdout.trim(), 10) {
        Ok(value) => Some(value),
        Err(_) => None
    }
}
