mod estado;
mod grid;
mod pecas;
mod tema;
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
    tema::Tema,
    visual::{Celula, Cor, Frame, borda},
};

enum Comando {
    Fechar,
    Reiniciar,
    Nada,
    ResetarTick,
}

const TAMANHO_BOLSO: usize = 6 * 2;

fn main() {
    let tema = match Tema::carregar("./temas/padrao.toml") {
        Ok(t) => t,
        Err(m) => {
            println!("{m}");
            panic!()
        }
    };

    let mut estado = Estado::new();

    let stdin = async_stdin();
    let mut stdout = io::stdout().into_raw_mode().unwrap();
    let mut keys = stdin.keys();

    let mut renderizador = preparar_renderizador(&tema);

    let mut delay_tick = 400;
    let delay_render = 50;

    let mut ultimo_tick = Instant::now();
    let mut ultimo_render = Instant::now();
    let mut comando = Comando::Nada;

    write!(stdout, "{}{}", cursor::Hide, clear::All).unwrap();

    loop {
        let input = keys.next();
        match input {
            Some(k) => match k {
                Ok(ch) => comando = tratar_input(ch, &mut estado),
                Err(_) => (),
            },
            None => (),
        }

        if ultimo_tick.elapsed() >= Duration::from_millis(delay_tick) {
            ultimo_tick = Instant::now();
            estado.tick();
        }
        if ultimo_render.elapsed() >= Duration::from_millis(delay_render) {
            ultimo_render = Instant::now();

            renderizador.desenhar(estado, TAMANHO_BOLSO as isize + 2, 0, false, &tema);
            renderizador.renderizar(&mut stdout, 1);
        } else {
            std::thread::sleep(Duration::from_millis(25));
        }

        match comando {
            Comando::Fechar => break,
            Comando::Reiniciar => {
                estado = Estado::new();
                comando = Comando::Nada
            }
            Comando::Nada => (),
            Comando::ResetarTick => {
                ultimo_tick = Instant::now();
                comando = Comando::Nada;
            }
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

fn preparar_renderizador(tema: &Tema) -> Frame {
    let (largura_grid, altura_grid) = ((LARGURA_GRID + 1) * 2, ALTURA_GRID + 1);
    let fundo = Celula {
        transparente: false,
        ch: 'X',
        bg: Cor::Magenta,
        fg: Cor::Preto,
    };
    let teste = Celula {
        transparente: false,
        ch: 'O',
        bg: Cor::Verde,
        fg: Cor::Vermelho,
    };

    let mut frame = Frame::new(largura_grid + 1 + TAMANHO_BOLSO, altura_grid, None);

    frame.desenhar_celulas(borda(tema), TAMANHO_BOLSO as isize + 1, 0, false);
    frame.desenhar_celula(teste, largura_grid + TAMANHO_BOLSO, altura_grid);

    frame
}

fn tratar_input(ch: Key, estado: &mut Estado) -> Comando {
    match ch {
        Key::Char('w') => {
            estado.derrubar_direto();
        }
        Key::Char('f') => {
            estado.guardar_peca();
        }
        Key::Char('a') => estado.mover(-1),
        Key::Char('d') => estado.mover(1),
        Key::Char('s') => {
            estado.cair();
            return Comando::ResetarTick;
        }
        Key::Char('e') => estado.girar(1),
        Key::Char('q') => estado.girar(-1),
        Key::Esc | Key::Ctrl('c') => return Comando::Fechar,
        Key::Char('r') => return Comando::Reiniciar,
        _ => (),
    };

    Comando::Nada
}
