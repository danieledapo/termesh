//! Small subset of the awesome [drawille](https://github.com/asciimoo/drawille)
//! library. The way this thing works is pretty clever: Unicode supports Braille
//! characters which essentially are dots in a 4 (rows) x 2 (columns) matrix and
//! we can use these dots patterns to emulate corners, slope, density, etc...
//! The best part is that the elements in the palette are composable by "or-ing"
//! the offsets, for example composing a high left dot with a left middle dot
//! gives the character that has a two dot vertical bar on the left!
//!
//! Since each character is a 4x2 matrix the user coordinates are remapped in
//! 4x2 cells.
//!

pub mod line;

use std::collections::BTreeMap;

mod utils;
use self::utils::btree_minmax;

use crate::Vector3;

static BRAILLE_PATTERN_BLANK: char = '\u{2800}';
static BRAILLE_OFFSET_MAP: [[u8; 2]; 4] = [
    [0x01, 0x08], // "⠁" , "⠈"
    [0x02, 0x10], // "⠂" , "⠐"
    [0x04, 0x20], // "⠄" , "⠐"
    [0x40, 0x80], // "⡀" , "⢀"
];

fn braille_offset_at(x: f32, y: f32) -> u8 {
    let mut xoff = x.round() % 2.0;
    if xoff < 0.0 {
        xoff += 2.0;
    }

    let mut yoff = y.round() % 4.0;
    if yoff < 0.0 {
        yoff += 4.0;
    }

    BRAILLE_OFFSET_MAP[yoff as usize][xoff as usize]
}

#[derive(Debug)]
pub struct Canvas {
    rows: BTreeMap<i32, BTreeMap<i32, Pixel>>,
    zrange: Option<(f32, f32)>,
}

#[derive(Debug, PartialEq)]
struct Pixel {
    // offset to sum to `BRAILLE_PATTERN_BLANK` to obtain the braille character
    // for a pixel
    braille_offset: u8,

    // z of the pixel, the smaller the closer to the camera it is
    z: f32,
}

impl Canvas {
    /// The optimal background color for rendering the Canvas
    pub fn background_color() -> impl termion::color::Color {
        termion::color::AnsiValue::grayscale(3)
    }

    pub fn new() -> Self {
        Canvas {
            rows: BTreeMap::new(),
            zrange: None,
        }
    }

    // convert coordinates from user space to canvas space
    pub fn pos(x: f32, y: f32) -> (i32, i32) {
        (
            (x.round() / 2.0).floor() as i32,
            (y.round() / 4.0).floor() as i32,
        )
    }

    pub fn rows(&self, with_colors: bool) -> Rows {
        let (min_row, max_row, min_col, _) =
            self.dimensions().unwrap_or((i32::max_value(), 0, 0, 0));

        self.frame(with_colors, min_row, max_row, min_col, None)
    }

    /// bounds are in canvas space, use `pos` to perform the conversion if
    /// needed.
    ///
    /// If `max_col` is `None` then each row is composed by the minimum
    /// characters for that row only and that each row possibly has a different
    /// length. On the other hand, when it's `Some(width)` all rows will have
    /// the same length.
    pub fn frame(
        &self,
        with_colors: bool,
        min_row: i32,
        max_row: i32,
        min_col: i32,
        max_col: Option<i32>,
    ) -> Rows {
        Rows {
            canvas: self,
            min_row,
            max_row,
            min_col,
            max_col,
            with_colors,
        }
    }

    // Get a tuple with the minimum row, maximum row, minimum column and maximum
    // column values in canvas space.
    pub fn dimensions(&self) -> Option<(i32, i32, i32, i32)> {
        btree_minmax(&self.rows).map(|(&min_row, &max_row)| {
            let (min_c, max_c) = self
                .rows
                .values()
                .map(|r| btree_minmax(r).unwrap_or((&0, &0)))
                .fold(
                    (i32::max_value(), i32::min_value()),
                    |(min_c, max_c), (row_min_c, row_max_c)| {
                        (min_c.min(*row_min_c), max_c.max(*row_max_c))
                    },
                );

            (min_row, max_row, min_c, max_c)
        })
    }

    pub fn clear(&mut self) {
        self.rows.clear();
    }

    pub fn line(&mut self, p0: Vector3, p1: Vector3) {
        for p in line::Line::new(p0.round(), p1.round()) {
            self.set(p);
        }
    }

    pub fn set(&mut self, p: Vector3) {
        use std::collections::btree_map::Entry;

        let (c, r) = Self::pos(p.x, p.y);

        self.zrange = match self.zrange {
            None => Some((p.z, p.z)),
            Some((minz, maxz)) => Some((minz.min(p.z), maxz.max(p.z))),
        };

        let braille_offset = braille_offset_at(p.x, p.y);

        match self.rows.entry(r).or_default().entry(c) {
            Entry::Vacant(v) => {
                v.insert(Pixel {
                    braille_offset,
                    z: p.z,
                });
            }
            Entry::Occupied(mut o) => {
                let pix = o.get_mut();

                pix.braille_offset |= braille_offset;
                pix.z = pix.z.min(p.z);
            }
        };
    }

