use std::env;
use std::f32::consts::PI;
use std::fs::File;
use std::io;
use std::io::Write;

use termion;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use termesh::drawille::Canvas;
use termesh::stl::Stl;

fn main() -> io::Result<()> {
    let stl_filepath = env::args().skip(1).next().unwrap();
    let mut f = File::open(stl_filepath)?;
    let mut stl = Stl::parse_binary(&mut f)?;

    let mut stdout = io::stdout().into_raw_mode()?;
    write!(
        stdout,
        "{}{}\r\n",
        termion::cursor::Save,
        termion::cursor::Hide
    )?;

    let mut canvas = Canvas::new();

    // TODO: automatically scale on startup according to terminal size
    for v in stl.vertices_mut() {
        v.scale(40.0);
    }

    update_canvas(&mut stdout, &stl, &mut canvas)?;

    let angle_inc = PI / 4.0;

    let mut angles = [0.0, 0.0, 0.0];

    for ev in io::stdin().keys() {
        let ev = ev?;

        match ev {
            termion::event::Key::Char('q') => break,
            termion::event::Key::Char('x') => {
                angles[0] = (angles[0] + angle_inc) % (2.0 * PI);
            }
            termion::event::Key::Char('y') => {
                angles[1] = (angles[1] + angle_inc) % (2.0 * PI);
            }
            termion::event::Key::Char('z') => {
                angles[2] = (angles[2] + angle_inc) % (2.0 * PI);
            }
            _ => continue,
        }

        let mut stl = stl.clone();

        for v in stl.vertices_mut() {
            v.rotate_x(angles[0]);
            v.rotate_y(angles[1]);
            v.rotate_z(angles[2]);
        }

        update_canvas(&mut stdout, &stl, &mut canvas)?;
    }

    write!(
        stdout,
        "{}{}{}\r\n",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        termion::cursor::Show
    )?;

    Ok(())
}

fn update_canvas<W: Write>(w: &mut W, stl: &Stl, canvas: &mut Canvas) -> io::Result<()> {
    for f in &stl.facets {
        canvas.triangle(
            f.vertices[0].x,
            f.vertices[0].y,
            f.vertices[1].x,
            f.vertices[1].y,
            f.vertices[2].x,
            f.vertices[2].y,
        );
    }

    write!(
        w,
        "{}{}\r\n",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
    )?;
    w.flush()?;

    for r in canvas.rows() {
        write!(w, "{}\r\n", r)?;
    }
    w.flush()?;

    canvas.clear();

    Ok(())
}
