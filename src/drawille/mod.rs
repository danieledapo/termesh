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
    rows: BTreeMap<i32, BTreeMap<i32, u8>>,
}

impl Canvas {
    pub fn new() -> Self {
        Canvas {
            rows: BTreeMap::new(),
        }
    }

    pub fn clear(&mut self) {
        self.rows.clear();
    }

    pub fn triangle(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32) {
        self.line(x1, y1, x2, y2);
        self.line(x1, y1, x3, y3);
        self.line(x2, y2, x3, y3);
    }

    pub fn line(&mut self, x1: f32, y1: f32, x2: f32, y2: f32) {
        for (x, y) in line::Line::new(x1.round(), y1.round(), x2.round(), y2.round()) {
            self.set(x, y);
        }
    }

    pub fn set(&mut self, x: f32, y: f32) {
        let (c, r) = canvas_pos(x, y);

        *self.rows.entry(r).or_default().entry(c).or_default() |= braille_offset_at(x, y);
    }

    pub fn unset(&mut self, x: f32, y: f32) {
        use std::collections::btree_map::Entry;

        let (c, r) = canvas_pos(x, y);

        if let Entry::Occupied(mut row) = self.rows.entry(r) {
            if let Entry::Occupied(mut c) = row.get_mut().entry(c) {
                *c.get_mut() &= !braille_offset_at(x, y);

                if *c.get() == 0 {
                    c.remove();
                }
            }

            if row.get().is_empty() {
                row.remove();
            }
        }
    }

    pub fn is_set(&self, x: f32, y: f32) -> bool {
        let dot_index = braille_offset_at(x, y);
        let (x, y) = canvas_pos(x, y);

        self.rows
            .get(&y)
            .and_then(|row| row.get(&x))
            .map_or(false, |c| c & dot_index != 0)
    }

    pub fn rows(&self) -> Rows {
        Rows::new(self)
    }
}

#[derive(Debug)]
pub struct Rows<'a> {
    canvas: &'a Canvas,
    min_row: i32,
    max_row: i32,
    min_col: i32,
}

impl<'a> Rows<'a> {
    fn new(canvas: &'a Canvas) -> Self {
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
        }
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
                        row.get(&x).map_or(BRAILLE_PATTERN_BLANK, |&off| {
                            std::char::from_u32(BRAILLE_PATTERN_BLANK as u32 + u32::from(off))
                                .unwrap()
                        })
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

    use super::Canvas;

    #[test]
    fn test_set() {
        let mut c = Canvas::new();

        c.set(0.0, 0.0);

        assert_eq!(
            c.rows,
            btreemap!{
                0 => btreemap!{0 => 1}
            }
        );
    }

    #[test]
    fn test_unset_empty() {
        let mut c = Canvas::new();

        c.set(1.0, 1.0);
        c.unset(1.0, 1.0);

        assert_eq!(c.rows, btreemap!{});
    }

    #[test]
    fn test_unset_non_empty() {
        let mut c = Canvas::new();

        c.set(0.0, 0.0);
        c.set(1.0, 1.0);
        c.unset(1.0, 1.0);

        assert_eq!(c.rows, btreemap!{ 0 => btreemap!{ 0 => 1 }});
    }

    #[test]
    fn test_clear() {
        let mut c = Canvas::new();

        c.set(1.0, 1.0);
        c.clear();

        assert_eq!(c.rows, btreemap!{});
    }

    #[test]
    fn test_get() {
        let mut c = Canvas::new();

        assert!(!c.is_set(0.0, 0.0));

        c.set(0.0, 0.0);
        assert!(c.is_set(0.0, 0.0));
        assert!(!c.is_set(0.0, 1.0));
        assert!(!c.is_set(1.0, 0.0));
        assert!(!c.is_set(1.0, 1.0));
    }

    #[test]
    fn test_frame() {
        let mut c = Canvas::new();
        assert_eq!(c.rows().collect::<Vec<_>>(), Vec::<String>::new());

        c.set(0.0, 0.0);
        assert_eq!(c.rows().collect::<Vec<_>>(), vec!["⠁".to_string()]);
    }

    #[test]
    fn test_rect() {
        let mut c = Canvas::new();

        c.line(0.0, 0.0, 20.0, 0.0);
        c.line(20.0, 0.0, 20.0, 20.0);
        c.line(0.0, 20.0, 20.0, 20.0);
        c.line(0.0, 0.0, 0.0, 20.0);
        c.line(0.0, 0.0, 20.0, 20.0);
        c.line(20.0, 0.0, 0.0, 20.0);

        assert_eq!(
            c.rows().collect::<Vec<_>>(),
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
            s.set(
                f32::from(x) / 20.0,
                (4.0 + f32::from(x).to_radians().sin() * 4.0).round(),
            );
        }

        let rows = s.rows().collect::<Vec<_>>();

        assert_eq!(rows, vec![
            "⠀⠀⠀⠀⠀⡐⠊⠑⢂⠀⠀⠀⠀⠀⡐⠊⠑⢂⠀⠀⠀⠀⠀⡐⠊⠑⢂⠀⠀⠀⠀⠀⡐⠊⠑⢂⠀⠀⠀⠀⠀⡐⠊⠑⢂⠀⠀⠀⠀⠀⡐⠊⠑⢂⠀⠀⠀⠀⠀⡐⠊⠑⢂⠀⠀⠀⠀⠀⡐⠊⠑⢂⠀⠀⠀⠀⠀⡐⠊⠑⢂⠀⠀⠀⠀⠀⡐⠊⠑⢂",
            "⠑⣀⠀⣀⠊⠀⠀⠀⠀⠑⣀⠀⣀⠊⠀⠀⠀⠀⠑⣀⠀⣀⠊⠀⠀⠀⠀⠑⣀⠀⣀⠊⠀⠀⠀⠀⠑⣀⠀⣀⠊⠀⠀⠀⠀⠑⣀⠀⣀⠊⠀⠀⠀⠀⠑⣀⠀⣀⠊⠀⠀⠀⠀⠑⣀⠀⣀⠊⠀⠀⠀⠀⠑⣀⠀⣀⠊⠀⠀⠀⠀⠑⣀⠀⣀⠊",
            "⠀⠀⠉⠀⠀⠀⠀⠀⠀⠀⠀⠉⠀⠀⠀⠀⠀⠀⠀⠀⠉⠀⠀⠀⠀⠀⠀⠀⠀⠉⠀⠀⠀⠀⠀⠀⠀⠀⠉⠀⠀⠀⠀⠀⠀⠀⠀⠉⠀⠀⠀⠀⠀⠀⠀⠀⠉⠀⠀⠀⠀⠀⠀⠀⠀⠉⠀⠀⠀⠀⠀⠀⠀⠀⠉⠀⠀⠀⠀⠀⠀⠀⠀⠉",
            ]);
    }
}