    pub fn is_set(&self, x: f32, y: f32) -> bool {
        let dot_index = braille_offset_at(x, y);
        let (x, y) = Self::pos(x, y);

        self.rows
            .get(&y)
            .and_then(|row| row.get(&x))
            .map_or(false, |c| c.braille_offset & dot_index != 0)
    }

    pub fn triangle(&mut self, p0: Vector3, p1: Vector3, p2: Vector3) {
        let midz = (p0.z + p1.z + p2.z) / 3.0;
        self.triangle_line(p0, p1, midz);
        self.triangle_line(p0, p2, midz);
        self.triangle_line(p1, p2, midz);
    }

    pub fn fill_triangle(&mut self, mut p0: Vector3, mut p1: Vector3, mut p2: Vector3) {
        // TODO: speed this up

        // ensure p0 is the point with highest y, then comes p1 and then p2
        if p1.y < p0.y {
            std::mem::swap(&mut p0, &mut p1);
        }
        if p2.y < p0.y {
            std::mem::swap(&mut p0, &mut p2);
        }
        if p2.y < p1.y {
            std::mem::swap(&mut p1, &mut p2);
        }

        let midz = (p0.z + p1.z + p2.z) / 3.0;

        for (line_start, line_end) in line::Line::new(p0, p1).zip(line::Line::new(p0, p2)) {
            self.triangle_line(line_start, line_end, midz);
        }

        for (line_start, line_end) in line::Line::new(p2, p0).zip(line::Line::new(p2, p1)) {
            self.triangle_line(line_start, line_end, midz);
        }

        for (line_start, line_end) in line::Line::new(p1, p0).zip(line::Line::new(p1, p2)) {
            self.triangle_line(line_start, line_end, midz);
        }

        self.triangle(p0, p1, p2);
    }

    // lines for triangles all have the same z for flat shading
    fn triangle_line(&mut self, p0: Vector3, p1: Vector3, z: f32) {
        for mut p in line::Line::new(p0.round(), p1.round()) {
            p.z = z;
            self.set(p);
        }
    }
}

#[derive(Debug)]
pub struct Rows<'a> {
    canvas: &'a Canvas,
    with_colors: bool,
    min_row: i32,
    max_row: i32,
    min_col: i32,
    max_col: Option<i32>,
}

impl<'a> Rows<'a> {
    fn braille(&self, pix: &Pixel) -> String {
        let c = std::char::from_u32(BRAILLE_PATTERN_BLANK as u32 + u32::from(pix.braille_offset))
            .unwrap();

        if !self.with_colors {
            return c.to_string();
        }

        let (zmin, zmax) = self.canvas.zrange.unwrap();

        // Use the ANSI grayscales as a form of alpha channel to simulate depth.
        // The first shades of black are not taken into account because they're
        // too bright which makes the eyes think the pixel is closer even though
        // it's not.
        let gray = if zmax - zmin != 0.0 {
            23 - num_traits::cast::<_, u8>(((pix.z - zmin) / (zmax - zmin) * 19.0).round()).unwrap()
        } else {
            // if there's only a z value then it's always at the top
            23
        };

        format!(
            "{}{}",
            termion::color::Fg(termion::color::AnsiValue::grayscale(gray)),
            c
        )
    }
}

impl<'a> Iterator for Rows<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.min_row > self.max_row {
            return None;
        }

        let row = match self.canvas.rows.get(&self.min_row) {
            None => String::new(),
            Some(row) => match self
                .max_col
                .or_else(|| btree_minmax(row).map(|(_, &mc)| mc))
            {
                None => String::new(),
                Some(max_c) => (self.min_col..=max_c)
                    .map(|x| {
                        row.get(&x)
                            .map_or(BRAILLE_PATTERN_BLANK.to_string(), |pix| self.braille(pix))
                    }).collect(),
            },
        };

        self.min_row += 1;

        Some(row)
    }
}

impl Default for Canvas {
    fn default() -> Self {
        Canvas::new()
    }
}

#[cfg(test)]
mod tests {
    use maplit::btreemap;

    use super::{Canvas, Pixel, Vector3};

    #[test]
    fn test_set() {
        let mut c = Canvas::new();

        c.set(Vector3::new(0.0, 0.0, 0.0));

        assert_eq!(
            c.rows,
            btreemap!{
                0 => btreemap!{0 => Pixel {braille_offset: 1, z: 0.0}}
            }
        );
    }

