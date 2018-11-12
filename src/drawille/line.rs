pub struct Line {
    x1: f32,
    y1: f32,
    xdiff: f32,
    ydiff: f32,
    xdir: f32,
    ydir: f32,
    i: f32,
}

impl Line {
    pub fn new(x1: f32, y1: f32, x2: f32, y2: f32) -> Self {
        Self {
            x1,
            y1,
            xdiff: (x1 - x2).abs(),
            ydiff: (y1 - y2).abs(),
            xdir: if x1 <= x2 { 1.0 } else { -1.0 },
            ydir: if y1 <= y2 { 1.0 } else { -1.0 },
            i: 0.0,
        }
    }
}

impl Iterator for Line {
    type Item = (f32, f32);

    fn next(&mut self) -> Option<Self::Item> {
        let r = self.xdiff.max(self.ydiff);

        if self.i > r {
            return None;
        }

        let mut x = self.x1;
        let mut y = self.y1;

        if self.xdiff != 0.0 {
            x += self.i * self.xdiff / r * self.xdir;
        }

        if self.ydiff != 0.0 {
            y += self.i * self.ydiff / r * self.ydir;
        }

        self.i += 1.0;

        Some((x, y))
    }
}

#[cfg(test)]
mod tests {
    use super::Line;

    #[test]
    fn test_horizontal() {
        let mut pts = vec![
            (3.0, 5.0),
            (4.0, 5.0),
            (5.0, 5.0),
            (6.0, 5.0),
            (7.0, 5.0),
            (8.0, 5.0),
            (9.0, 5.0),
        ];
        assert_eq!(
            Line::new(3.0, 5.0, 9.0, 5.0)
                .into_iter()
                .collect::<Vec<_>>(),
            pts
        );

        pts.reverse();
        assert_eq!(
            Line::new(9.0, 5.0, 3.0, 5.0)
                .into_iter()
                .collect::<Vec<_>>(),
            pts
        );
    }

    #[test]
    fn test_vertical() {
        let mut pts = vec![
            (7.0, 0.0),
            (7.0, 1.0),
            (7.0, 2.0),
            (7.0, 3.0),
            (7.0, 4.0),
            (7.0, 5.0),
        ];
        assert_eq!(
            Line::new(7.0, 0.0, 7.0, 5.0)
                .into_iter()
                .collect::<Vec<_>>(),
            pts
        );

        pts.reverse();
        assert_eq!(
            Line::new(7.0, 5.0, 7.0, 0.0)
                .into_iter()
                .collect::<Vec<_>>(),
            pts
        );
    }

    #[test]
    fn test_diagonal() {
        let mut pts = vec![
            (0.0, 0.0),
            (1.0, 1.0),
            (2.0, 2.0),
            (3.0, 3.0),
            (4.0, 4.0),
            (5.0, 5.0),
            (6.0, 6.0),
            (7.0, 7.0),
            (8.0, 8.0),
            (9.0, 9.0),
            (10.0, 10.0),
            (11.0, 11.0),
            (12.0, 12.0),
            (13.0, 13.0),
            (14.0, 14.0),
            (15.0, 15.0),
            (16.0, 16.0),
            (17.0, 17.0),
            (18.0, 18.0),
            (19.0, 19.0),
            (20.0, 20.0),
        ];
        assert_eq!(
            Line::new(0.0, 0.0, 20.0, 20.0)
                .into_iter()
                .collect::<Vec<_>>(),
            pts
        );

        pts.reverse();
        assert_eq!(
            Line::new(20.0, 20.0, 0.0, 0.0)
                .into_iter()
                .collect::<Vec<_>>(),
            pts
        );
    }
}
