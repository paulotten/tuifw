#![windows_subsystem = "windows"]
#![deny(warnings)]

use std::cmp::{min, max};
use tuifw_screen::{Screen, Point, Color, Attr, Vector, Event, Key};

fn draw_box(screen: &mut dyn Screen, p: &mut Point) {
    p.y = min(max(p.y - 4, 0) + 8, screen.size().y - 1) - 4;
    if p.y < 0 { return; }
    let w = 0 .. screen.size().x;
    screen.out(p.offset(Vector { x: -6, y: -4 }), Color::Green, None, Attr::empty(), "             ", w.clone(), w.clone());
    screen.out(p.offset(Vector { x: -7, y: -3 }), Color::Green, None, Attr::empty(), " ╔═══════════╗ ", w.clone(), w.clone());
    screen.out(p.offset(Vector { x: -7, y: -2 }), Color::Green, None, Attr::empty(), " ║     ↑     ║ ", w.clone(), w.clone());
    screen.out(p.offset(Vector { x: -7, y: -1 }), Color::Green, None, Attr::empty(), " ║     k     ║ ", w.clone(), w.clone());
    screen.out(p.offset(Vector { x: -7, y: 0 }), Color::Green, None, Attr::empty(), " ║ ←h     l→ ║ ", w.clone(), w.clone());
    screen.out(p.offset(Vector { x: -7, y: 1 }), Color::Green, None, Attr::empty(), " ║     j     ║ ", w.clone(), w.clone());
    screen.out(p.offset(Vector { x: -7, y: 2 }), Color::Green, None, Attr::empty(), " ║     ↓     ║ ", w.clone(), w.clone());
    screen.out(p.offset(Vector { x: -7, y: 3 }), Color::Green, None, Attr::empty(), " ╚═══════════╝ ", w.clone(), w.clone());
    screen.out(p.offset(Vector { x: -6, y: 4 }), Color::Green, None, Attr::empty(), "             ", w.clone(), w.clone());
}

fn main() {
    let mut screen = unsafe { tuifw_screen::init() }.unwrap();
    let screen = screen.as_mut();
    let mut p = Point { x: screen.size().x / 2, y: screen.size().y / 2 };
    draw_box(screen, &mut p);
    loop {
        if let Some(e) = screen.update(None, true).unwrap() {
            if let Some((n, d)) = match e {
                Event::Key(n, Key::Left) => Some((n, Vector { x: -1, y: 0 })),
                Event::Key(n, Key::Right) => Some((n, Vector { x: 1, y: 0 })),
                Event::Key(n, Key::Up) => Some((n, Vector { x: 0, y: -1 })),
                Event::Key(n, Key::Down) => Some((n, Vector { x: 0, y: 1 })),
                Event::Key(n, Key::Char('h')) => Some((n, Vector { x: -1, y: 0 })),
                Event::Key(n, Key::Char('l')) => Some((n, Vector { x: 1, y: 0 })),
                Event::Key(n, Key::Char('k')) => Some((n, Vector { x: 0, y: -1 })),
                Event::Key(n, Key::Char('j')) => Some((n, Vector { x: 0, y: 1 })),
                Event::Key(_, Key::Escape) => break,
                _ => None,
            } {
                for _ in 0 .. n.get() {
                    p = p.offset(d);
                    draw_box(screen, &mut p);
                }
            }
        }
    }
}