    #[test]
    fn test_clear() {
        let mut c = Canvas::new();

        c.set(Vector3::new(1.0, 1.0, 0.0));
        c.clear();

        assert_eq!(c.rows, btreemap!{});
    }

    #[test]
    fn test_get() {
        let mut c = Canvas::new();

        assert!(!c.is_set(0.0, 0.0));

        c.set(Vector3::new(0.0, 0.0, 0.0));
        assert!(c.is_set(0.0, 0.0));
        assert!(!c.is_set(0.0, 1.0));
        assert!(!c.is_set(1.0, 0.0));
        assert!(!c.is_set(1.0, 1.0));
    }

    #[test]
    fn test_frame() {
        let mut c = Canvas::new();
        assert_eq!(c.rows(false).collect::<Vec<_>>(), Vec::<String>::new());

        c.set(Vector3::new(0.0, 0.0, 0.0));
        assert_eq!(c.rows(false).collect::<Vec<_>>(), vec!["⠁".to_string()]);
    }

    #[test]
    fn test_rect() {
        let mut c = Canvas::new();

        c.line(Vector3::new(0.0, 0.0, 0.0), Vector3::new(20.0, 0.0, 0.0));
        c.line(Vector3::new(20.0, 0.0, 0.0), Vector3::new(20.0, 20.0, 0.0));
        c.line(Vector3::new(0.0, 20.0, 0.0), Vector3::new(20.0, 20.0, 0.0));
        c.line(Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 20.0, 0.0));
        c.line(Vector3::new(0.0, 0.0, 0.0), Vector3::new(20.0, 20.0, 0.0));
        c.line(Vector3::new(20.0, 0.0, 0.0), Vector3::new(0.0, 20.0, 0.0));

        assert_eq!(
            c.rows(false).collect::<Vec<_>>(),
            vec![
                "⡟⢍⠉⠉⠉⠉⠉⠉⢉⠝⡇",
                "⡇⠀⠑⢄⠀⠀⢀⠔⠁⠀⡇",
                "⡇⠀⠀⠀⢑⢔⠁⠀⠀⠀⡇",
                "⡇⠀⢀⠔⠁⠀⠑⢄⠀⠀⡇",
                "⣇⠔⠁⠀⠀⠀⠀⠀⠑⢄⡇",
                "⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠁",
            ]
        );
    }

    #[test]
    fn test_sine_example() {
        let mut s = Canvas::new();

        for x in (0_u16..3600).step_by(20) {
            s.set(Vector3::new(
                f32::from(x) / 20.0,
                (4.0 + f32::from(x).to_radians().sin() * 4.0).round(),
                0.0,
            ));
        }

        let rows = s.rows(false).collect::<Vec<_>>();

        assert_eq!(rows, vec![
            "⠀⠀⠀⠀⠀⡐⠊⠑⢂⠀⠀⠀⠀⠀⡐⠊⠑⢂⠀⠀⠀⠀⠀⡐⠊⠑⢂⠀⠀⠀⠀⠀⡐⠊⠑⢂⠀⠀⠀⠀⠀⡐⠊⠑⢂⠀⠀⠀⠀⠀⡐⠊⠑⢂⠀⠀⠀⠀⠀⡐⠊⠑⢂⠀⠀⠀⠀⠀⡐⠊⠑⢂⠀⠀⠀⠀⠀⡐⠊⠑⢂⠀⠀⠀⠀⠀⡐⠊⠑⢂",
            "⠑⣀⠀⣀⠊⠀⠀⠀⠀⠑⣀⠀⣀⠊⠀⠀⠀⠀⠑⣀⠀⣀⠊⠀⠀⠀⠀⠑⣀⠀⣀⠊⠀⠀⠀⠀⠑⣀⠀⣀⠊⠀⠀⠀⠀⠑⣀⠀⣀⠊⠀⠀⠀⠀⠑⣀⠀⣀⠊⠀⠀⠀⠀⠑⣀⠀⣀⠊⠀⠀⠀⠀⠑⣀⠀⣀⠊⠀⠀⠀⠀⠑⣀⠀⣀⠊",
            "⠀⠀⠉⠀⠀⠀⠀⠀⠀⠀⠀⠉⠀⠀⠀⠀⠀⠀⠀⠀⠉⠀⠀⠀⠀⠀⠀⠀⠀⠉⠀⠀⠀⠀⠀⠀⠀⠀⠉⠀⠀⠀⠀⠀⠀⠀⠀⠉⠀⠀⠀⠀⠀⠀⠀⠀⠉⠀⠀⠀⠀⠀⠀⠀⠀⠉⠀⠀⠀⠀⠀⠀⠀⠀⠉⠀⠀⠀⠀⠀⠀⠀⠀⠉",
            ]);
    }
}
