use std::env;
use std::fs;
use std::process;

#[derive(Debug)]
struct PerfData {
    n: usize,
    instructions: f64,
    #[allow(dead_code)]
    cycles: f64,
    ipc: f64,
    l1_loads: f64,
    l1_misses: f64,
    miss_rate: f64,
}

fn read_csv(file_path: &str) -> Result<Vec<PerfData>, String> {
    let contents = fs::read_to_string(file_path)
        .map_err(|_| format!("Could not open file: {}", file_path))?;

    let mut lines = contents.lines();

    // Skip header
    lines.next();

    let mut data = Vec::new();

    for line in lines {
        let parts: Vec<_> = line.split(',').collect();
        if parts.len() >= 7 {
            let n = parts[0].parse::<usize>()
                .map_err(|_| format!("Invalid N value: {}", parts[0]))?;
            let instructions = parts[1].parse::<f64>()
                .map_err(|_| format!("Invalid instructions value: {}", parts[1]))?;
            let cycles = parts[2].parse::<f64>()
                .map_err(|_| format!("Invalid cycles value: {}", parts[2]))?;
            let ipc = parts[3].parse::<f64>()
                .map_err(|_| format!("Invalid IPC value: {}", parts[3]))?;
            let l1_loads = parts[4].parse::<f64>()
                .map_err(|_| format!("Invalid L1 loads value: {}", parts[4]))?;
            let l1_misses = parts[5].parse::<f64>()
                .map_err(|_| format!("Invalid L1 misses value: {}", parts[5]))?;
            let miss_rate = parts[6].parse::<f64>()
                .map_err(|_| format!("Invalid miss rate value: {}", parts[6]))?;

            data.push(PerfData {
                n,
                instructions,
                cycles,
                ipc,
                l1_loads,
                l1_misses,
                miss_rate,
            });
        }
    }

    Ok(data)
}

