use {
    palette::{FromColor, Hsv, ShiftHue, Srgb},
    std::{
        fs::File,
        io,
        os::unix::fs::FileExt,
        thread,
        time::{Duration, Instant},
    },
};

fn config_file() -> File {
    dirs::config_dir()
        .map(|dir| dir.join("alacritty/alacritty.yml"))
        .and_then(|config| File::options().write(true).truncate(true).open(config).ok())
        .unwrap_or_else(|| panic!("create alacritty.yml yourself"))
}

fn rainbow_rgb(offset: f32) -> String {
    let hsv_u8 = Hsv::new_srgb(255_u8, 255, 150);
    let hsv_f32 = hsv_u8.into_format();
    let srgb = Srgb::from_color(hsv_f32.shift_hue(offset));
    let [r, g, b]: [u8; 3] = srgb.into_format().into();

    format!("#{r:02X}{g:02X}{b:02X}")
}

fn main() -> io::Result<()> {
    let time = Instant::now();
    let config = config_file();

    loop {
        let rgb = rainbow_rgb(((time.elapsed().as_secs_f32() * 60.0) % 360.0) as f32);
        let buffer = format!(
            "
colors:
  primary:
    background: \"#ffffff\"
    foreground: \"{rgb}\"

  normal:
    black:   \"#ffffff\"
    red:     \"{rgb}\"
    green:   \"{rgb}\"
    yellow:  \"{rgb}\"
    blue:    \"{rgb}\"
    magenta: \"{rgb}\"
    cyan:    \"{rgb}\"
    white:   \"#909090\"

  bright:
    black:    \"#909090\"
    red:      \"{rgb}\"
    green:    \"{rgb}\"
    yellow:   \"{rgb}\"
    blue:     \"{rgb}\"
    magenta:  \"{rgb}\"
    cyan:     \"{rgb}\"
    white:    \"#909090\"

cursor:
  blinking: Never
  style: Beam
  unfocused_hollow: false
  thickness: 0.1

font:
  normal:
    family: 'roboto mono'
    style: 'demibold'

  bold:
    family: 'roboto mono'
    style: 'heavy'

  italic:
    family: 'roboto mono'
    style: 'heavy italic'

  bold_italic:
    family: 'roboto mono'
    style: 'heavy italic'

  builtin_box_drawing: true
  size: 11.5

scrolling:
  history: 5000
  multiplier: 5

selection:
  save_to_clipboard: true

window:
  dimensions:
    columns: 72
    lines: 24

  opacity: 0.9

  padding:
    x: 12
    y: 12

  position:
    x: 955
    y: 545
"
        );
        config.write_at(buffer.as_bytes(), 0)?;
        config.sync_all()?;

        thread::sleep(Duration::from_millis(50));
    }
}
