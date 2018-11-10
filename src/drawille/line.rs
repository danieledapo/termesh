pub struct Line {
    x1: usize,
    y1: usize,
    xdiff: usize,
    ydiff: usize,
    xdir: f64,
    ydir: f64,
    i: usize,
}

impl Line {
    pub fn new(x1: usize, y1: usize, x2: usize, y2: usize) -> Self {
        Self {
            x1,
            y1,
            xdiff: x1.max(x2) - x1.min(x2),
            ydiff: y1.max(y2) - y1.min(y2),
            xdir: if x1 <= x2 { 1.0 } else { -1.0 },
            ydir: if y1 <= y2 { 1.0 } else { -1.0 },
            i: 0,
        }
    }
}

impl Iterator for Line {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        use num_traits::cast;

        let r = self.xdiff.max(self.ydiff);

        if self.i > r {
            return None;
        }

        let mut x: f64 = cast(self.x1).unwrap();
        let mut y: f64 = cast(self.y1).unwrap();
        let r: f64 = cast(r).unwrap();

        if self.xdiff != 0 {
            let i: f64 = cast(self.i).unwrap();
            let xdiff: f64 = cast(self.xdiff).unwrap();

            x += i * xdiff / r * self.xdir;
        }

        if self.ydiff != 0 {
            let i: f64 = cast(self.i).unwrap();
            let ydiff: f64 = cast(self.ydiff).unwrap();

            y += i * ydiff / r * self.ydir;
        }

        self.i += 1;

        Some((cast(x).unwrap(), cast(y).unwrap()))
    }
}

#[cfg(test)]
mod tests {
    use super::Line;

    #[test]
    fn test_horizontal() {
        let mut pts = vec![(3, 5), (4, 5), (5, 5), (6, 5), (7, 5), (8, 5), (9, 5)];
        assert_eq!(Line::new(3, 5, 9, 5).into_iter().collect::<Vec<_>>(), pts);

        pts.reverse();
        assert_eq!(Line::new(9, 5, 3, 5).into_iter().collect::<Vec<_>>(), pts);
    }

    #[test]
    fn test_vertical() {
        let mut pts = vec![(7, 0), (7, 1), (7, 2), (7, 3), (7, 4), (7, 5)];
        assert_eq!(Line::new(7, 0, 7, 5).into_iter().collect::<Vec<_>>(), pts);

        pts.reverse();
        assert_eq!(Line::new(7, 5, 7, 0).into_iter().collect::<Vec<_>>(), pts);
    }

    #[test]
    fn test_diagonal() {
        let mut pts = vec![
            (0, 0),
            (1, 1),
            (2, 2),
            (3, 3),
            (4, 4),
            (5, 5),
            (6, 6),
            (7, 7),
            (8, 8),
            (9, 9),
            (10, 10),
            (11, 11),
            (12, 12),
            (13, 13),
            (14, 14),
            (15, 15),
            (16, 16),
            (17, 17),
            (18, 18),
            (19, 19),
            (20, 20),
        ];
        assert_eq!(Line::new(0, 0, 20, 20).into_iter().collect::<Vec<_>>(), pts);

        pts.reverse();
        assert_eq!(Line::new(20, 20, 0, 0).into_iter().collect::<Vec<_>>(), pts);
    }
}
