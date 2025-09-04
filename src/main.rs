use clap::{Parser, Subcommand};
use anyhow::Result;

mod pdf;
mod analyze;
mod export;
mod util;

/// Lecture Note Analyzer
#[derive(Parser)]
#[command(name = "lecture-analyzer")]
#[command(version = "0.1.0")]
#[command(about = "Summarize lecture notes and export key insights", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Analyze one or more lecture PDFs
    Analyze {
        /// Input PDF files
        #[arg(required = true)]
        files: Vec<String>,

        /// Export summary to PDF file
        #[arg(long)]
        export: Option<String>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Analyze { files, export } => {
            // Step 1: Extract text
            let mut all_text = String::new();
            for file in files {
                let text = pdf::extract_text(&file)?;
                all_text.push_str(&text);
                all_text.push('\n');
            }

            // Step 2: Analyze
            let (keywords, summary) = analyze::summarize(&all_text);

            // Step 3: Suggested resources
            let links = util::suggest_links(&keywords);

            // Step 4: Export or print
            if let Some(path) = export {
                export::export_to_pdf(&path, &keywords, &summary, &links)?;
                println!("Summary exported to {}", path);
            } else {
                println!("📌 Keywords: {:?}", keywords);
                println!("📝 Summary:\n{}", summary.join("\n"));
                println!("🔗 Links:\n{}", links.join("\n"));
            }
        }
    }

    Ok(())
}
