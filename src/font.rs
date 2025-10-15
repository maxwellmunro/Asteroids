use crate::constants;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Canvas;
use sdl2::video::Window;

pub fn render_text(
    text: &str,
    mut x: i32,
    y: i32,
    canvas: &mut Canvas<Window>,
) -> Result<(), String> {
    canvas.set_draw_color(Color::RGB(255, 255, 255));

    text.as_bytes().iter().try_for_each(|&c| {
        if c >= '0' as u8 && c <= '9' as u8 {
            let idx = c as u32 - '0' as u32;

            render_path(constants::font::NUMBERS[idx as usize], x, y, canvas)?;

        } else if c >= 'a' as u8 && c <= 'z' as u8 {
            let idx = c as u32 - 'a' as u32;

            render_path(constants::font::LETTERS[idx as usize], x, y, canvas)?;
        }

        x += (constants::font::FONT_SIZE + constants::font::MARGIN) as i32;
        Ok::<(), String>(())
    })?;

    Ok(())
}

fn render_path(
    path: &[[(f32, f32); 2]],
    x: i32,
    y: i32,
    canvas: &mut Canvas<Window>,
) -> Result<(), String> {
    path.iter().try_for_each(|l| {
        let p1 = Point::new(
            (l[0].0 * constants::font::FONT_SIZE as f32) as i32 + x,
            (l[0].1 * constants::font::FONT_SIZE as f32) as i32 + y,
        );
        let p2 = Point::new(
            (l[1].0 * constants::font::FONT_SIZE as f32) as i32 + x,
            (l[1].1 * constants::font::FONT_SIZE as f32) as i32 + y,
        );

        canvas.draw_line(p1, p2)?;

        Ok::<(), String>(())
    })?;

    Ok(())
}

pub fn render_lives(lives: u32, screen_bounds: &Rect, canvas: &mut Canvas<Window>) -> Result<(), String> {
    for i in 0..(lives as i32) {
        let mid = screen_bounds.width() / 2;
        let dx = (i as f32 - (lives as f32 / 2.0)) * (constants::font::FONT_SIZE + constants::font::MARGIN) as f32;
        render_life_char(mid as i32 + dx as i32, 10, canvas)?;
    }

    Ok(())
}

fn render_life_char(x: i32, y: i32, canvas: &mut Canvas<Window>) -> Result<(), String> {
    constants::font::LIFE_CHAR.iter().try_for_each(|l| {
        let p1 = Point::new(
            (l[0].0 * constants::font::FONT_SIZE as f32) as i32 + x,
            (l[0].1 * constants::font::FONT_SIZE as f32) as i32 + y,
        );
        let p2 = Point::new(
            (l[1].0 * constants::font::FONT_SIZE as f32) as i32 + x,
            (l[1].1 * constants::font::FONT_SIZE as f32) as i32 + y,
        );

        canvas.draw_line(p1, p2)?;

        Ok::<(), String>(())
    })?;

    Ok(())
}
