use ab_glyph::{Font, FontRef, PxScale, ScaleFont};
use image::{
    ExtendedColorType, ImageBuffer, ImageEncoder, Rgba, RgbaImage, codecs::png::PngEncoder,
};

use crate::error::Error;

const REGULAR: &[u8] = include_bytes!("../assets/DejaVuSans.ttf");
const BOLD: &[u8] = include_bytes!("../assets/DejaVuSans-Bold.ttf");
const OBLIQUE: &[u8] = include_bytes!("../assets/DejaVuSans-Oblique.ttf");

const TARGET_WIDTH: u32 = 1200;
const FONT_SIZE_PX: f32 = 22.0;
const CELL_PADDING_X: u32 = 14;
const CELL_PADDING_Y: u32 = 10;
const BORDER_COLOR: Rgba<u8> = Rgba([210, 210, 210, 255]);
const HEADER_BG: Rgba<u8> = Rgba([244, 244, 244, 255]);
const CELL_BG: Rgba<u8> = Rgba([255, 255, 255, 255]);
const TEXT_COLOR: Rgba<u8> = Rgba([25, 25, 25, 255]);

/// Render a GFM-style pipe table (header + body rows) to a PNG byte buffer.
pub struct TableImage<'a> {
    header: &'a [String],
    rows: &'a [Vec<String>],
}

impl<'a> TableImage<'a> {
    pub fn new(header: &'a [String], rows: &'a [Vec<String>]) -> Self {
        Self { header, rows }
    }

    pub fn render_png(&self) -> Result<Vec<u8>, Error> {
        let fonts = FontSet::load()?;
        let columns = self.header.len();

        let header_runs: Vec<Vec<InlineRun>> = self
            .header
            .iter()
            .map(|c| InlineRun::parse(c, true))
            .collect();
        let body_runs: Vec<Vec<Vec<InlineRun>>> = self
            .rows
            .iter()
            .map(|row| {
                pad_row(row, columns)
                    .iter()
                    .map(|c| InlineRun::parse(c, false))
                    .collect()
            })
            .collect();

        let column_widths = ColumnLayout::compute(&header_runs, &body_runs, columns, &fonts);

        let header_lines: Vec<Vec<Vec<InlineRun>>> = header_runs
            .iter()
            .zip(column_widths.iter())
            .map(|(runs, &w)| fonts.wrap(runs, w))
            .collect();
        let body_lines: Vec<Vec<Vec<Vec<InlineRun>>>> = body_runs
            .iter()
            .map(|row| {
                row.iter()
                    .zip(column_widths.iter())
                    .map(|(runs, &w)| fonts.wrap(runs, w))
                    .collect()
            })
            .collect();

        let line_height = fonts.line_height();
        let header_height = row_height(&header_lines, line_height);
        let body_heights: Vec<u32> = body_lines
            .iter()
            .map(|r| row_height(r, line_height))
            .collect();

        let total_height =
            1 + header_height + body_heights.iter().sum::<u32>() + body_heights.len() as u32 + 1;
        let total_width = column_widths.iter().sum::<u32>() + columns as u32 + 1;

        let mut canvas = Canvas::new(total_width, total_height);
        let mut y: u32 = 0;
        canvas.draw_horizontal_line(y);
        y += 1;

        canvas.fill_row(y, header_height, &column_widths, HEADER_BG);
        canvas.draw_row_cells(y, &column_widths, &header_lines, line_height, &fonts);
        y += header_height;
        canvas.draw_horizontal_line(y);
        y += 1;

        for (row_lines, &row_height_px) in body_lines.iter().zip(body_heights.iter()) {
            canvas.draw_row_cells(y, &column_widths, row_lines, line_height, &fonts);
            y += row_height_px;
            canvas.draw_horizontal_line(y);
            y += 1;
        }

        canvas.draw_vertical_lines(&column_widths);
        canvas.into_png()
    }
}

// ── Inline run parsing ──────────────────────────────────────────────

#[derive(Debug, Clone)]
struct InlineRun {
    text: String,
    bold: bool,
    italic: bool,
}

