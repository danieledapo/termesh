use std::f32::consts::PI;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::PathBuf;
use std::time;

use structopt::StructOpt;

use termion;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use termesh::drawille::Canvas;
use termesh::stl::Stl;

/// Display 3D objects in the terminal using Braille characters.
#[derive(Debug, StructOpt)]
struct App {
    /// Scale the input mesh by a given factor. If passed disables autoscaling.
    #[structopt(short = "s", long = "scale")]
    scale: Option<f32>,

    /// Rotate the input mesh around the x axis by a given angle in radians
    /// before displaying.
    #[structopt(short = "x", long = "rotation-x", default_value = "0")]
    rotation_x: f32,

    /// Rotate the input mesh around the y axis by a given angle in radians
    /// before displaying.
    #[structopt(short = "y", long = "rotation-y", default_value = "0")]
    rotation_y: f32,

    /// Rotate the input mesh around the z axis by a given angle in radians
    /// before displaying.
    #[structopt(short = "z", long = "rotation-z", default_value = "0")]
    rotation_z: f32,

    /// Do not render using true colors. This will effectively make the depth
    /// all the same.
    #[structopt(long = "no-depth")]
    no_depth: bool,

    /// Display only the wireframe of the mesh.
    #[structopt(short = "w", long = "wireframe")]
    only_wireframe: bool,

    /// Display a mesh and exit.
    #[structopt(long = "non-interactive")]
    non_interactive: bool,

    /// Input mesh to display. Only binary STL as of now.
    #[structopt(parse(from_os_str))]
    mesh_filepath: PathBuf,
}

fn main() -> io::Result<()> {
    let app = App::from_args();

    let mut f = File::open(&app.mesh_filepath)?;
    let stl = Stl::parse_binary(&mut f)?;

    if app.non_interactive || !termion::is_tty(&io::stdout()) {
        non_interactive(app, stl)
    } else {
        interactive(app, stl)
    }
}

fn non_interactive(config: App, mut stl: Stl) -> io::Result<()> {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    rotate_stl(
        &mut stl,
        config.rotation_x,
        config.rotation_y,
        config.rotation_z,
    );
    scale_stl(&mut stl, config.scale.unwrap_or(1.0));

    render_stl(&mut stdout, &stl, false, None, &config)?;

    Ok(())
}

fn interactive(mut config: App, stl: Stl) -> io::Result<()> {
    let mut stdout = io::stdout().into_raw_mode()?;
    write!(stdout, "{}\r\n", termion::cursor::Hide)?;

    let angle_inc = PI / 6.0;

    let mut draw = |c: &App, mut stl| -> io::Result<Vec<String>> {
        let terminal_size = termion::terminal_size()?;

        rotate_stl(&mut stl, c.rotation_x, c.rotation_y, c.rotation_z);

        let padding = 5;
        let scale = c.scale.unwrap_or_else(|| {
            determine_scale_factor(&stl, terminal_size.0 - padding, terminal_size.1 - padding)
        });

        scale_stl(&mut stl, scale);
        render_stl(
            &mut stdout,
            &stl,
            true,
            Some((i32::from(terminal_size.0), i32::from(terminal_size.1))),
            c,
        )
    };

    let mut current_frame = draw(&config, stl.clone())?;

    for ev in io::stdin().keys() {
        let ev = ev?;

        let redraw = match ev {
            termion::event::Key::Char('q') => break,
            termion::event::Key::Char('x') => {
                config.rotation_x = (config.rotation_x + angle_inc) % (2.0 * PI);
                true
            }
            termion::event::Key::Char('X') => {
                config.rotation_x = (config.rotation_x - angle_inc) % (2.0 * PI);
                true
            }
            termion::event::Key::Char('y') => {
                config.rotation_y = (config.rotation_y + angle_inc) % (2.0 * PI);
                true
            }
            termion::event::Key::Char('Y') => {
                config.rotation_y = (config.rotation_y - angle_inc) % (2.0 * PI);
                true
            }
            termion::event::Key::Char('z') => {
                config.rotation_z = (config.rotation_z + angle_inc) % (2.0 * PI);
                true
            }
            termion::event::Key::Char('Z') => {
                config.rotation_z = (config.rotation_z - angle_inc) % (2.0 * PI);
                true
            }
            termion::event::Key::Char('s') => {
                if let Err(err) = save_frame(&config, &current_frame) {
                    reset_screen(&mut stdout)?;
                    return Err(err);
                }

                false
            }
            _ => continue,
        };

        if redraw {
            current_frame = draw(&config, stl.clone())?;
        }
    }

    reset_screen(&mut stdout)?;

    Ok(())
}

