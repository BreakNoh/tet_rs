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
    visual::{Celula, Cor, Frame, borda, caixa},
};

enum Comando {
    Fechar,
    Reiniciar,
    Nada,
    ResetarTick,
}

const TAMANHO_BOLSO: usize = 6 * 2;
const TAXA_DIMINUICAO_DELAY: f32 = 0.9;
const DELAY_MINIMO_TICK: Duration = Duration::from_millis(1000);

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

    let (mut renderizador, offset_h, offset_v) = preparar_renderizador(&tema);

    let mut ultimo_nivel = 0;
    let mut delay_tick = Duration::from_millis(1000);
    let delay_render = Duration::from_millis(25);

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

        if ultimo_tick.elapsed() >= delay_tick {
            ultimo_tick = Instant::now();
            estado.tick();

            if estado.nivel != ultimo_nivel {
                ultimo_nivel = estado.nivel;
                delay_tick = delay_tick
                    .mul_f32(TAXA_DIMINUICAO_DELAY)
                    .max(DELAY_MINIMO_TICK);
            }
        }

        if ultimo_render.elapsed() >= delay_render {
            ultimo_render = Instant::now();

            renderizador.desenhar(estado, TAMANHO_BOLSO as isize + 2, 0, false, &tema);
            renderizador.escrever(&format!("nível: {}", estado.nivel), 0, ALTURA_GRID - 8);
            renderizador.escrever("linhas:", 0, ALTURA_GRID - 7);
            renderizador.escrever(&estado.linhas_limpas.to_string(), 0, ALTURA_GRID - 6);
            renderizador.escrever("pontuação:", 0, ALTURA_GRID - 5);
            renderizador.escrever(&estado.pontuacao.to_string(), 0, ALTURA_GRID - 4);

            let mut conteiner_bolso = caixa(TAMANHO_BOLSO, TAMANHO_BOLSO / 2, &tema);
            conteiner_bolso.escrever(" guardada ", 1, 0);

            renderizador.desenhar(conteiner_bolso, 0, 1, false, &tema);

            let mut conteiner_prox = caixa(TAMANHO_BOLSO, TAMANHO_BOLSO / 2, &tema);
            conteiner_prox.escrever(" seguinte ", 1, 0);

            renderizador.desenhar(
                conteiner_prox,
                (LARGURA_GRID * 2 + 4 + TAMANHO_BOLSO) as isize,
                1,
                false,
                &tema,
            );

            if let Some(peca) = estado.peca_guardada {
                let (x, y) = posicinoar_peca(peca.id(), 0, 0);
                renderizador.desenhar(peca, x, y, true, &tema);
            }

            let ori_x = (LARGURA_GRID * 2 + 4 + TAMANHO_BOLSO) as isize;
            let mut ori_y = 0;

            for i in 1..=4 {
                let prox_peca = estado.prox_peca(i);
                let tam_peca = prox_peca.tamanho();
                let (x, y) = posicinoar_peca(prox_peca.id(), ori_x, ori_y);

                if i != 1 {
                    renderizador.desenhar_quadrado(
                        Celula::vazia(),
                        ori_x as usize,
                        (ori_y + 2) as usize,
                        ori_x as usize + 10,
                        ori_y as usize + 5,
                    );
                }
                renderizador.desenhar(prox_peca, x, y, true, &tema);

                ori_y += (if i == 1 {
                    TAMANHO_BOLSO / 2 - 1
                } else {
                    TAMANHO_BOLSO / 3
                }) as isize;
            }

            renderizador.renderizar(&mut stdout, offset_h, offset_v);
        } else {
            std::thread::sleep(Duration::from_millis(50));
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

fn posicinoar_peca(id_peca: u8, origem_x: isize, origem_y: isize) -> (isize, isize) {
    let (x, y) = match id_peca {
        7 => (2, 2),
        6 => (4, 3),
        _ => (3, 3),
    };
    (x + origem_x, y + origem_y)
}

fn preparar_renderizador(tema: &Tema) -> (Frame, u16, u16) {
    let (largura_grid, altura_grid) = ((LARGURA_GRID + 1) * 2, ALTURA_GRID + 1);
    let (largura_term, altura_term) = terminal_size().unwrap();

    let offset_h = (largura_term - largura_grid as u16) / 2 - TAMANHO_BOLSO as u16;
    let offset_v = (altura_term - altura_grid as u16) / 2;

    let mut frame = Frame::new(largura_grid + 2 + TAMANHO_BOLSO * 2, altura_grid, None);

    frame.desenhar_celulas(borda(tema), TAMANHO_BOLSO as isize + 1, 0, false);

    (frame, offset_h, offset_v)
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