impl InlineRun {
    /// Parse a cell string into inline runs, splitting on `*...*` and `**...**`.
    fn parse(text: &str, header: bool) -> Vec<Self> {
        let mut runs = Vec::new();
        let mut buffer = String::new();
        let bytes = text.as_bytes();
        let mut i = 0;
        while i < bytes.len() {
            if bytes[i] == b'*' {
                let start = i;
                while i < bytes.len() && bytes[i] == b'*' {
                    i += 1;
                }
                let stars = i - start;
                let pattern: String = std::iter::repeat('*').take(stars).collect();
                if let Some(end) = text[i..].find(&pattern) {
                    if !buffer.is_empty() {
                        runs.push(Self {
                            text: std::mem::take(&mut buffer),
                            bold: header,
                            italic: false,
                        });
                    }
                    let inner = &text[i..i + end];
                    runs.push(Self {
                        text: inner.to_string(),
                        bold: header || stars >= 2,
                        italic: stars == 1 || stars == 3,
                    });
                    i += end + stars;
                } else {
                    buffer.push_str(&pattern);
                }
            } else {
                let ch = text[i..].chars().next().unwrap();
                buffer.push(ch);
                i += ch.len_utf8();
            }
        }
        if !buffer.is_empty() {
            runs.push(Self {
                text: buffer,
                bold: header,
                italic: false,
            });
        }
        if runs.is_empty() {
            runs.push(Self {
                text: String::new(),
                bold: header,
                italic: false,
            });
        }
        runs
    }
}

fn pad_row(row: &[String], columns: usize) -> Vec<String> {
    let mut padded: Vec<String> = row.iter().cloned().collect();
    while padded.len() < columns {
        padded.push(String::new());
    }
    padded.truncate(columns);
    padded
}

// ── Font bundle ─────────────────────────────────────────────────────

struct FontSet<'a> {
    regular: FontRef<'a>,
    bold: FontRef<'a>,
    oblique: FontRef<'a>,
    scale: PxScale,
}

impl<'a> FontSet<'a> {
    fn load() -> Result<Self, Error> {
        Ok(Self {
            regular: FontRef::try_from_slice(REGULAR)
                .map_err(|_| Error::InvalidImage("failed to load regular font".into()))?,
            bold: FontRef::try_from_slice(BOLD)
                .map_err(|_| Error::InvalidImage("failed to load bold font".into()))?,
            oblique: FontRef::try_from_slice(OBLIQUE)
                .map_err(|_| Error::InvalidImage("failed to load oblique font".into()))?,
            scale: PxScale::from(FONT_SIZE_PX),
        })
    }

    fn select(&self, run: &InlineRun) -> &FontRef<'a> {
        if run.bold {
            &self.bold
        } else if run.italic {
            &self.oblique
        } else {
            &self.regular
        }
    }

    fn measure(&self, run: &InlineRun) -> f32 {
        let font = self.select(run);
        let scaled = font.as_scaled(self.scale);
        measure_text(&scaled, &run.text)
    }

    fn longest_word_width(&self, runs: &[InlineRun]) -> f32 {
        let mut longest = 0.0_f32;
        for run in runs {
            let font = self.select(run);
            let scaled = font.as_scaled(self.scale);
            for word in run.text.split_whitespace() {
                let w = measure_text(&scaled, word);
                if w > longest {
                    longest = w;
                }
            }
        }
        longest
    }

    fn runs_width(&self, runs: &[InlineRun]) -> f32 {
        runs.iter().map(|run| self.measure(run)).sum()
    }

    fn line_height(&self) -> u32 {
        let scaled = self.regular.as_scaled(self.scale);
        (scaled.height() + scaled.line_gap() * 0.2).ceil() as u32
    }

    fn wrap(&self, runs: &[InlineRun], cell_width: u32) -> Vec<Vec<InlineRun>> {
        let available = cell_width.saturating_sub(2 * CELL_PADDING_X) as f32;
        let mut lines: Vec<Vec<InlineRun>> = Vec::new();
        let mut current: Vec<InlineRun> = Vec::new();
        let mut current_width = 0.0_f32;

        for run in runs {
            let font = self.select(run);
            let scaled = font.as_scaled(self.scale);
            let mut pending = String::new();

            let flush_pending =
                |pending: &mut String, current: &mut Vec<InlineRun>, run: &InlineRun| {
                    if !pending.is_empty() {
                        current.push(InlineRun {
                            text: std::mem::take(pending),
                            bold: run.bold,
                            italic: run.italic,
                        });
                    }
                };

            for token in tokenize_with_spaces(&run.text) {
                let token_width = measure_text(&scaled, &token);
                let fits = current_width + token_width <= available;
                let is_space = token.chars().all(|c| c.is_whitespace());

                if fits {
                    pending.push_str(&token);
                    current_width += token_width;
                } else if is_space {
                    flush_pending(&mut pending, &mut current, run);
                    if !current.is_empty() {
                        lines.push(std::mem::take(&mut current));
                    }
                    current_width = 0.0;
                } else {
                    flush_pending(&mut pending, &mut current, run);
                    if !current.is_empty() {
                        lines.push(std::mem::take(&mut current));
                    }
                    pending.push_str(&token);
                    current_width = token_width;
                }
            }
            flush_pending(&mut pending, &mut current, run);
        }

        if !current.is_empty() {
            lines.push(current);
        }
        if lines.is_empty() {
            lines.push(Vec::new());
        }
        lines
    }
}

