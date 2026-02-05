use std::io::{Stdout, Write};

use crate::{
    grid::{ALTURA_GRID, LARGURA_GRID},
    tema::{self, BordaTema, Tema},
};

use serde::Deserialize;
use termion::{clear, color::*, cursor, raw::RawTerminal};

const LIMPAR_COR: &str = "\x1b[0m";
const ESPACO: char = ' ';
const ESQUERA_BLOCO: char = '\u{1FB34}'; // 🬴 
const DIREITA_BLOCO: char = '\u{1FB38}'; // 🬸 
const QUARTO_BAIXO: char = '\u{2584}'; //▄
const QUARTO_CIMA: char = '\u{2580}'; //▀
const SOMBRA_FRACA: char = '\u{2591}'; //░
const SOMBRA_MEDIA: char = '\u{2592}'; // ▒
const PINTADO: char = '\u{1CE8f}'; //𜺏
const QUARTO_CIMA_SOMBRA_MEDIA: char = '\u{1FB91}'; // 🮑
const QUARTO_BAIXO_SOMBRA_MEDIA: char = '\u{1FB92}'; // 🮑

const PAREDE_BORDA: char = '\u{2551}'; // ║
const BASE_BORDA: char = '\u{2550}'; // ═
const CANTO_ESQUERDO_BORDA: char = '\u{255a}'; // ╚
const CANTO_DIREITO_BORDA: char = '\u{255d}'; // ╝ 

#[derive(Clone, Copy, Debug, Deserialize)]
pub enum Cor {
    Vazio,
    Magenta,
    MagentaClaro,
    Azul,
    AzulClaro,
    Vermelho,
    VermelhoClaro,
    Amarelo,
    AmareloClaro,
    Ciano,
    CianoClaro,
    Verde,
    VerdeClaro,
    Branco,
    Preto,
    Cinza,
}

impl Cor {
    fn bg(&self) -> String {
        match self {
            Cor::Vazio => Bg(Reset).to_string(),
            Cor::Magenta => Bg(Magenta).to_string(),
            Cor::MagentaClaro => Bg(LightMagenta).to_string(),

            Cor::Azul => Bg(Blue).to_string(),
            Cor::AzulClaro => Bg(LightBlue).to_string(),

            Cor::Vermelho => Bg(Red).to_string(),
            Cor::VermelhoClaro => Bg(LightRed).to_string(),

            Cor::Amarelo => Bg(Yellow).to_string(),
            Cor::AmareloClaro => Bg(LightYellow).to_string(),

            Cor::Ciano => Bg(Cyan).to_string(),
            Cor::CianoClaro => Bg(LightCyan).to_string(),

            Cor::Verde => Bg(Green).to_string(),
            Cor::VerdeClaro => Bg(LightGreen).to_string(),

            Cor::Branco => Bg(White).to_string(),
            Cor::Preto => Bg(Black).to_string(),

            Cor::Cinza => Bg(LightBlack).to_string(),
        }
    }
    fn fg(&self) -> String {
        match &self {
            Cor::Vazio => Fg(Reset).to_string(),
            Cor::Magenta => Fg(Magenta).to_string(),
            Cor::MagentaClaro => Fg(LightMagenta).to_string(),

            Cor::Azul => Fg(Blue).to_string(),
            Cor::AzulClaro => Fg(LightBlue).to_string(),

            Cor::Vermelho => Fg(Red).to_string(),
            Cor::VermelhoClaro => Fg(LightRed).to_string(),

            Cor::Amarelo => Fg(Yellow).to_string(),
            Cor::AmareloClaro => Fg(LightYellow).to_string(),

            Cor::Ciano => Fg(Cyan).to_string(),
            Cor::CianoClaro => Fg(LightCyan).to_string(),

            Cor::Verde => Fg(Green).to_string(),
            Cor::VerdeClaro => Fg(LightGreen).to_string(),

            Cor::Branco => Fg(White).to_string(),
            Cor::Preto => Fg(Black).to_string(),

            Cor::Cinza => Fg(LightBlack).to_string(),
        }
    }
}

