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

use crate::utils::btree_minmax;
use crate::vector3::Vector3;

static BRAILLE_PATTERN_BLANK: char = '\u{2800}';
static BRAILLE_OFFSET_MAP: [[u8; 2]; 4] = [
    [0x01, 0x08], // "⠁" , "⠈"
    [0x02, 0x10], // "⠂" , "⠐"
    [0x04, 0x20], // "⠄" , "⠐"
    [0x40, 0x80], // "⡀" , "⢀"
];

fn canvas_pos(x: f32, y: f32) -> (i32, i32) {
    (
        (x.round() / 2.0).floor() as i32,
        (y.round() / 4.0).floor() as i32,
    )
}

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
    pub fn new() -> Self {
        Canvas {
            rows: BTreeMap::new(),
            zrange: None,
        }
    }

    pub fn clear(&mut self) {
        self.rows.clear();
    }

    pub fn triangle(&mut self, p0: Vector3, p1: Vector3, p2: Vector3) {
        self.line(p0, p1);
        self.line(p0, p2);
        self.line(p1, p2);
    }

    pub fn fill_triangle(&mut self, mut p0: Vector3, mut p1: Vector3, mut p2: Vector3) {
        // TODO: doesn't seem to completely fill triangles...

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

        for (line_start, line_end) in line::Line::new(p0, p1).zip(line::Line::new(p0, p2)) {
            self.line(line_start, line_end);
        }

        for (line_start, line_end) in line::Line::new(p2, p0).zip(line::Line::new(p2, p1)) {
            self.line(line_start, line_end);
        }
    }

    pub fn line(&mut self, p0: Vector3, p1: Vector3) {
        for p in line::Line::new(p0.round(), p1.round()) {
            self.set(p);
        }
    }

    pub fn set(&mut self, p: Vector3) {
        use std::collections::btree_map::Entry;

        let (c, r) = canvas_pos(p.x, p.y);

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
        let (x, y) = canvas_pos(x, y);

        self.rows
            .get(&y)
            .and_then(|row| row.get(&x))
            .map_or(false, |c| c.braille_offset & dot_index != 0)
    }

    pub fn rows(&self, with_colors: bool) -> Rows {
        Rows::new(self, with_colors)
    }
}

#[derive(Debug)]
pub struct Rows<'a> {
    canvas: &'a Canvas,
    with_colors: bool,
    min_row: i32,
    max_row: i32,
    min_col: i32,
}

impl<'a> Rows<'a> {
    fn new(canvas: &'a Canvas, with_colors: bool) -> Self {
        let (min_row, max_row, min_col) = match btree_minmax(&canvas.rows) {
            None => (i32::max_value(), 0, 0),
            Some((&min_row, &max_row)) => {
                let min_c = canvas
                    .rows
                    .values()
                    .flat_map(|row| btree_minmax(row).map(|(m, _)| m))
                    .min();

                // if btree_minmax(&canvas.rows) succeeded there's no way this can fail
                let &min_c = min_c.unwrap();

                (min_row, max_row, min_c)
            }
        };

        Self {
            canvas,
            min_row,
            max_row,
            min_col,
            with_colors,
        }
    }

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
        let gray = 23_u8
            - num_traits::cast::<_, u8>(((pix.z - zmin) / (zmax - zmin) * 20.0).round()).unwrap();

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
            Some(row) => match btree_minmax(row) {
                None => String::new(),
                Some((_, &max_c)) => (self.min_col..=max_c)
                    .map(|x| {
                        row.get(&x)
                            .map_or(BRAILLE_PATTERN_BLANK.to_string(), |pix| self.braille(pix))
                    })
                    .collect(),
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
