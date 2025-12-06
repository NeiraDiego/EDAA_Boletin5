use std::env;
use std::fs;
use std::process;

fn config(
    mut args: impl Iterator<Item = String>,
) -> Result<(String, String, String), &'static str> {
    args.next();

    match args.next() {
        Some(arg) => {
            match fs::read_to_string(&arg) {
                Ok(contents) => {
                    let mut name: Vec<_> = arg.split('.').collect();
                    name.pop();
                    let name = name.join(".");

                    let unit = match args.next() {
                        Some(arg) => arg,
                        None => return Err("Didn't get a unit"),
                    };

                    Ok((contents, name, unit))
                },
                _ => Err("Could not open file"),
            }
        },
        None => Err("Didn't get a file path"),
    }
}

fn main() {
    let (file_contents, file_name, file_unit) = config(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    let mut lines = file_contents.lines();

    let header = lines.next();

    if let Some(header) = header {
        let headers: Vec<_> = header.split(',').map(|h| format!("\\texttt{{{}}}", h)).collect();
        let c = "|c".repeat(headers.len());

        println!("\\begin{{tabular}}{{{c}|}}\n    \\hline");

        println!("    \\multicolumn{{{}}}{{|c|}}{{\\texttt{{{}}}}} \\\\\n    \\hline",
            headers.len(),
            format!("{} [{}]", file_name.replace('_', "\\_"), file_unit),
        );

        let header = headers.join(" & ").replace('_', "\\_");

        println!("    {header} \\\\\n    \\hline");

        for line in lines {
            let line: Vec<_> = line.split(',').collect();
            let row = line.join(" & ");

            println!("    {row} \\\\\n    \\hline");
        }

        println!("\\end{{tabular}}");
    }
}
