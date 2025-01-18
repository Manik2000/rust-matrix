use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{poll, read, Event, KeyCode},
    execute,
    style::{Color, Print, SetForegroundColor},
    terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode, size},
    QueueableCommand,
};
use rand::Rng;
use std::io::{stdout, Write, Result};
use std::time::Duration;


struct Drop {
    x: u16,
    y: i32,
    speed: u16,
    length: u16,
}

impl Drop {
    fn new(x: u16, length: u16) -> Self {
        Self {
            x,
            y: -(length as i32),
            speed: rand::thread_rng().gen_range(1..=3),
            length,
        }
    }

    fn update(&mut self, height: u16) {
        self.y += self.speed as i32;
        if self.y > height as i32 {
            self.y = -(self.length as i32);
        }
    }
}

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, Hide)?;

    let (width, height) = size()?;

    let mut drops: Vec<Drop> = (0..width)
        .map(|x| Drop::new(x, rand::thread_rng().gen_range(5..15)))
        .collect();

    let chars: Vec<char> = "ｱｲｳｴｵｶｷｸｹｺｻｼｽｾｿﾀﾁﾂﾃﾄﾅﾆﾇﾈﾉﾊﾋﾌﾍﾎﾏﾐﾑﾒﾓﾔﾕﾖﾗﾘﾙﾚﾛﾜﾝ1234567890"
        .chars()
        .collect();

    let chars_length: usize = chars.len();

    loop {
        if poll(Duration::from_millis(50))? {
            if let Event::Key(key) = read()? {
                if key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }

        stdout.queue(Clear(ClearType::All))?;

        for drop in drops.iter_mut() {
            drop.update(height);

            for i in 0..drop.length {
                let y = drop.y - i as i32;
                if y >= 0 && y < height as i32 {
                    let intensity = 255 - (i * 255 / drop.length) as u8;
                    stdout
                        .queue(MoveTo(drop.x, y as u16))?
                        .queue(SetForegroundColor(Color::Rgb { 
                            r: 0, 
                            g: intensity, 
                            b: 0 
                        }))?
                        .queue(Print(
                            chars[rand::thread_rng().gen_range(0..chars_length)].to_string(),
                        ))?;
                }
            }
        }

        stdout.flush()?;
    }

    execute!(stdout, Show)?;
    disable_raw_mode()?;
    Ok(())
}
