#[derive(Debug)]
pub struct Line {
    x1: f32,
    y1: f32,
    xstep: f32,
    ystep: f32,
    steps: u64,
}

impl Line {
    pub fn new(x1: f32, y1: f32, x2: f32, y2: f32) -> Self {
        let xdiff = x2 - x1;
        let ydiff = y2 - y1;

        let steps = xdiff.abs().max(ydiff.abs());

        // if steps == 0.0 the line is actually just a point
        if steps == 0.0 {
            Self {
                x1,
                y1,
                xstep: xdiff,
                ystep: ydiff,
                steps: 1,
            }
        } else {
            Self {
                x1,
                y1,
                xstep: xdiff / steps,
                ystep: ydiff / steps,
                steps: num_traits::cast(steps.round().abs() + 1.0).unwrap(),
            }
        }
    }
}

impl Iterator for Line {
    type Item = (f32, f32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.steps == 0 {
            return None;
        }

        let x = self.x1;
        let y = self.y1;

        self.x1 += self.xstep;
        self.y1 += self.ystep;
        self.steps -= 1;

        Some((x, y))
    }
}

#[cfg(test)]
mod tests {
    use super::Line;

    #[test]
    fn test_point() {
        assert_eq!(
            Line::new(0.0, 0.0, 0.0, 0.0)
                .into_iter()
                .collect::<Vec<_>>(),
            vec![(0.0, 0.0)]
        );
    }

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
