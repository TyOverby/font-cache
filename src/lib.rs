use std::collections::HashMap;

#[derive(Clone, Copy)]
pub struct CharInfo {
    advance: (u32, u32),
    pixel_offset: (u32, u32),
}

pub struct RenderedFont<I> {
    name: String,
    font_size: u32,

    image: I,
    line_height: u32,
    max_width: u32,
    char_info: HashMap<char, CharInfo>,
    kerning: HashMap<(char, char), (i32, i32)>
}

pub struct OutputPosition {
    pub c: char,
    pub pos: (i32, i32),
    pub size: (u32, u32)
}

impl <I> RenderedFont<I> {
    pub fn new(
        name: String,
        font_size: u32,

        image: I,
        line_height: u32,
        max_width: u32,
        char_info: HashMap<char, CharInfo>,
        kerning: HashMap<(char, char), (i32, i32)>) -> RenderedFont<I> {

        RenderedFont {
            name: name,
            font_size: font_size,

            image: image,
            line_height: line_height,
            max_width: max_width,
            char_info: char_info,
            kerning: kerning
        }
    }

    pub fn kerning(&self, a: char, b: char) -> (i32, i32) {
        self.kerning.get(&(a, b)).cloned().unwrap_or((0, 0))
    }

    pub fn line_height(&self) -> u32 {
        self.line_height
    }

    pub fn max_width(&self) -> u32 {
        self.max_width
    }

    pub fn char_info(&self, c: char) -> Option<CharInfo> {
        self.char_info.get(&c).cloned()
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn size(&self) -> u32 {
        self.font_size
    }

    pub fn image(&self) -> &I {
        &self.image
    }

    pub fn image_mut(&mut self) -> &mut I {
        &mut self.image
    }

    pub fn map_img<B, F>(self, mapping_fn: F) -> RenderedFont<B>
    where F: FnOnce(I) -> B {
        RenderedFont {
            name: self.name,
            font_size: self.font_size,

            image: mapping_fn(self.image),
            line_height: self.line_height,
            max_width: self.max_width,
            char_info: self.char_info,
            kerning: self.kerning
        }

    }

    pub fn positions_for(&self, text: &str) -> Vec<OutputPosition> {
        let mut out = Vec::with_capacity(text.len());

        let mut x: i32 = 0;
        let mut y: i32 = self.line_height() as i32;

        let mut prev = None;

        for current in text.chars() {
            if current == '\n' {
                x = 0;
                y += self.line_height() as i32;
                prev = None;
                continue;
            }

            if let Some(prev) = prev {
                let (dx, dy) = self.kerning(prev, current);
                x += dx;
                y += dy;
            }

            if let Some(CharInfo{advance, pixel_offset}) = self.char_info(current) {
                let (ox, oy) = pixel_offset;
                let pos = (x + ox as i32, y - oy as i32);
                let size = advance;

                out.push(OutputPosition {
                    c: current,
                    pos: pos,
                    size: size,
                });

                let (dx, dy) = advance;
                x += dx as i32;
                y += dy as i32;
            }

            prev = Some(current);
        }

        out
    }
}

