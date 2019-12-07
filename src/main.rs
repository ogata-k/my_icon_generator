use raqote::*;

fn main() {
    let _ = create_icon(3000).write_png("icon.png");
}

fn create_icon(size: i16) -> DrawTarget {
    let width: i16 = size;
    let height : i16 = size;
    let mut dt = DrawTarget::new(width.into(), height.into());

    // back ground
    let mut pb = PathBuilder::new();
    pb.rect(0., 0., width.into(), height.into());
    dt.fill(
        &pb.finish(),
        &Source::Solid(SolidSource::from_unpremultiplied_argb(255, 250, 250, 250)),
            &DrawOptions::new()
    );

    // outline
    let lup: Point = Point::new(width as f32 / 10., 2. * (height as f32 / 9.));
    let view_size: Point = Point::new(7. * (width as f32 / 10.), 2. * (height as f32 / 5.));
    let rbp : Point = Point::new(lup.x + view_size.x, lup.y + view_size.y);
    let mut pb = PathBuilder::new();
    pb.move_to((5. * lup.x + rbp.x) / 6. , 99. * (rbp.y / 100.));
    pb.quad_to((9. * lup.x + rbp.x) / 10.,49. * (rbp.y / 50.), lup.x, rbp.y);
    pb.line_to(lup.x, 99. * (rbp.y / 100.));
    pb.cubic_to(
        0.4 * width as f32, 0.9 * lup.y,
        1.55 * rbp.x, -0.05 * lup.y,
        (3. * lup.x + 5. * rbp.x) / 8., 101. * (rbp.y / 100.)
    );
    dt.stroke(
        &pb.finish(),
        &Source::Solid(SolidSource::from_unpremultiplied_argb(255, 0, 0, 0)),
        &StrokeStyle{
            width: 0.01 * size as f32,
            cap: LineCap::Round,
            ..StrokeStyle::default()
        },
        &DrawOptions::new()
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
      &StrokeStyle{
          width: 0.06 * size as f32,
          cap: LineCap::Round,
          ..StrokeStyle::default()
      },
      &DrawOptions::new()
    );

    return dt;
}