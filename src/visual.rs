use crate::grid::LARGURA_GRID;

use super::{grid::Grid, pecas::WrapperPeca};
use termion::{color::*, style};

const ESQUERA_BLOCO: char = '\u{1FB34}'; // 🬴 
const DIREITA_BLOCO: char = '\u{1FB38}'; // 🬸 

const PAREDE_BORDA: char = '\u{2551}'; // ║
const BASE_BORDA: char = '\u{2550}'; // ═
const CANTO_ESQUERDO_BORDA: char = '\u{255a}'; // ╚
const CANTO_DIREITO_BORDA: char = '\u{255d}'; // ╝ 

fn paleta(id: u8) -> String {
    match id {
        1 => format!("{}{}", Bg(LightMagenta), Fg(Magenta)), // T
        2 => format!("{}{}", Bg(LightBlue), Fg(Blue)),       // LE
        3 => format!("{}{}", Bg(LightRed), Fg(Red)),         // SE
        4 => format!("{}{}", Bg(LightRed), Fg(Yellow)),      // LD
        5 => format!("{}{}", Bg(LightGreen), Fg(Green)),     // SD
        6 => format!("{}{}", Bg(LightYellow), Fg(Yellow)),   // O
        7 => format!("{}{}", Bg(LightCyan), Fg(Cyan)),       // I
        99 => format!("{}{}", Bg(White), Fg(LightBlack)),    // fantasma
        _ => String::from(""),
    }
}

pub fn renderizar(
    mut grid: Grid,
    peca_segurada: Option<WrapperPeca>,
    x: isize,
    y: isize,
    offset_horizontal: u16,
    fantasma: Option<(WrapperPeca, isize, isize)>,
) -> String {
    let mut render = String::new();

    if let Some((peca, x, y)) = fantasma {
        grid.posicionar_peca_forcado(peca, x, y, Some(99));
    }

    if let Some(peca) = peca_segurada {
        grid.posicionar_peca(peca, x, y);
    }

    for linha in grid.posicoes.iter() {
        render.push_str(&" ".repeat(offset_horizontal as usize));
        render.push(PAREDE_BORDA);
        for bloco in linha.iter() {
            if *bloco != 0 {
                render.push_str(&paleta(*bloco));
                render.push(ESQUERA_BLOCO);
                render.push(DIREITA_BLOCO);
                render.push_str(&format!("{}{}", Bg(Reset), Fg(Reset)));
            } else {
                render.push_str("  ");
            }
        }
        render.push(PAREDE_BORDA);
        render.push_str("\r\n");
    }

    render.push_str(&" ".repeat(offset_horizontal as usize));
    render.push(CANTO_ESQUERDO_BORDA);
    render.push_str(&BASE_BORDA.to_string().repeat(LARGURA_GRID * 2));
    render.push(CANTO_DIREITO_BORDA);

    render
}