fn measure_text<F: Font>(scaled: &impl ScaleFont<F>, text: &str) -> f32 {
    let mut width = 0.0_f32;
    let mut previous: Option<ab_glyph::GlyphId> = None;
    for ch in text.chars() {
        let glyph_id = scaled.glyph_id(ch);
        if let Some(prev) = previous {
            width += scaled.kern(prev, glyph_id);
        }
        width += scaled.h_advance(glyph_id);
        previous = Some(glyph_id);
    }
    width
}

fn tokenize_with_spaces(text: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut buffer = String::new();
    let mut in_space = false;
    for ch in text.chars() {
        let is_space = ch.is_whitespace();
        if buffer.is_empty() {
            in_space = is_space;
            buffer.push(ch);
        } else if is_space == in_space {
            buffer.push(ch);
        } else {
            tokens.push(std::mem::take(&mut buffer));
            buffer.push(ch);
            in_space = is_space;
        }
    }
    if !buffer.is_empty() {
        tokens.push(buffer);
    }
    tokens
}

// ── Column layout ───────────────────────────────────────────────────

struct ColumnLayout;

impl ColumnLayout {
    fn compute(
        header_runs: &[Vec<InlineRun>],
        body_runs: &[Vec<Vec<InlineRun>>],
        columns: usize,
        fonts: &FontSet<'_>,
    ) -> Vec<u32> {
        let min_widths: Vec<u32> = (0..columns)
            .map(|i| {
                let mut max_word = 0.0_f32;
                if let Some(runs) = header_runs.get(i) {
                    max_word = max_word.max(fonts.longest_word_width(runs));
                }
                for row in body_runs {
                    if let Some(runs) = row.get(i) {
                        max_word = max_word.max(fonts.longest_word_width(runs));
                    }
                }
                max_word.ceil() as u32 + 2 * CELL_PADDING_X
            })
            .collect();

        let natural_widths: Vec<u32> = (0..columns)
            .map(|i| {
                let mut max_w = 0.0_f32;
                if let Some(runs) = header_runs.get(i) {
                    max_w = max_w.max(fonts.runs_width(runs));
                }
                for row in body_runs {
                    if let Some(runs) = row.get(i) {
                        max_w = max_w.max(fonts.runs_width(runs));
                    }
                }
                max_w.ceil() as u32 + 2 * CELL_PADDING_X
            })
            .collect();

        let available = TARGET_WIDTH.saturating_sub(columns as u32 + 1);
        let natural_sum: u32 = natural_widths.iter().sum();
        if natural_sum <= available {
            return natural_widths;
        }

        let min_sum: u32 = min_widths.iter().sum();
        if min_sum >= available {
            return min_widths;
        }

        let slack = available - min_sum;
        let extra_desired = natural_sum - min_sum;
        let mut widths: Vec<u32> = min_widths
            .iter()
            .zip(natural_widths.iter())
            .map(|(&min, &natural)| {
                let want = natural - min;
                min + (want as u64 * slack as u64 / extra_desired as u64) as u32
            })
            .collect();

        let drift = available as i64 - widths.iter().sum::<u32>() as i64;
        if drift > 0 {
            if let Some(last) = widths.last_mut() {
                *last += drift as u32;
            }
        }
        widths
    }
}

fn row_height<T>(lines_per_cell: &[Vec<T>], line_height: u32) -> u32 {
    let max_lines = lines_per_cell
        .iter()
        .map(|l| l.len().max(1))
        .max()
        .unwrap_or(1);
    line_height * max_lines as u32 + 2 * CELL_PADDING_Y
}

// ── Canvas / drawing ────────────────────────────────────────────────

struct Canvas {
    image: RgbaImage,
}

impl Canvas {
    fn new(width: u32, height: u32) -> Self {
        Self {
            image: ImageBuffer::from_pixel(width, height, CELL_BG),
        }
    }

    fn width(&self) -> u32 {
        self.image.width()
    }

    fn height(&self) -> u32 {
        self.image.height()
    }

    fn blend_pixel(&mut self, x: u32, y: u32, color: Rgba<u8>, coverage: f32) {
        if coverage <= 0.0 {
            return;
        }
        let coverage = coverage.clamp(0.0, 1.0);
        let bg = *self.image.get_pixel(x, y);
        let blend = |fg: u8, bg: u8| -> u8 {
            (fg as f32 * coverage + bg as f32 * (1.0 - coverage))
                .round()
                .clamp(0.0, 255.0) as u8
        };
        let r = blend(color.0[0], bg.0[0]);
        let g = blend(color.0[1], bg.0[1]);
        let b = blend(color.0[2], bg.0[2]);
        self.image.put_pixel(x, y, Rgba([r, g, b, 255]));
    }

