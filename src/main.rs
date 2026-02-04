mod estado;
mod grid;
mod pecas;
mod visual;

use std::{
    io::{self, Write},
    time::{Duration, Instant},
};
use termion::{
    async_stdin, clear, cursor, event::Key, input::TermRead, raw::IntoRawMode, terminal_size,
};

use crate::{
    estado::Estado,
    grid::{ALTURA_GRID, LARGURA_GRID},
};

fn main() {
    let mut estado = estado::Estado::new();
    let stdin = async_stdin();
    let mut stdout = io::stdout().into_raw_mode().unwrap();
    let mut keys = stdin.keys();

    let mut delay_tick = 400;
    let delay_render = 50;

    let mut ultimo_tick = Instant::now();
    let mut ultimo_render = Instant::now();

    write!(stdout, "{}{}", cursor::Hide, clear::All).unwrap();
    loop {
        let input = keys.next();
        match input {
            Some(k) => match k {
                Ok(ch) if ch == Key::Char('w') => {
                    estado.derrubar_direto();
                    renderizar(&estado, &mut stdout);
                }
                Ok(ch) if ch == Key::Char('a') => estado.mover(-1),
                Ok(ch) if ch == Key::Char('d') => estado.mover(1),
                Ok(ch) if ch == Key::Char('s') => {
                    estado.tick();
                    renderizar(&estado, &mut stdout);
                }
                Ok(ch) if ch == Key::Char('e') => estado.girar(1),
                Ok(ch) if ch == Key::Char('q') => estado.girar(-1),
                Ok(ch) if ch == Key::Esc || ch == Key::Ctrl('c') => break,
                Ok(ch) if ch == Key::Char('r') => estado = Estado::new(),
                _ => (),
            },
            None => (),
        }

        if ultimo_tick.elapsed() >= Duration::from_millis(delay_tick) {
            ultimo_tick = Instant::now();
            estado.tick();
        }
        if ultimo_render.elapsed() >= Duration::from_millis(delay_render) {
            ultimo_render = Instant::now();
            renderizar(&estado, &mut stdout);
        } else {
            std::thread::sleep(Duration::from_millis(25));
        }
    }
    write!(
        stdout,
        "{}{}{}",
        cursor::Show,
        clear::All,
        cursor::Goto(1, 1)
    )
    .unwrap();
    stdout.flush().unwrap();
}

fn renderizar(estado: &Estado, stdout: &mut termion::raw::RawTerminal<io::Stdout>) {
    let (colunas, linhas) = terminal_size().unwrap();
    let offset_horizontal = colunas / 2 - (LARGURA_GRID + 1) as u16;
    let offset_vertical = linhas / 2 - ((ALTURA_GRID + 1) / 2) as u16;

    write!(stdout, "{}", cursor::Goto(1, offset_vertical)).unwrap();
    write!(stdout, "{}", estado.render(offset_horizontal)).unwrap();
    stdout.flush().unwrap();
}
