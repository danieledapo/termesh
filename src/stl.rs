//! Simple module to parse stl files.

use std::fmt;
use std::io;
use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};

pub type Vector3 = (f32, f32, f32);

#[derive(Clone)]
pub struct Stl {
    header: [u8; 80],
    facets: Vec<Facet>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Facet {
    vertices: [Vector3; 3],
    normal: Vector3,
}

impl Stl {
    pub fn parse_binary<R: Read>(r: &mut R) -> io::Result<Stl> {
        let mut header = [0; 80];

        r.read_exact(&mut header)?;
        let ntriangles = r.read_u32::<LittleEndian>()?;

        let mut facets = Vec::with_capacity(num_traits::cast(ntriangles).unwrap_or(0));

        let parse_v3 = |r: &mut R| -> io::Result<Vector3> {
            let x = r.read_f32::<LittleEndian>()?;
            let y = r.read_f32::<LittleEndian>()?;
            let z = r.read_f32::<LittleEndian>()?;

            Ok((x, y, z))
        };

        for _ in 0..ntriangles {
            let n = parse_v3(r)?;
            let a = parse_v3(r)?;
            let b = parse_v3(r)?;
            let c = parse_v3(r)?;

            let attribute_count = r.read_u16::<LittleEndian>()?;
            io::copy(
                &mut r.by_ref().take(u64::from(attribute_count)),
                &mut io::sink(),
            )?;

            facets.push(Facet {
                vertices: [a, b, c],
                normal: n,
            });
        }

        Ok(Stl { header, facets })
    }
}

// cannot use derive for Stl because header is a fixed length array and Rust
// doesn't auto implement traits for all slices of all possible lenghts.

impl fmt::Debug for Stl {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Stl")
            .field("header", &format_args!("{:?}", &self.header[..]))
            .field("facets", &self.facets)
            .finish()
    }
}

impl PartialEq for Stl {
    fn eq(&self, other: &Stl) -> bool {
        self.header[..] == other.header[..] && self.facets == other.facets
    }
}

#[cfg(test)]
mod tests {
    use std::io;

    use super::{Facet, Stl};

    #[test]
    fn test_parse_cube() {
        let cube = include_bytes!("../data/cube.stl");

        let stl = Stl::parse_binary(&mut io::Cursor::new(&cube[..]));
        assert!(stl.is_ok());

        let stl = stl.unwrap();

        assert_eq!(
            stl,
            Stl {
                header: [
                    b'E', b'x', b'p', b'o', b'r', b't', b'e', b'd', b' ', b'f', b'r', b'o', b'm',
                    b' ', b'B', b'l', b'e', b'n', b'd', b'e', b'r', b'-', b'2', b'.', b'7', b'9',
                    b' ', b'(', b's', b'u', b'b', b' ', b'0', b')', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ],
                facets: vec![
                    Facet {
                        normal: (-1.0, 0.0, 0.0),
                        vertices: [(-1.0, -1.0, -1.0), (-1.0, -1.0, 1.0), (-1.0, 1.0, 1.0)],
                    },
                    Facet {
                        normal: (-1.0, 0.0, 0.0),
                        vertices: [(-1.0, 1.0, 1.0), (-1.0, 1.0, -1.0), (-1.0, -1.0, -1.0)],
                    },
                    Facet {
                        normal: (0.0, 1.0, 0.0),
                        vertices: [(-1.0, 1.0, -1.0), (-1.0, 1.0, 1.0), (1.0, 1.0, 1.0)],
                    },
                    Facet {
                        normal: (0.0, 1.0, 0.0),
                        vertices: [(1.0, 1.0, 1.0), (1.0, 1.0, -1.0), (-1.0, 1.0, -1.0)],
                    },
                    Facet {
                        normal: (1.0, 0.0, 0.0),
                        vertices: [(1.0, 1.0, -1.0), (1.0, 1.0, 1.0), (1.0, -1.0, 1.0)],
                    },
                    Facet {
                        normal: (1.0, 0.0, 0.0),
                        vertices: [(1.0, -1.0, 1.0), (1.0, -1.0, -1.0), (1.0, 1.0, -1.0)],
                    },
                    Facet {
                        normal: (0.0, -1.0, 0.0),
                        vertices: [(-1.0, -1.0, 1.0), (-1.0, -1.0, -1.0), (1.0, -1.0, -1.0)],
                    },
                    Facet {
                        normal: (0.0, -1.0, 0.0),
                        vertices: [(1.0, -1.0, -1.0), (1.0, -1.0, 1.0), (-1.0, -1.0, 1.0)],
                    },
                    Facet {
                        normal: (0.0, 0.0, -1.0),
                        vertices: [(1.0, -1.0, -1.0), (-1.0, -1.0, -1.0), (-1.0, 1.0, -1.0)],
                    },
                    Facet {
                        normal: (0.0, 0.0, -1.0),
                        vertices: [(-1.0, 1.0, -1.0), (1.0, 1.0, -1.0), (1.0, -1.0, -1.0)],
                    },
                    Facet {
                        normal: (0.0, 0.0, 1.0),
                        vertices: [(1.0, 1.0, 1.0), (-1.0, 1.0, 1.0), (-1.0, -1.0, 1.0)],
                    },
                    Facet {
                        normal: (0.0, 0.0, 1.0),
                        vertices: [(-1.0, -1.0, 1.0), (1.0, -1.0, 1.0), (1.0, 1.0, 1.0)],
                    },
                ]
            }
        );
    }
}
