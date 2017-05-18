use rusttype::{FontCollection, Scale, Font, point};
use Bingo;
use image::{RgbaImage, Rgba, Pixel};
use imageproc::drawing::{draw_hollow_rect_mut, draw_filled_rect_mut};
use imageproc::rect::Rect;
use imageproc::filter::gaussian_blur_f32;
use std::mem::replace;

fn calculate_width(font: &Font, text: &str, scale: Scale) -> i32 {
    if let Some(glyph) = font.layout(text, scale, point(0.0, 0.0)).last() {
        if let Some(bb) = glyph.pixel_bounding_box() {
            return bb.max.x;
        }
    }
    0
}

pub fn render(board: &Bingo,
              cell_size: u32,
              cell_padding: i32,
              font: &[u8],
              font_size: f32)
              -> RgbaImage {
    let total_size = 5 * cell_size;
    let font = FontCollection::from_bytes(font).into_font().unwrap();
    let scale = Scale::uniform(font_size);
    let v_metrics = font.v_metrics(scale);
    let v_align = v_metrics.ascent - font_size / 2.0;
    let line_size = font_size + v_metrics.line_gap;

    let mut image = RgbaImage::new(total_size, total_size);

    draw_filled_rect_mut(&mut image,
                         Rect::at(0, 0).of_size(total_size, total_size),
                         Rgba::from_channels(44, 47, 52, 255));

    for cell_x in 0..5 {
        let begin_x = cell_size * cell_x;

        for cell_y in 0..5 {
            let begin_y = cell_size * cell_y;

            draw_hollow_rect_mut(&mut image,
                                 Rect::at(begin_x as i32 - 1, begin_y as i32 - 1)
                                     .of_size(cell_size, cell_size),
                                 Rgba::from_channels(30, 35, 40, 255));

            draw_hollow_rect_mut(&mut image,
                                 Rect::at(begin_x as i32 + 1, begin_y as i32 + 1)
                                     .of_size(cell_size, cell_size),
                                 Rgba::from_channels(90, 95, 100, 255));
        }
    }

    image = gaussian_blur_f32(&image, 0.5);

    for (cell_y, row) in board.cells.iter().enumerate() {
        let begin_y = cell_size * cell_y as u32;
        let end_y = begin_y + cell_size;
        let center_y = (begin_y as f32 + end_y as f32) / 2.0;
        for (cell_x, text) in row.iter().enumerate() {
            let begin_x = cell_size * cell_x as u32;

            let text = &text.replace(" ★", "");

            let mut lines = Vec::new();
            let mut line = String::new();
            let mut test_buf = String::new();
            for word in text.split_whitespace() {
                test_buf.push_str(word);
                let line_width = calculate_width(&font, &test_buf, scale);
                if line_width + 2 * cell_padding > cell_size as i32 {
                    let finished_line = replace(&mut line, String::new());
                    lines.push(finished_line);
                    test_buf.clear();
                } else if !line.is_empty() {
                    line.push_str(" ");
                    test_buf.push_str(" ");
                }
                line.push_str(word);
                test_buf.push_str(word);
            }
            if !line.is_empty() {
                lines.push(line);
            }

            let lines_height = line_size * (lines.len() as f32 - 1.0);
            let lines_offset = -(lines_height / 2.0);

            for (line_i, line) in lines.iter().enumerate() {
                let line_width = calculate_width(&font, line, scale);
                let line_x = (cell_size as f32 - line_width as f32) / 2.0 + begin_x as f32;
                let line_y = lines_offset + line_size * line_i as f32 + v_align + center_y;

                let layout = font.layout(line, scale, point(line_x, line_y));

                for glyph in layout {
                    if let Some(bb) = glyph.pixel_bounding_box() {
                        glyph.draw(|x, y, v| {
                            let x = x as i32 + bb.min.x;
                            let y = y as i32 + bb.min.y;
                            if x < total_size as i32 && y < total_size as i32 && x >= 0 && y >= 0 {
                                let pixel = image.get_pixel_mut(x as _, y as _);
                                pixel.blend(&Rgba::from_channels(255, 255, 255, (v * 255.0) as _));
                            }
                        });
                    }
                }
            }
        }
    }

    image
}
