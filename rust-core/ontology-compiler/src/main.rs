mod args;
mod compiler;

use clap::Parser;
use anyhow::{Result, Context};
use std::fs;
use ontology_engine::OntologyDef;

fn main() -> Result<()> {
    let args = args::Args::parse();

    println!("Starting Ontology Compiler...");
    println!("Input Directory: {:?}", args.input);
    println!("Output File: {:?}", args.output);

    // 1. Compile OWL definitions
    let compiler = compiler::Compiler::new();
    compiler.load_ttl_files(&args.input)?;
    let mut ontology = compiler.compile()?;

    println!("Compiled {} Object Types", ontology.object_types.len());
    println!("Compiled {} Link Types", ontology.link_types.len());
    println!("Compiled {} Interfaces", ontology.interfaces.len());

    // 2. Merge Sidecar (Actions/Functions)
    if let Some(sidecar_path) = args.sidecar {
        println!("Loading sidecar: {:?}", sidecar_path);
        let sidecar_content = fs::read_to_string(&sidecar_path)
            .context("Failed to read sidecar file")?;

        // We parse the sidecar as a partial OntologyDef (or a specific struct matching the yaml)
        // Since serde_yaml ignores missing fields, we can try parsing into OntologyDef directly
        // assuming the yaml structure matches (which it does based on extraction).
        #[derive(serde::Deserialize)]
        struct Sidecar {
            #[serde(default)]
            action_types: Vec<ontology_engine::ActionTypeDef>,
            #[serde(default)]
            function_types: Vec<ontology_engine::FunctionTypeDef>,
        }

        let sidecar: Sidecar = serde_yaml::from_str(&sidecar_content)
            .context("Failed to parse sidecar YAML")?;

        ontology.action_types = sidecar.action_types;
        ontology.function_types = sidecar.function_types;

        println!("Merged {} Action Types", ontology.action_types.len());
        println!("Merged {} Function Types", ontology.function_types.len());
    }

    // 3. Serialize to JSON
    let config = ontology_engine::OntologyConfig { ontology };
    let json = serde_json::to_string_pretty(&config)
        .context("Failed to serialize ontology to JSON")?;

    fs::write(&args.output, json)
        .context("Failed to write output file")?;

    println!("Success! Ontology compiled to {:?}", args.output);

    Ok(())
}