    fn fill_row(&mut self, y: u32, height: u32, column_widths: &[u32], color: Rgba<u8>) {
        let total_w: u32 = column_widths.iter().sum::<u32>() + column_widths.len() as u32 + 1;
        for yy in y..(y + height) {
            for xx in 0..total_w.min(self.width()) {
                self.image.put_pixel(xx, yy, color);
            }
        }
    }

    fn draw_horizontal_line(&mut self, y: u32) {
        if y >= self.height() {
            return;
        }
        let w = self.width();
        for x in 0..w {
            self.image.put_pixel(x, y, BORDER_COLOR);
        }
    }

    fn draw_vertical_line(&mut self, x: u32) {
        if x >= self.width() {
            return;
        }
        let h = self.height();
        for y in 0..h {
            self.image.put_pixel(x, y, BORDER_COLOR);
        }
    }

    fn draw_vertical_lines(&mut self, column_widths: &[u32]) {
        let mut x: u32 = 0;
        for &cw in column_widths {
            self.draw_vertical_line(x);
            x += cw + 1;
        }
        self.draw_vertical_line(x);
    }

    fn draw_row_cells(
        &mut self,
        y: u32,
        column_widths: &[u32],
        lines_per_cell: &[Vec<Vec<InlineRun>>],
        line_height: u32,
        fonts: &FontSet<'_>,
    ) {
        let mut x: u32 = 1;
        for (i, &cw) in column_widths.iter().enumerate() {
            let empty: Vec<Vec<InlineRun>> = Vec::new();
            let cell_lines = lines_per_cell.get(i).unwrap_or(&empty);
            self.draw_cell_text(x, y, cell_lines, line_height, fonts);
            x += cw + 1;
        }
    }

    fn draw_cell_text(
        &mut self,
        cell_x: u32,
        cell_y: u32,
        lines: &[Vec<InlineRun>],
        line_height: u32,
        fonts: &FontSet<'_>,
    ) {
        let regular_scaled = fonts.regular.as_scaled(fonts.scale);
        let ascent = regular_scaled.ascent();

        let mut y_cursor = cell_y + CELL_PADDING_Y;
        for line in lines {
            let baseline = y_cursor as f32 + ascent;
            let mut x_cursor = (cell_x + CELL_PADDING_X) as f32;
            let mut previous_glyph: Option<PreviousGlyph> = None;
            for run in line {
                let font = fonts.select(run);
                let scaled = font.as_scaled(fonts.scale);
                for ch in run.text.chars() {
                    let glyph_id = scaled.glyph_id(ch);
                    if let Some(prev) = previous_glyph {
                        if prev.bold == run.bold && prev.italic == run.italic {
                            x_cursor += scaled.kern(prev.id, glyph_id);
                        }
                    }
                    let glyph = glyph_id
                        .with_scale_and_position(fonts.scale, ab_glyph::point(x_cursor, baseline));
                    if let Some(outlined) = font.outline_glyph(glyph) {
                        let bounds = outlined.px_bounds();
                        let width = self.width();
                        let height = self.height();
                        outlined.draw(|gx, gy, coverage| {
                            let px = bounds.min.x as i32 + gx as i32;
                            let py = bounds.min.y as i32 + gy as i32;
                            if px >= 0 && py >= 0 && (px as u32) < width && (py as u32) < height {
                                self.blend_pixel(px as u32, py as u32, TEXT_COLOR, coverage);
                            }
                        });
                    }
                    x_cursor += scaled.h_advance(glyph_id);
                    previous_glyph = Some(PreviousGlyph {
                        id: glyph_id,
                        bold: run.bold,
                        italic: run.italic,
                    });
                }
            }
            y_cursor += line_height;
        }
    }

    fn into_png(self) -> Result<Vec<u8>, Error> {
        let mut buffer = Vec::new();
        let encoder = PngEncoder::new(&mut buffer);
        encoder
            .write_image(
                self.image.as_raw(),
                self.image.width(),
                self.image.height(),
                ExtendedColorType::Rgba8,
            )
            .map_err(|e| Error::InvalidImage(format!("png encoding failed: {e}")))?;
        Ok(buffer)
    }
}

#[derive(Debug, Clone, Copy)]
struct PreviousGlyph {
    id: ab_glyph::GlyphId,
    bold: bool,
    italic: bool,
}
