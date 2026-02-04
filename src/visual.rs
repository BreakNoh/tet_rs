use super::{grid::Grid, pecas::WrapperPeca};
use termion::{
    color::{self, *},
    style,
};

const ESQUERA_BLOCO: char = '\u{1FB34}'; // 🬴 
const DIREITA_BLOCO: char = '\u{1FB38}'; // 🬸 

fn paleta(id: u8) -> String {
    match id {
        3 => format!("{}{}", Bg(LightRed), Fg(Red)),   //SE
        2 => format!("{}{}", Bg(LightBlue), Fg(Blue)), // LE
        5 => format!("{}{}", Bg(LightGreen), Fg(Green)),
        4 => format!("{}{}", Bg(LightRed), Fg(Yellow)), // LD
        1 => format!("{}{}", Bg(LightMagenta), Fg(Magenta)), // T
        7 => format!("{}{}", Bg(LightCyan), Fg(Cyan)),  // I
        6 => format!("{}{}", Bg(LightYellow), Fg(Yellow)), // O
        _ => String::from(""),
    }
}

pub fn renderizar(
    mut grid: Grid,
    peca_segurada: Option<WrapperPeca>,
    x: isize,
    y: isize,
) -> String {
    let mut render = String::new();

    if let Some(peca) = peca_segurada {
        grid.posicionar_peca(peca, x, y);
    }

    for linha in grid.posicoes.iter() {
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
        render.push('\n');
    }

    render
}
