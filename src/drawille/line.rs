use crate::Vector3;

#[derive(Debug)]
pub struct Line {
    p0: Vector3,
    step: Vector3,
    steps: u64,
}

impl Line {
    pub fn new(p0: Vector3, p1: Vector3) -> Self {
        let dir = p1 - p0;

        let steps = dir.x.abs().max(dir.y.abs()).max(dir.z.abs());

        // if steps == 0.0 the line is actually just a point
        if steps == 0.0 {
            Self {
                p0,
                step: Vector3::new(0.0, 0.0, 0.0),
                steps: 1,
            }
        } else {
            Self {
                p0,
                step: dir / steps,
                steps: num_traits::cast(steps.round().abs() + 1.0).unwrap(),
            }
        }
    }
}

impl Iterator for Line {
    type Item = Vector3;

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, num_traits::cast(self.steps))
    }

    fn next(&mut self) -> Option<Self::Item> {
        if self.steps == 0 {
            return None;
        }

        let p = self.p0;

        self.p0 += self.step;
        self.steps -= 1;

        Some(p)
    }
}

#[cfg(test)]
mod tests {
    use super::{Line, Vector3};

    #[test]
    fn test_point() {
        assert_eq!(
            Line::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 0.0))
                .into_iter()
                .collect::<Vec<_>>(),
            vec![Vector3::new(0.0, 0.0, 0.0)]
        );
    }

    #[test]
    fn test_horizontal() {
        let mut pts = vec![
            Vector3::new(3.0, 5.0, 0.0),
            Vector3::new(4.0, 5.0, 0.0),
            Vector3::new(5.0, 5.0, 0.0),
            Vector3::new(6.0, 5.0, 0.0),
            Vector3::new(7.0, 5.0, 0.0),
            Vector3::new(8.0, 5.0, 0.0),
            Vector3::new(9.0, 5.0, 0.0),
        ];
        assert_eq!(
            Line::new(Vector3::new(3.0, 5.0, 0.0), Vector3::new(9.0, 5.0, 0.0))
                .into_iter()
                .collect::<Vec<_>>(),
            pts
        );

        pts.reverse();
        assert_eq!(
            Line::new(Vector3::new(9.0, 5.0, 0.0), Vector3::new(3.0, 5.0, 0.0))
                .into_iter()
                .collect::<Vec<_>>(),
            pts
        );
    }

    #[test]
    fn test_vertical() {
        let mut pts = vec![
            Vector3::new(7.0, 0.0, 0.0),
            Vector3::new(7.0, 1.0, 0.0),
            Vector3::new(7.0, 2.0, 0.0),
            Vector3::new(7.0, 3.0, 0.0),
            Vector3::new(7.0, 4.0, 0.0),
            Vector3::new(7.0, 5.0, 0.0),
        ];
        assert_eq!(
            Line::new(Vector3::new(7.0, 0.0, 0.0), Vector3::new(7.0, 5.0, 0.0))
                .into_iter()
                .collect::<Vec<_>>(),
            pts
        );

        pts.reverse();
        assert_eq!(
            Line::new(Vector3::new(7.0, 5.0, 0.0), Vector3::new(7.0, 0.0, 0.0))
                .into_iter()
                .collect::<Vec<_>>(),
            pts
        );
    }

    #[test]
    fn test_diagonal() {
        let mut pts = vec![
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(1.0, 1.0, 1.0),
            Vector3::new(2.0, 2.0, 2.0),
            Vector3::new(3.0, 3.0, 3.0),
            Vector3::new(4.0, 4.0, 4.0),
            Vector3::new(5.0, 5.0, 5.0),
            Vector3::new(6.0, 6.0, 6.0),
            Vector3::new(7.0, 7.0, 7.0),
            Vector3::new(8.0, 8.0, 8.0),
            Vector3::new(9.0, 9.0, 9.0),
            Vector3::new(10.0, 10.0, 10.0),
            Vector3::new(11.0, 11.0, 11.0),
            Vector3::new(12.0, 12.0, 12.0),
            Vector3::new(13.0, 13.0, 13.0),
            Vector3::new(14.0, 14.0, 14.0),
            Vector3::new(15.0, 15.0, 15.0),
            Vector3::new(16.0, 16.0, 16.0),
            Vector3::new(17.0, 17.0, 17.0),
            Vector3::new(18.0, 18.0, 18.0),
            Vector3::new(19.0, 19.0, 19.0),
            Vector3::new(20.0, 20.0, 20.0),
        ];
        assert_eq!(
            Line::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(20.0, 20.0, 20.0))
                .into_iter()
                .collect::<Vec<_>>(),
            pts
        );

        pts.reverse();
        assert_eq!(
            Line::new(Vector3::new(20.0, 20.0, 20.0), Vector3::new(0.0, 0.0, 0.0))
                .into_iter()
                .collect::<Vec<_>>(),
            pts
        );
    }
}
