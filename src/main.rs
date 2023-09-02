use clap::Parser;

#[derive(Parser)]
struct Args {
    path: std::path::PathBuf,
}

const ANNOTATIONS: [&str; 5] = ["FIXME", "HACK", "NOTE", "TODO", "XXX"];

fn process_line(line: &str) -> Option<String> {
    if ANNOTATIONS
        .iter()
        .any(|&annotation| line.starts_with(annotation))
    {
        None
    } else {
        Some(line.to_string())
    }
}

fn main() {
    let args = Args::parse();

    let text = std::fs::read_to_string(&args.path).expect("Could not read file");
    let lines = text.lines().map(process_line).filter_map(|line| line);
    let text = lines.collect::<Vec<_>>().join("\n");

    println!("{}", text);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_removes_todo_lines() {
        assert_eq!(process_line("TODO Do something"), None);
    }

    #[test]
    fn it_removes_note_lines() {
        assert_eq!(process_line("NOTE Some notes"), None);
    }

    #[test]
    fn it_keeps_other_lines() {
        let content = "Some content";
        assert_eq!(process_line(content), Some(content.to_string()));
    }
}
