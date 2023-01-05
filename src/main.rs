use stap::Module;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();

    if args.len() != 2 {
        eprintln!("Usage: stap FILE");
        std::process::exit(1);
    }

    let _exe_name = &args[0];
    let source_file = &args[1];

    let content = match std::fs::read_to_string(source_file) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Unable to read {}. {:?}", source_file, e);
            std::process::exit(1);
        }
    };

    let module = Module::parse(&content);

    println!("{:?}", module);
}