fn generate_plot(
    title: &str,
    ylabel: &str,
    data_mm: &[(usize, f64)],
    data_mmt: &[(usize, f64)],
    data_mms: &[(usize, f64)],
    use_log_scale: bool,
) {
    let ylabel_escaped = ylabel.replace('_', "\\_");
    let title_escaped = title.replace('_', "\\_");

    println!("\\begin{{tikzpicture}}");
    println!("    \\begin{{axis}}[");
    println!("        title={{{}}},", title_escaped);
    println!("        xlabel={{Tamaño de matriz (N)}},");
    println!("        ylabel={{{}}},", ylabel_escaped);
    println!("        legend pos=north west,");
    println!("        grid=major,");
    println!("        width=12cm,");
    println!("        height=8cm,");

    if use_log_scale {
        println!("        ymode=log,");
    }

    println!("    ]");

    // Colores y marcadores para cada implementación
    let styles = [
        ("blue", "square*", "mm (Básico)"),
        ("red", "triangle*", "mmt (Transpuesto)"),
        ("green!50!black", "diamond*", "mms (Blocking)"),
    ];

    let datasets = [data_mm, data_mmt, data_mms];

    for (idx, data) in datasets.iter().enumerate() {
        let (color, marker, legend) = styles[idx];

        println!("        \\addplot[");
        println!("            color={},", color);
        println!("            mark={},", marker);
        println!("            thick,");
        println!("            mark size=3pt,");
        println!("        ]");
        println!("        coordinates {{");

        for (n, value) in *data {
            println!("            ({}, {})", n, value);
        }

        println!("        }};");
        println!("        \\addlegendentry{{{}}}", legend);
    }

    println!("    \\end{{axis}}");
    println!("\\end{{tikzpicture}}");
    println!();
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        eprintln!("Uso: {} <mm-exp3.csv> <mmt-exp3.csv> <mms-exp3.csv>", args[0]);
        process::exit(1);
    }

    // Leer los tres archivos CSV
    let mm_data = read_csv(&args[1]).unwrap_or_else(|err| {
        eprintln!("Error leyendo {}: {}", args[1], err);
        process::exit(1);
    });

    let mmt_data = read_csv(&args[2]).unwrap_or_else(|err| {
        eprintln!("Error leyendo {}: {}", args[2], err);
        process::exit(1);
    });

    let mms_data = read_csv(&args[3]).unwrap_or_else(|err| {
        eprintln!("Error leyendo {}: {}", args[3], err);
        process::exit(1);
    });

    // Generar encabezado LaTeX
    println!("% Gráficos generados automáticamente por parser_exp3.rs");
    println!("% Requiere: \\usepackage{{tikz}} y \\usepackage{{pgfplots}}");
    println!("% \\pgfplotsset{{compat=1.18}}");
    println!();

    // Gráfico 1: Instructions vs N
    let mm_instructions: Vec<(usize, f64)> = mm_data.iter()
        .map(|d| (d.n, d.instructions))
        .collect();
    let mmt_instructions: Vec<(usize, f64)> = mmt_data.iter()
        .map(|d| (d.n, d.instructions))
        .collect();
    let mms_instructions: Vec<(usize, f64)> = mms_data.iter()
        .map(|d| (d.n, d.instructions))
        .collect();

    generate_plot(
        "Instrucciones ejecutadas vs Tamaño de matriz",
        "Instrucciones",
        &mm_instructions,
        &mmt_instructions,
        &mms_instructions,
        false,
    );

    // Gráfico 2: IPC vs N
    let mm_ipc: Vec<(usize, f64)> = mm_data.iter()
        .map(|d| (d.n, d.ipc))
        .collect();
    let mmt_ipc: Vec<(usize, f64)> = mmt_data.iter()
        .map(|d| (d.n, d.ipc))
        .collect();
    let mms_ipc: Vec<(usize, f64)> = mms_data.iter()
        .map(|d| (d.n, d.ipc))
        .collect();

    generate_plot(
        "IPC (Instructions Per Cycle) vs Tamaño de matriz",
        "IPC",
        &mm_ipc,
        &mmt_ipc,
        &mms_ipc,
        false,
    );

    // Gráfico 3: L1-dcache loads vs N
    let mm_l1_loads: Vec<(usize, f64)> = mm_data.iter()
        .map(|d| (d.n, d.l1_loads))
        .collect();
    let mmt_l1_loads: Vec<(usize, f64)> = mmt_data.iter()
        .map(|d| (d.n, d.l1_loads))
        .collect();
    let mms_l1_loads: Vec<(usize, f64)> = mms_data.iter()
        .map(|d| (d.n, d.l1_loads))
        .collect();

    generate_plot(
        "L1 Data Cache Loads vs Tamaño de matriz",
        "L1 Loads",
        &mm_l1_loads,
        &mmt_l1_loads,
        &mms_l1_loads,
        false,
    );

    // Gráfico 4: L1-dcache misses vs N
    let mm_l1_misses: Vec<(usize, f64)> = mm_data.iter()
        .map(|d| (d.n, d.l1_misses))
        .collect();
    let mmt_l1_misses: Vec<(usize, f64)> = mmt_data.iter()
        .map(|d| (d.n, d.l1_misses))
        .collect();
    let mms_l1_misses: Vec<(usize, f64)> = mms_data.iter()
        .map(|d| (d.n, d.l1_misses))
        .collect();

    generate_plot(
        "L1 Data Cache Misses vs Tamaño de matriz",
        "L1 Misses",
        &mm_l1_misses,
        &mmt_l1_misses,
        &mms_l1_misses,
        true, // Usar escala logarítmica para ver mejor las diferencias
    );

    // Gráfico 5: L1 miss rate vs N
    let mm_miss_rate: Vec<(usize, f64)> = mm_data.iter()
        .map(|d| (d.n, d.miss_rate))
        .collect();
    let mmt_miss_rate: Vec<(usize, f64)> = mmt_data.iter()
        .map(|d| (d.n, d.miss_rate))
        .collect();
    let mms_miss_rate: Vec<(usize, f64)> = mms_data.iter()
        .map(|d| (d.n, d.miss_rate))
        .collect();

    generate_plot(
        "L1 Miss Rate vs Tamaño de matriz",
        "Miss Rate (\\%)",
        &mm_miss_rate,
        &mmt_miss_rate,
        &mms_miss_rate,
        false,
    );
}
