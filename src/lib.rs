use std::collections::HashMap;

/// Placement information about a specific character.
#[derive(Clone, Copy)]
pub struct CharInfo {
    /// The number of pixels (x, y) that are advanced after this character
    /// is drawn.
    pub advance: (u32, u32),
    /// The distance that the pen should move before printing the character.
    pub pixel_offset: (u32, u32),
}

/// A representation of a fully-rendered font that contains a atlas image
/// and the metadata required to draw from it.
pub struct RenderedFont<I> {
    name: String,
    font_size: u32,

    image: I,
    line_height: u32,
    max_width: u32,
    char_info: HashMap<char, CharInfo>,
    kerning: HashMap<(char, char), (i32, i32)>
}

/// The position of a character when drawn from a string.
pub struct OutputPosition {
    /// The character being drawn
    pub c: char,
    /// The position of the character on the screen
    pub pos: (i32, i32),
    /// The size of the character
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

    /// Returns the offsets `(dx, dy)` in pixels that should be applied
    /// to the difference in position between chars `a` and `b` where
    /// `a` comes immediately before `b` in the text.
    ///
    /// If the font doesn't specify a special kerning between these
    /// characters, `(0, 0)` is returned instead.
    pub fn kerning(&self, a: char, b: char) -> (i32, i32) {
        self.kerning.get(&(a, b)).cloned().unwrap_or((0, 0))
    }

    /// Returns the suggested distance between lines of text.
    pub fn line_height(&self) -> u32 {
        self.line_height
    }

    /// Returns the maximum width of a single char using this font.
    pub fn max_width(&self) -> u32 {
        self.max_width
    }

    /// Returns the offset and advance information regarding the specified
    /// character.
    pub fn char_info(&self, c: char) -> Option<CharInfo> {
        self.char_info.get(&c).cloned()
    }

    /// Returns the name of this font.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the size that this font was rendered at.
    pub fn size(&self) -> u32 {
        self.font_size
    }

    /// Returns a reference to the contained image.
    pub fn image(&self) -> &I {
        &self.image
    }

    /// Returns a mutable reference to the contained image.
    pub fn image_mut(&mut self) -> &mut I {
        &mut self.image
    }

    /// Applies a transformation function to the image of this rendered font
    /// producing a new rendered font with that image.
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

    /// Given a string, this function returns a vec containing all of the
    /// positions of each character as it should be rendered to the screen.
    ///
    /// The position is relative to the (0, 0) coordinate, and progress
    /// in the +x, +y direction.
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

