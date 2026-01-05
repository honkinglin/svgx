use clap::Parser;
use std::fs;
use std::path::PathBuf;
use svgx::parser;
use svgx::plugins::{
    CleanupAttrs, CleanupIds, CleanupListOfValues, CleanupNumericValues, CollapseGroups,
    ConvertColors, ConvertEllipseToCircle, ConvertOneStopGradients, ConvertPathData,
    ConvertShapeToPath, ConvertStyleToAttrs, ConvertTransform, MergePaths, MoveElemsAttrsToGroup,
    MoveGroupAttrsToElems, Plugin, RemoveComments, RemoveDesc, RemoveDimensions, RemoveDoctype,
    RemoveEditorsNSData, RemoveEmptyAttrs, RemoveEmptyContainers, RemoveEmptyText,
    RemoveHiddenElems, RemoveMetadata, RemoveRasterImages, RemoveScriptElement, RemoveStyleElement,
    RemoveTitle, RemoveUnknownsAndDefaults, RemoveUnusedNS, RemoveUselessDefs,
    RemoveUselessStrokeAndFill, RemoveXMLProcInst, SortAttrs, SortDefsChildren,
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
            // Ordered Pipeline:
            let plugins: Vec<Box<dyn Plugin>> = vec![
                Box::new(RemoveDoctype),
                Box::new(RemoveXMLProcInst),
                Box::new(RemoveComments),
                Box::new(RemoveMetadata),
                Box::new(RemoveTitle),
                Box::new(RemoveDesc),
                Box::new(RemoveEditorsNSData),
                Box::new(RemoveScriptElement),
                Box::new(RemoveRasterImages),
                Box::new(RemoveStyleElement),
                Box::new(ConvertStyleToAttrs),
                Box::new(CleanupAttrs),
                Box::new(RemoveDimensions),
                Box::new(MoveGroupAttrsToElems),
                Box::new(MoveElemsAttrsToGroup),
                Box::new(ConvertOneStopGradients),
                Box::new(CleanupIds),
                Box::new(RemoveUselessDefs),
                Box::new(RemoveEmptyContainers), // Remove Empty Containers (including defs)
                Box::new(RemoveHiddenElems),
                Box::new(RemoveEmptyText),
                Box::new(CollapseGroups),
                Box::new(RemoveUselessStrokeAndFill),
                Box::new(ConvertEllipseToCircle),
                Box::new(ConvertShapeToPath),
                Box::new(ConvertPathData::default()),
                Box::new(ConvertTransform::default()),
                Box::new(MergePaths),
                Box::new(ConvertColors),
                Box::new(CleanupNumericValues::default()),
                Box::new(CleanupListOfValues::default()),
                Box::new(RemoveUnknownsAndDefaults::default()),
                Box::new(RemoveEmptyAttrs),
                Box::new(RemoveUnusedNS), // Late? Check usage
                Box::new(SortAttrs),
                Box::new(SortDefsChildren),
            ];

            for plugin in plugins {
                // println!("Running plugin: {}", std::any::type_name_of_val(&*plugin));
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
