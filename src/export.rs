use anyhow::{Result, Context};
use printpdf::*;
use std::fs::File;
use std::io::BufWriter;

pub fn export_to_pdf(path: &str, keywords: &[String], summary: &[String], links: &[String]) -> Result<()> {
    // Page and layout constants
    let page_width = Mm(210.0);
    let page_height = Mm(297.0);
    let margin_left = Mm(20.0);
    let margin_right = Mm(20.0);
    let margin_top = Mm(17.0);
    let margin_bottom = Mm(17.0);
    let usable_width = page_width.0 - margin_left.0 - margin_right.0;
    let line_height = Mm(9.0);

    // Create PDF document and first page/layer
    let (doc, page1, layer1) = PdfDocument::new("Lecture Summary", page_width, page_height, "Layer 1");
    let mut page = page1;
    let mut layer = layer1;

    // Load built-in fonts
    let font = doc.add_builtin_font(BuiltinFont::Helvetica).context("Failed to load font")?;
    let font_bold = doc.add_builtin_font(BuiltinFont::HelveticaBold).context("Failed to load bold font")?;

    // Helper: wrap text to fit width (approximate, since printpdf doesn't measure text)
    fn wrap_text(text: &str, max_chars: usize) -> Vec<String> {
        let mut lines = Vec::new();
        let mut t = text.trim();
        while t.len() > max_chars {
            let (line, rest) = t.split_at(max_chars);
            if let Some(idx) = line.rfind(' ') {
                lines.push(line[..idx].to_string());
                t = &t[idx+1..];
            } else {
                lines.push(line.to_string());
                t = rest;
            }
            t = t.trim_start();
        }
        if !t.is_empty() {
            lines.push(t.to_string());
        }
        lines
    }

    // Helper: write a line, wrap, and paginate if needed
    let mut y = page_height.0 - margin_top.0;
    let mut write_wrapped = |text: &str, size: f64, bold: bool, bullet: bool, y: &mut f64| {
        let max_chars = (usable_width / (size * 0.45)) as usize;
        let lines = wrap_text(text, max_chars);
        for (i, line) in lines.iter().enumerate() {
            if *y < margin_bottom.0 + line_height.0 {
                // Add new page if needed
                let (new_page, new_layer) = doc.add_page(page_width, page_height, "Layer");
                page = new_page;
                layer = new_layer;
                *y = page_height.0 - margin_top.0;
            }
            let draw_text = if bullet && i == 0 { format!("• {}", line) } else { line.clone() };
            let font_ref = if bold { &font_bold } else { &font };
            let current_layer = doc.get_page(page).get_layer(layer);
            current_layer.use_text(
                &draw_text,
                size as f64,
                Mm(margin_left.0),
                Mm(*y),
                font_ref,
            );
            *y -= line_height.0;
        }
    };

    // Title
    write_wrapped("Lecture Summary", 20.0, true, false, &mut y);
    y -= 4.0;
    // Keywords
    write_wrapped("Keywords:", 14.0, true, false, &mut y);
    for k in keywords {
        write_wrapped(k, 12.0, false, true, &mut y);
    }
    y -= 2.0;
    // Summary
    write_wrapped("Summary:", 14.0, true, false, &mut y);
    for s in summary {
        write_wrapped(s, 12.0, false, true, &mut y);
    }
    y -= 2.0;
    // Links
    write_wrapped("Links:", 14.0, true, false, &mut y);
    for l in links {
        write_wrapped(l, 12.0, false, true, &mut y);
    }

    // Save the PDF
    let file = File::create(path).context("Failed to create PDF file")?;
    let mut writer = BufWriter::new(file);
    doc.save(&mut writer).context("Failed to write PDF")?;
    Ok(())
}