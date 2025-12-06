use std::env;
use std::fs;
use std::process;

#[derive(Debug)]
struct CsvData {
    name: String,
    n_values: Vec<usize>,
    time_values: Vec<f64>,
}

fn read_csv(file_path: &str) -> Result<CsvData, String> {
    let contents = fs::read_to_string(file_path)
        .map_err(|_| format!("Could not open file: {}", file_path))?;

    let mut lines = contents.lines();

    // Skip header
    lines.next();

    let mut n_values = Vec::new();
    let mut time_values = Vec::new();

    for line in lines {
        let parts: Vec<_> = line.split(',').collect();
        if parts.len() >= 2 {
            if let Ok(n) = parts[0].parse::<usize>() {
                if let Ok(time) = parts[1].parse::<f64>() {
                    n_values.push(n);
                    time_values.push(time);
                }
            }
        }
    }

    // Extract name from file path (remove .csv extension)
    let name = file_path.trim_end_matches(".csv").to_string();

    Ok(CsvData {
        name,
        n_values,
        time_values,
    })
}

fn generate_latex_plot(data_sets: Vec<CsvData>) {
    println!("\\begin{{tikzpicture}}");
    println!("    \\begin{{axis}}[");
    println!("        xlabel={{Tamaño de matriz (N)}},");
    println!("        ylabel={{Tiempo (segundos)}},");
    println!("        legend pos=north west,");
    println!("        grid=major,");
    println!("        width=12cm,");
    println!("        height=8cm,");
    println!("    ]");

    // Define colors for each plot
    let colors = ["blue", "red", "green!50!black"];
    let markers = ["*", "square*", "triangle*"];

    for (idx, data) in data_sets.iter().enumerate() {
        let color = colors[idx % colors.len()];
        let marker = markers[idx % markers.len()];

        println!("        \\addplot[");
        println!("            color={},", color);
        println!("            mark={},", marker);
        println!("            thick,");
        println!("        ]");
        println!("        coordinates {{");

        for (n, time) in data.n_values.iter().zip(data.time_values.iter()) {
            println!("            ({}, {})", n, time);
        }

        println!("        }};");

        // Create legend entry with escaped underscores
        let legend_name = data.name.replace('_', "\\_");
        println!("        \\addlegendentry{{{}}}", legend_name);
    }

    println!("    \\end{{axis}}");
    println!("\\end{{tikzpicture}}");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        eprintln!("Uso: {} <mmNvsT.csv> <mmtNvsT.csv> <mmsNvsT.csv>", args[0]);
        process::exit(1);
    }

    let mut data_sets = Vec::new();

    for file_path in &args[1..4] {
        match read_csv(file_path) {
            Ok(data) => data_sets.push(data),
            Err(err) => {
                eprintln!("Error procesando {}: {}", file_path, err);
                process::exit(1);
            }
        }
    }

    generate_latex_plot(data_sets);
}
