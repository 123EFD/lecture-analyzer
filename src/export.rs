use anyhow::{Context, Result};
use printpdf::*;
use std::fs::File;
use std::io::BufWriter;
use crate::export::indices::PdfPageIndex;
use crate::export::indices::PdfLayerIndex;

#[allow(dead_code)]
/// Handles page and line placement, and paginates as needed.
struct PdfPaginator<'a> {
    doc: &'a mut PdfDocumentReference,
    page_width: Mm,
    page_height: Mm,
    margin_left: Mm,
    margin_right: Mm,
    margin_top: Mm,
    margin_bottom: Mm,
    line_height: f64,
    usable_width: f64,
    y: f64,
    cur_page_idx: PdfPageIndex,
    cur_layer_idx: PdfLayerIndex,
}

impl<'a> PdfPaginator<'a> {
    fn new(
        doc: &'a mut PdfDocumentReference,
        page_width: Mm,
        page_height: Mm,
        margin_left: Mm,
        margin_right: Mm,
        margin_top: Mm,
        margin_bottom: Mm,
        line_height: f64,
    ) -> Self {
        let (page_idx, layer_idx) = doc.add_page(page_width, page_height, "Layer 1");
        let usable_width = page_width.0 - margin_left.0 - margin_right.0;
        Self {
            doc,
            page_width,
            page_height,
            margin_left,
            margin_right,
            margin_top,
            margin_bottom,
            line_height,
            usable_width,
            y: page_height.0 - margin_top.0,
            cur_page_idx: page_idx,
            cur_layer_idx: layer_idx,
        }
    }

    fn ensure_space(&mut self) {
        if self.y < self.margin_bottom.0 + self.line_height {
            let (new_page_idx, new_layer_idx) =
                self.doc
                    .add_page(self.page_width, self.page_height, "Layer");
            self.cur_page_idx = new_page_idx;
            self.cur_layer_idx = new_layer_idx;
            self.y = self.page_height.0 - self.margin_top.0;
        }
    }

    fn wrap_text(&self, text: &str, font_size: f64) -> Vec<String> {
        let max_chars = (self.usable_width / (font_size * 0.45)) as usize;
        let mut lines = Vec::new();
        let mut t = text.trim();
        while t.len() > max_chars {
            let (line, rest) = t.split_at(max_chars);
            if let Some(idx) = line.rfind(' ') {
                lines.push(line[..idx].to_string());
                t = &t[idx + 1..];
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

    fn write_wrapped(
        &mut self,
        text: &str,
        font: &IndirectFontRef,
        font_bold: &IndirectFontRef,
        size: f64,
        bold: bool,
        bullet: bool,
    ) {
        let lines = self.wrap_text(text, size);
        for (i, line) in lines.iter().enumerate() {
            self.ensure_space();
            let draw_text = if bullet && i == 0 {
                format!("• {}", line)
            } else {
                line.clone()
            };
            let font_ref = if bold { font_bold } else { font };
            let layer = self
                .doc
                .get_page(self.cur_page_idx)
                .get_layer(self.cur_layer_idx);
            layer.use_text(
                &draw_text,
                size,                                  // font size in Pt
                self.margin_left,                      // use Mm directly
                Mm(self.y),                            // use Mm directly
                font_ref,                              // pass as reference
            );
            self.y -= self.line_height;
        }
    }
}

/// Export lecture summary to a PDF file.
pub fn export_to_pdf(
    path: &str,
    keywords: &[String],
    summary: &[String],
    links: &[String],
) -> Result<()> {
    // Page and layout constants
    let page_width = Mm(210.0);
    let page_height = Mm(297.0);
    let margin_left = Mm(20.0);
    let margin_right = Mm(20.0);
    let margin_top = Mm(17.0);
    let margin_bottom = Mm(17.0);
    let line_height = 9.0;

    // Create PDF document
    let mut doc = PdfDocument::empty("Lecture Summary");

    // Load built-in fonts
    let font = doc
        .add_builtin_font(BuiltinFont::Helvetica)
        .context("Failed to load font")?;
    let font_bold = doc
        .add_builtin_font(BuiltinFont::HelveticaBold)
        .context("Failed to load bold font")?;

    // Initialize paginator
    let mut paginator = PdfPaginator::new(
        &mut doc,
        page_width,
        page_height,
        margin_left,
        margin_right,
        margin_top,
        margin_bottom,
        line_height,
    );

    // Write content with pagination
    paginator.write_wrapped(
        "Lecture Summary",
        &font,
        &font_bold,
        20.0,
        true,
        false,
    );
    paginator.y -= 4.0; // extra space

    paginator.write_wrapped("Keywords:", &font, &font_bold, 14.0, true, false);
    for k in keywords {
        paginator.write_wrapped(k, &font, &font_bold, 12.0, false, true);
    }
    paginator.y -= 2.0;

    paginator.write_wrapped("Summary:", &font, &font_bold, 14.0, true, false);
    for s in summary {
        paginator.write_wrapped(s, &font, &font_bold, 12.0, false, true);
    }
    paginator.y -= 2.0;

    paginator.write_wrapped("Links:", &font, &font_bold, 14.0, true, false);
    for l in links {
        paginator.write_wrapped(l, &font, &font_bold, 12.0, false, true);
    }

    // Save the PDF
    let file = File::create(path)
        .with_context(|| format!("Failed to create PDF file: {path}"))?;
    let mut writer = BufWriter::new(file);
    doc.save(&mut writer)
        .context("Failed to write PDF to output file")?;
    Ok(())
}