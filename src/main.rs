use clap::{App, Arg};
use raqote::*;
use std::fs;
use std::io;
use std::path::Path;

fn main() -> Result<(), io::Error> {
    let matches = App::new("icon-gen")
        .version("1.0")
        .author("ogata-k. <ogtkzk712@gmail.com>")
        .about("my icon generator. but png only.")
        .arg(
            Arg::with_name("size")
                .short("s")
                .long("size")
                .default_value("2000")
                .help("image size")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .default_value("icon.png")
                .help("Sets the input file to use"),
        )
        .get_matches();
    let size: u16 = matches
        .value_of("size")
        .expect("不正なサイズです。")
        .parse()
        .expect("fail parse size");
    let path: &str = matches.value_of("file").expect("不正なファイルパスです。");

    run(path, size)
}

fn run<P: AsRef<std::path::Path>>(path: P, size: u16) -> Result<(), io::Error> {
    let path: &Path = path.as_ref();
    let mut split_path: Vec<String> = path
        .to_string_lossy()
        .split("/")
        .map(|s| s.to_string())
        .collect();
    split_path
        .pop()
        .expect("ファイル名を取得できませんでした。");
    let dir_path: String = split_path.join("/");
    fs::create_dir_all(&dir_path)?;

    Ok(create_icon(size.into()).write_png(&path)?)
}

fn create_icon(size: u16) -> DrawTarget {
    let width: u16 = size;
    let height: u16 = size;
    let mut dt = DrawTarget::new(width.into(), height.into());

    // back ground
    let mut pb = PathBuilder::new();
    pb.rect(0., 0., width.into(), height.into());
    dt.fill(
        &pb.finish(),
        &Source::Solid(SolidSource::from_unpremultiplied_argb(255, 250, 250, 250)),
        &DrawOptions::new(),
    );

    // outline
    let lup: Point = Point::new(width as f32 / 10., 2. * (height as f32 / 9.));
    let view_size: Point = Point::new(7. * (width as f32 / 10.), 2. * (height as f32 / 5.));
    let rbp: Point = Point::new(lup.x + view_size.x, lup.y + view_size.y);
    let mut pb = PathBuilder::new();
    pb.move_to((5. * lup.x + rbp.x) / 6., 99. * (rbp.y / 100.));
    pb.quad_to(
        (9. * lup.x + rbp.x) / 10.,
        49. * (rbp.y / 50.),
        lup.x,
        rbp.y,
    );
    pb.line_to(lup.x, 99. * (rbp.y / 100.));
    pb.cubic_to(
        0.4 * width as f32,
        0.9 * lup.y,
        1.55 * rbp.x,
        -0.05 * lup.y,
        (3. * lup.x + 5. * rbp.x) / 8.,
        101. * (rbp.y / 100.),
    );
    dt.stroke(
        &pb.finish(),
        &Source::Solid(SolidSource::from_unpremultiplied_argb(255, 0, 0, 0)),
        &StrokeStyle {
            width: 0.01 * size as f32,
            cap: LineCap::Round,
            ..StrokeStyle::default()
        },
        &DrawOptions::new(),
    );

    // inner ellipse
    let mut pb = PathBuilder::new();
    let mid_x = (5. * lup.x + 4. * rbp.x) / 9.;
    let from_y = (5. * lup.y + 7. * rbp.y) / 12.;
    pb.move_to(mid_x, from_y);
    pb.line_to(mid_x, 1.02 * from_y);
    dt.stroke(
        &pb.finish(),
        &Source::Solid(SolidSource::from_unpremultiplied_argb(255, 0, 0, 0)),
        &StrokeStyle {
            width: 0.06 * size as f32,
            cap: LineCap::Round,
            ..StrokeStyle::default()
        },
        &DrawOptions::new(),
    );

    return dt;
}
