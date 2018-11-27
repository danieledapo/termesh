use std::f32::consts::PI;
use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::exit;
use std::time;

use structopt::StructOpt;

use termion;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use termesh::drawille::Canvas;
use termesh::dsl;
use termesh::stl::Stl;
use termesh::Vector3;

/// Display 3D objects in the terminal using Braille characters.
#[derive(Debug, StructOpt)]
struct App {
    /// Scale the input mesh by a given factor. If passed disables autoscaling.
    #[structopt(short = "s", long = "scale")]
    scale: Option<f32>,

    /// Rotate the input mesh around the x axis by a given angle in radians
    /// before displaying.
    #[structopt(
        short = "x",
        long = "rotation-x",
        default_value = "0",
        raw(allow_hyphen_values = "true")
    )]
    rotation_x: f32,

    /// Rotate the input mesh around the y axis by a given angle in radians
    /// before displaying.
    #[structopt(
        short = "y",
        long = "rotation-y",
        default_value = "0",
        raw(allow_hyphen_values = "true")
    )]
    rotation_y: f32,

    /// Rotate the input mesh around the z axis by a given angle in radians
    /// before displaying.
    #[structopt(
        short = "z",
        long = "rotation-z",
        default_value = "0",
        raw(allow_hyphen_values = "true")
    )]
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

    /// Input mesh to display. If the extension is `tmesh` then it's assumed
    /// that the mesh is written using the Termesh DSL otherwise it's assumed
    /// it's a binary STL.
    #[structopt(parse(from_os_str))]
    mesh_filepath: PathBuf,
}

trait Scene: Clone {
    fn vertices<'s>(&'s self) -> Box<dyn Iterator<Item = &Vector3> + 's>;
    fn vertices_mut<'s>(&'s mut self) -> Box<dyn Iterator<Item = &mut Vector3> + 's>;
    fn render(&self, canvas: &mut Canvas, only_wireframe: bool);
}

impl Scene for Stl {
    fn vertices<'s>(&'s self) -> Box<dyn Iterator<Item = &Vector3> + 's> {
        Box::new(self.vertices())
    }

    fn vertices_mut<'s>(&'s mut self) -> Box<dyn Iterator<Item = &mut Vector3> + 's> {
        Box::new(self.vertices_mut())
    }

    fn render(&self, canvas: &mut Canvas, only_wireframe: bool) {
        if only_wireframe {
            for f in &self.facets {
                canvas.triangle(f.vertices[0], f.vertices[1], f.vertices[2]);
            }
        } else {
            for f in &self.facets {
                canvas.fill_triangle(f.vertices[0], f.vertices[1], f.vertices[2]);
            }
        }
    }
}

impl<'input> Scene for dsl::ast::Module<'input> {
    fn vertices<'s>(&'s self) -> Box<dyn Iterator<Item = &Vector3> + 's> {
        Box::new(self.vertices())
    }

    fn vertices_mut<'s>(&'s mut self) -> Box<dyn Iterator<Item = &mut Vector3> + 's> {
        Box::new(self.vertices_mut())
    }

    fn render(&self, canvas: &mut Canvas, only_wireframe: bool) {
        let mut env = std::collections::HashMap::new();

        for stmt in &self.statements {
            match stmt.expr {
                termesh::dsl::ast::Expr::Vertex(name, pos) => {
                    env.insert(name, pos);
                }
                termesh::dsl::ast::Expr::Line(v0, v1) => {
                    canvas.line(env[v0], env[v1]);
                }
                termesh::dsl::ast::Expr::Triangle(v0, v1, v2) => {
                    if only_wireframe {
                        canvas.triangle(env[v0], env[v1], env[v2]);
                    } else {
                        canvas.fill_triangle(env[v0], env[v1], env[v2]);
                    }
                }
            }
        }
    }
}

fn main() -> io::Result<()> {
    let app = App::from_args();

    let mut f = File::open(&app.mesh_filepath)?;

    if let Some(ext) = app.mesh_filepath.extension() {
        if ext == std::ffi::OsStr::new("tmesh") {
            let mut buf = String::new();
            f.read_to_string(&mut buf)?;

            match dsl::parse_module(&buf) {
                Ok(prog) => {
                    if let Err(typecheck_err) = dsl::type_check(&prog) {
                        eprintln!();
                        print_dsl_error(typecheck_err, &app.mesh_filepath);
                        exit(1);
                    }

                    if app.non_interactive || !termion::is_tty(&io::stdout()) {
                        non_interactive(app, prog)?;
                    } else {
                        interactive(app, prog)?;
                    }
                }
                Err(parse_error) => {
                    eprintln!();
                    print_dsl_error(parse_error, &app.mesh_filepath);
                    exit(1);
                }
            }

            return Ok(());
        }
    }

    let stl = Stl::parse_binary(&mut f)?;

    if app.non_interactive || !termion::is_tty(&io::stdout()) {
        non_interactive(app, stl)
    } else {
        interactive(app, stl)
    }
}