fn render_stl<W: Write>(
    w: &mut W,
    stl: &Stl,
    clear: bool,
    max_dimensions: Option<(i32, i32)>,
    config: &App,
) -> io::Result<Vec<String>> {
    let mut canvas = Canvas::new();

    if config.only_wireframe {
        for f in &stl.facets {
            canvas.triangle(f.vertices[0], f.vertices[1], f.vertices[2]);
        }
    } else {
        for f in &stl.facets {
            canvas.fill_triangle(f.vertices[0], f.vertices[1], f.vertices[2]);
        }
    }

    // callers can clear the screen by themselves, but it usually causes
    // flickering on big terminals. Therefore defer clearing the screen until
    // the very last.
    if clear {
        // changing the background color needs clearing before it can be
        // rendered effectively
        if !config.no_depth {
            write!(
                w,
                "{}",
                termion::color::Bg(termesh::drawille::Canvas::background_color())
            )?;
        }
        clear_screen(w)?;
    }

    let frame = match max_dimensions {
        None => {
            let frame = canvas.rows(!config.no_depth).collect::<Vec<_>>();

            for r in &frame {
                write!(w, "{}\r\n", r)?;
            }
            w.flush()?;

            frame
        }
        Some((max_width, max_height)) => {
            if let Some((min_r, max_r, min_c, max_c)) = canvas.dimensions() {
                let padded = |min, max, max_len| {
                    if max_len <= max - min {
                        min
                    } else {
                        let padding = max_len - (max - min);
                        min - padding / 2
                    }
                };

                let min_r = padded(min_r, max_r, max_height);
                let min_c = padded(min_c, max_c, max_width);

                let frame = canvas
                    .frame(!config.no_depth, min_r, max_r, min_c, Some(max_c))
                    .collect::<Vec<_>>();

                for r in &frame {
                    write!(w, "{}\r\n", r)?;
                }
                w.flush()?;

                frame
            } else {
                vec![]
            }
        }
    };

    Ok(frame)
}

fn rotate_stl(stl: &mut Stl, rotation_x: f32, rotation_y: f32, rotation_z: f32) {
    if rotation_x == 0.0 && rotation_y == 0.0 && rotation_z == 0.0 {
        return;
    }

    for v in stl.vertices_mut() {
        if rotation_x != 0.0 {
            v.rotate_x(rotation_x);
        }

        if rotation_y != 0.0 {
            v.rotate_y(rotation_y);
        }

        if rotation_z != 0.0 {
            v.rotate_z(rotation_z);
        }
    }
}

fn scale_stl(stl: &mut Stl, scale: f32) {
    if scale == 1.0 {
        return;
    }

    for v in stl.vertices_mut() {
        *v *= scale;
    }
}

fn determine_scale_factor(stl: &Stl, max_width: u16, max_height: u16) -> f32 {
    let mut vs = stl.vertices();

    let (w, h) = vs
        .next()
        .map(|v| {
            vs.fold((v.x, v.y, v.x, v.y), |(min_x, min_y, max_x, max_y), v| {
                (
                    min_x.min(v.x),
                    min_y.min(v.y),
                    max_x.max(v.x),
                    max_y.max(v.y),
                )
            })
        })
        .map_or((1.0, 1.0), |(min_x, min_y, max_x, max_y)| {
            (max_x - min_x, max_y - min_y)
        });

    let scalex = f32::from(max_width) / w * 2.0;
    let scaley = f32::from(max_height) / h * 4.0;

    scalex.min(scaley)
}

fn save_frame(config: &App, frame: &[String]) -> io::Result<()> {
    let mut out = File::create(&format!(
        "{}-{}.txt",
        config
            .mesh_filepath
            .file_stem()
            .unwrap_or(std::ffi::OsStr::new(""))
            .to_string_lossy(),
        time::SystemTime::now()
            .duration_since(time::SystemTime::UNIX_EPOCH)
            .expect("clock drift")
            .as_secs()
    ))?;

    for row in frame {
        write!(out, "{}\r\n", row)?;
    }

    Ok(())
}

fn clear_screen<W: Write>(w: &mut W) -> io::Result<()> {
    write!(
        w,
        "{}{}\r\n",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
    )?;
    w.flush()?;

    Ok(())
}

fn reset_screen<W: Write>(w: &mut W) -> io::Result<()> {
    write!(
        w,
        "{}{}{}{}\r\n",
        termion::color::Bg(termion::color::Reset),
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        termion::cursor::Show
    )
}
