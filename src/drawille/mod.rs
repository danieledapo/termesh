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

mod line;

use std::collections::BTreeMap;

use crate::utils::btree_minmax;

static BRAILLE_PATTERN_BLANK: char = '\u{2800}';
static BRAILLE_OFFSET_MAP: [[u8; 2]; 4] = [
    [0x01, 0x08], // "⠁" , "⠈"
    [0x02, 0x10], // "⠂" , "⠐"
    [0x04, 0x20], // "⠄" , "⠐"
    [0x40, 0x80], // "⡀" , "⢀"
];

fn canvas_pos(x: usize, y: usize) -> (usize, usize) {
    (x / 2, y / 4)
}

fn braille_offset_at(x: usize, y: usize) -> u8 {
    BRAILLE_OFFSET_MAP[y % 4][x % 2]
}

#[derive(Debug)]
pub struct Canvas {
    rows: BTreeMap<usize, BTreeMap<usize, u8>>,
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

    pub fn line(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        for (x, y) in line::Line::new(x1, y1, x2, y2) {
            self.set(x, y);
        }
    }

    pub fn set(&mut self, x: usize, y: usize) {
        let (c, r) = canvas_pos(x, y);

        *self.rows.entry(r).or_default().entry(c).or_default() |= braille_offset_at(x, y);
    }

    pub fn unset(&mut self, x: usize, y: usize) {
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

    pub fn is_set(&self, x: usize, y: usize) -> bool {
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
    min_row: usize,
    max_row: usize,
    min_col: usize,
}

impl<'a> Rows<'a> {
    fn new(canvas: &'a Canvas) -> Self {
        let (min_row, max_row, min_col) = match btree_minmax(&canvas.rows) {
            None => (usize::max_value(), 0, 0),
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

        c.set(0, 0);

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

        c.set(1, 1);
        c.unset(1, 1);

        assert_eq!(c.rows, btreemap!{});
    }

    #[test]
    fn test_unset_non_empty() {
        let mut c = Canvas::new();

        c.set(0, 0);
        c.set(1, 1);
        c.unset(1, 1);

        assert_eq!(c.rows, btreemap!{ 0 => btreemap!{ 0 => 1 }});
    }

    #[test]
    fn test_clear() {
        let mut c = Canvas::new();

        c.set(1, 1);
        c.clear();

        assert_eq!(c.rows, btreemap!{});
    }

    #[test]
    fn test_get() {
        let mut c = Canvas::new();

        assert!(!c.is_set(0, 0));

        c.set(0, 0);
        assert!(c.is_set(0, 0));
        assert!(!c.is_set(0, 1));
        assert!(!c.is_set(1, 0));
        assert!(!c.is_set(1, 1));
    }

    #[test]
    fn test_frame() {
        let mut c = Canvas::new();
        assert_eq!(c.rows().collect::<Vec<_>>(), Vec::<String>::new());

        c.set(0, 0);
        assert_eq!(c.rows().collect::<Vec<_>>(), vec!["⠁".to_string()]);
    }

    #[test]
    fn test_rect() {
        let mut c = Canvas::new();

        c.line(0, 0, 20, 0);
        c.line(20, 0, 20, 20);
        c.line(0, 20, 20, 20);
        c.line(0, 0, 0, 20);
        c.line(0, 0, 20, 20);
        c.line(20, 0, 0, 20);

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
                usize::from(x / 20),
                (4.0 + f64::from(x).to_radians().sin() * 4.0).round() as usize,
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