fn paleta(id: u8) -> (char, char, Cor, Cor) {
    let esquerdo = ESPACO;
    let direito = ESPACO;

    match id {
        1 => (esquerdo, direito, Cor::Magenta, Cor::Vazio), // T
        2 => (esquerdo, direito, Cor::Azul, Cor::Vazio),    // LE
        3 => (esquerdo, direito, Cor::Vermelho, Cor::Vazio), // SE
        4 => (SOMBRA_MEDIA, SOMBRA_MEDIA, Cor::Amarelo, Cor::Vermelho), // LD
        5 => (esquerdo, direito, Cor::Verde, Cor::Vazio),   // SD
        6 => (esquerdo, direito, Cor::Amarelo, Cor::Vazio), // O
        7 => (esquerdo, direito, Cor::Ciano, Cor::Vazio),   // I
        99 => (PINTADO, PINTADO, Cor::Vazio, Cor::Cinza),   // fantasma
        _ => (esquerdo, direito, Cor::Magenta, Cor::Verde), // ERRO
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Celula {
    transparente: bool,
    ch: char,
    bg: Cor,
    fg: Cor,
}

pub fn borda(tema: &Tema) -> Vec<Vec<Celula>> {
    let mut frame = Frame::new(LARGURA_GRID * 2 + 2, ALTURA_GRID + 1);

    let BordaTema {
        parede,
        base,
        canto_inf_dir,
        canto_inf_esq,
        cor,
        padrao,
        ..
    } = tema.borda;
    let [bg, fg] = cor.unwrap_or(tema.cores.padrao);

    let parede = Celula::new(parede.unwrap_or(padrao), bg, fg, false);
    let base = Celula::new(base.unwrap_or(padrao), bg, fg, false);
    let canto_dir = Celula::new(canto_inf_dir.unwrap_or(padrao), bg, fg, false);
    let canto_esq = Celula::new(canto_inf_esq.unwrap_or(padrao), bg, fg, false);

    frame.desenhar_quadrado(parede, 0, 0, 0, ALTURA_GRID - 1);
    frame.desenhar_quadrado(
        parede,
        LARGURA_GRID * 2 + 1,
        0,
        LARGURA_GRID * 2 + 1,
        ALTURA_GRID - 1,
    );
    frame.desenhar_quadrado(base, 1, ALTURA_GRID, LARGURA_GRID * 2 + 1, ALTURA_GRID);
    frame.desenhar_celula(canto_esq, 0, ALTURA_GRID);
    frame.desenhar_celula(canto_dir, LARGURA_GRID * 2 + 1, ALTURA_GRID);

    frame.celulas(tema)
}

pub fn bloco(vazio: bool, id: u8, tema: &Tema) -> [Celula; 2] {
    if vazio {
        return [Celula::vazia(); 2];
    }

    let (esq, dir, bg, fg) = tema.visual_id(id);
    [
        Celula::new(esq, bg, fg, false),
        Celula::new(dir, bg, fg, false),
    ]
}

impl Celula {
    pub fn vazia() -> Self {
        Celula {
            transparente: true,
            ch: ' ',
            bg: Cor::Preto,
            fg: Cor::Branco,
        }
    }
    pub fn new(ch: char, bg: Cor, fg: Cor, transparente: bool) -> Self {
        Celula {
            ch,
            bg,
            fg,
            transparente,
        }
    }
}

pub struct Frame {
    celulas: Vec<Vec<Celula>>,
    largura: usize,
    altura: usize,
}

pub trait Desenhavel {
    fn celulas(&self, tema: &Tema) -> Vec<Vec<Celula>> {
        Vec::new()
    }
    fn frame(&self, tema: &Tema) -> Frame {
        Frame::new(0, 0)
    }
}

impl Frame {
    pub fn new(largura: usize, altura: usize) -> Self {
        let celulas = (0..altura)
            .map(|_| (0..largura).map(|_| Celula::vazia()).collect())
            .collect();

        Frame {
            celulas,
            largura,
            altura,
        }
    }

    pub fn de_celula(celulas: Vec<Vec<Celula>>) -> Self {
        let altura = celulas.len();
        let largura = match celulas.get(0) {
            Some(l) => l.len(),
            None => 0,
        };

        Frame {
            celulas,
            altura,
            largura,
        }
    }

    pub fn desenhar_celula_forcado(&mut self, celula: Celula, x: usize, y: usize) {
        self.celulas[y][x] = celula;
    }

    pub fn desenhar_celula(&mut self, celula: Celula, x: usize, y: usize) {
        if celula.transparente || x >= self.largura || y >= self.altura {
            return;
        }
        self.desenhar_celula_forcado(celula, x, y);
    }

    pub fn desenhar_quadrado(
        &mut self,
        celula: Celula,
        x0: usize,
        y0: usize,
        xf: usize,
        yf: usize,
    ) {
        for y in y0..=yf {
            for x in x0..=xf {
                self.desenhar_celula(celula, x, y);
            }
        }
    }

    pub fn desenhar(
        &mut self,
        objeto: impl Desenhavel,
        x: isize,
        y: isize,
        transparencia: bool,
        tema: &Tema,
    ) {
        let mut celulas = objeto.celulas(tema);
        let frame = objeto.frame(tema);

        if celulas.len() == 0 {
            celulas = frame.celulas(tema);
        }

        self.desenhar_celulas(celulas, x, y, transparencia);
    }

    pub fn desenhar_celulas(
        &mut self,
        celulas: Vec<Vec<Celula>>,
        x: isize,
        y: isize,
        transparencia: bool,
    ) {
        for (dy, linha) in celulas.iter().enumerate() {
            let pos_y = dy as isize + y;
            if pos_y > self.altura as isize || pos_y < 0 {
                continue;
            }
            for (dx, celula) in linha.iter().enumerate() {
                let pos_x = dx as isize + x;

                if pos_x > self.altura as isize || pos_x < 0 {
                    continue;
                }

                if transparencia {
                    self.desenhar_celula(*celula, pos_x as usize, pos_y as usize);
                } else {
                    self.desenhar_celula_forcado(*celula, pos_x as usize, pos_y as usize);
                }
            }
        }
    }

    fn rasterizar(&self) -> String {
        let mut buffer = String::new();

        for linha in self.celulas.iter() {
            for c in linha.iter() {
                buffer.push_str(&c.bg.bg());
                buffer.push_str(&c.fg.fg());
                buffer.push(c.ch);
                buffer.push_str(LIMPAR_COR);
            }
            buffer.push_str("\r\n");
        }

        buffer
    }

    pub fn renderizar(&self, stdout: &mut RawTerminal<Stdout>, offset_y: u16) {
        let buffer = self.rasterizar();
        let origem = cursor::Goto(1, offset_y);
        let limpar_tela = clear::All;
        write!(stdout, "{limpar_tela}{origem}{buffer}").unwrap();
        stdout.flush().unwrap();
    }
}

impl Desenhavel for Frame {
    fn celulas(&self, tema: &Tema) -> Vec<Vec<Celula>> {
        self.celulas.clone()
    }
}
