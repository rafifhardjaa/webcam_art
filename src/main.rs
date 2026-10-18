use crossterm::{
    cursor,
    execute,
    terminal,
};
use opencv::{
    core::Size,
    imgproc,
    prelude::*,
    videoio::{VideoCapture, CAP_ANY},
};
use std::{
    io::{self, BufWriter, Write},
    time::Duration,
};

const ASCII_CHARS: &[u8] = b" .,:;i1tfLCG08@";

fn brightness_to_char(value: u8) -> u8 {
    let index = (value as usize * (ASCII_CHARS.len() - 1)) / 255;
    ASCII_CHARS[index]
}

fn process_frame(frame: &Mat, cols: u16, rows: u16, output: &mut Vec<u8>) {
    let target_w = cols as i32;
    let target_h = rows as i32;

    // Flip horizontal (1 = flip sumbu Y) untuk koreksi mirror webcam
    let mut flipped = Mat::default();
    opencv::core::flip(frame, &mut flipped, 1).unwrap();

    let mut gray = Mat::default();
    imgproc::cvt_color(&flipped, &mut gray, imgproc::COLOR_BGR2GRAY, 0).unwrap();

    let mut resized = Mat::default();
    imgproc::resize(
        &gray,
        &mut resized,
        Size::new(target_w, target_h),
        0.0,
        0.0,
        imgproc::INTER_AREA,
    )
    .unwrap();

    let mut eq = Mat::default();
    imgproc::equalize_hist(&resized, &mut eq).unwrap();

    output.clear();
    output.reserve((target_w * target_h + target_h) as usize);

    for y in 0..target_h {
        for x in 0..target_w {
            let lum: &u8 = eq.at_2d(y, x).unwrap();
            output.push(brightness_to_char(*lum));
        }
        if y < target_h - 1 {
            output.push(b'\n');
        }
    }
}

fn main() -> opencv::Result<()> {
    let mut camera = VideoCapture::new(0, CAP_ANY)?;
    let mut frame = Mat::default();

    let stdout = io::stdout();
    let mut out = BufWriter::with_capacity(1024 * 1024, stdout.lock());

    // Reset terminal ke default, background hitam, foreground putih
    out.write_all(b"\x1b[0m\x1b[2J\x1b[H").ok();
    execute!(out, cursor::Hide).ok();

    let mut pixel_buf: Vec<u8> = Vec::with_capacity(1024 * 1024);

    loop {
        camera.read(&mut frame)?;

        if frame.empty() {
            continue;
        }

        let (term_cols, term_rows) = terminal::size().unwrap_or((80, 24));
        let render_rows = term_rows.saturating_sub(1);

        process_frame(&frame, term_cols, render_rows, &mut pixel_buf);

        // Kembali ke pojok kiri atas
        out.write_all(b"\x1b[H").ok();
        out.write_all(&pixel_buf).ok();
        out.flush().ok();

        std::thread::sleep(Duration::from_millis(50));
    }
}