fn non_interactive<S: Scene>(config: App, mut scene: S) -> io::Result<()> {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    rotate_scene(
        &mut scene,
        config.rotation_x,
        config.rotation_y,
        config.rotation_z,
    );
    scale_scene(&mut scene, config.scale.unwrap_or(1.0));

    render_scene(&mut stdout, &scene, false, None, &config)?;

    Ok(())
}

fn interactive<S: Scene>(mut config: App, scene: S) -> io::Result<()> {
    let mut stdout = io::stdout().into_raw_mode()?;
    write!(stdout, "{}\r\n", termion::cursor::Hide)?;

    let angle_inc = PI / 6.0;

    let mut draw = |c: &App, mut scene| -> io::Result<Vec<String>> {
        let terminal_size = termion::terminal_size()?;

        rotate_scene(&mut scene, c.rotation_x, c.rotation_y, c.rotation_z);

        let padding = 5;
        let scale = c.scale.unwrap_or_else(|| {
            determine_scale_factor(&scene, terminal_size.0 - padding, terminal_size.1 - padding)
        });

        scale_scene(&mut scene, scale);
        render_scene(
            &mut stdout,
            &scene,
            true,
            Some((i32::from(terminal_size.0), i32::from(terminal_size.1))),
            c,
        )
    };

    let mut current_frame = draw(&config, scene.clone())?;

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
            termion::event::Key::Char('w') => {
                config.only_wireframe = !config.only_wireframe;
                true
            }
            termion::event::Key::Char('d') => {
                config.no_depth = !config.no_depth;
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
            current_frame = draw(&config, scene.clone())?;
        }
    }

    reset_screen(&mut stdout)?;

    Ok(())
}

fn render_scene<W: Write, S: Scene>(
    w: &mut W,
    scene: &S,
    clear: bool,
    max_dimensions: Option<(i32, i32)>,
    config: &App,
) -> io::Result<Vec<String>> {
    let mut canvas = Canvas::new();

    scene.render(&mut canvas, config.only_wireframe);

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
        } else {
            write!(
                w,
                "{}{}",
                termion::color::Bg(termion::color::Reset),
                termion::color::Fg(termion::color::Reset)
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

fn rotate_scene<S: Scene>(scene: &mut S, rotation_x: f32, rotation_y: f32, rotation_z: f32) {
    if rotation_x == 0.0 && rotation_y == 0.0 && rotation_z == 0.0 {
        return;
    }

    for v in scene.vertices_mut() {
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

fn scale_scene<S: Scene>(scene: &mut S, scale: f32) {
    if scale == 1.0 {
        return;
    }

    for v in scene.vertices_mut() {
        *v *= scale;
    }
}

fn determine_scale_factor<S: Scene>(scene: &S, max_width: u16, max_height: u16) -> f32 {
    let mut vs = scene.vertices();

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

fn print_dsl_error<T: std::fmt::Display>(err: dsl::ast::Error<T>, filepath: &PathBuf) {
    use termion::color::{Fg, LightCyan, LightRed, Reset};

    let line_no = (err.line_no + 1).to_string();
    let left_padding = line_no.chars().count() + 1;

    eprintln!("{}error{}: {}", Fg(LightRed), Fg(Reset), err.kind);
    eprintln!(
        "{fill:pad$}{}-->{} {}:{}",
        Fg(LightCyan),
        Fg(Reset),
        filepath.display(),
        err.line_no + 1,
        fill = " ",
        pad = left_padding - 1
    );
    eprintln!(
        "{fill:pad$}{}|{}",
        Fg(LightCyan),
        Fg(Reset),
        fill = " ",
        pad = left_padding
    );
    eprintln!("{} {}|{} {}", line_no, Fg(LightCyan), Fg(Reset), err.line,);
    eprintln!(
        "{fill:pad$}{}|{}",
        Fg(LightCyan),
        Fg(Reset),
        fill = " ",
        pad = left_padding
    );
    eprintln!();
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
