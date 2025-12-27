use clap::Parser;
use std::fs;
use std::path::PathBuf;
use svgx::parser;
use svgx::plugins::{
    CleanupAttrs, CleanupIds, ConvertColors, Plugin, RemoveComments, RemoveDoctype,
    RemoveEditorsNSData, RemoveEmptyText, RemoveHiddenElems, RemoveMetadata, RemoveUselessDefs,
    RemoveXMLProcInst,
};
use svgx::printer;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input SVG file
    input: PathBuf,

    /// Output SVG file (optional)
    #[arg(short, long)]
    output: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();

    let text = fs::read_to_string(&args.input).expect("Could not read input file");

    match parser::parse(&text) {
        Ok(mut doc) => {
            // Apply plugins
            let plugins: Vec<Box<dyn Plugin>> = vec![
                Box::new(RemoveDoctype),
                Box::new(RemoveXMLProcInst),
                Box::new(RemoveComments),
                Box::new(RemoveMetadata),
                Box::new(RemoveEditorsNSData),
                Box::new(CleanupAttrs),
                // Cleanup IDs and Defs (IDs first to mark used? No.
                // If we run CleanupIds first, it removes IDs that are unused.
                // If we run RemoveUselessDefs first, it removes defs children that are unused.
                // Ideally:
                // 1. RemoveUselessDefs: removes elements in defs that are not referenced.
                // 2. CleanupIds: removes IDs on other elements that are not referenced.
                // Actually if an element in defs is removed, its ID is gone.
                // If we run CleanupIds, we strip ID from element.
                // Order: RemoveUselessDefs -> CleanupIds might be better?
                // Or maybe CleanupIds should treat defs children specially?
                // svgo: cleanupIds happens, then removeUselessDefs.
                // Wait, if cleanupIds removes the ID from a rect in defs (because unused),
                // then removeUselessDefs sees a rect without ID in defs -> removes it.
                // So CleanupIds effectively enables RemoveUselessDefs to kill it.
                Box::new(CleanupIds),
                Box::new(RemoveUselessDefs),
                Box::new(RemoveHiddenElems),
                Box::new(RemoveEmptyText),
                Box::new(ConvertColors),
            ];

            for plugin in plugins {
                plugin.apply(&mut doc);
            }

            let out = printer::print(&doc);
            if let Some(output_path) = args.output {
                fs::write(output_path, out).expect("Could not write output file");
            } else {
                println!("{}", out);
            }
        }
        Err(e) => {
            eprintln!("Error parsing SVG: {}", e);
            std::process::exit(1);
        }
    }
}
