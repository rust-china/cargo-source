use cargo_source::*;

fn main() -> R<()> {
    let mut args: Vec<String> = std::env::args().collect();
    if args.len() >= 2 && args[1] == "source" {
        args.remove(1);
    }
    let mut cli = Cli::default();
    if let Err(err) = cli.run_command(args) {
        println!("{}", err.to_string());
    }
    Ok(())
}
