use crate::{pecas::WrapperPeca, tema, visual::Cor};
use serde::Deserialize;
use std::fs::read_to_string;

#[derive(Debug, Deserialize)]
pub struct Tema {
    pub borda: BordaTema,
    pub cores: CoresTema,
    pub chars: CharsTema,
}

#[derive(Debug, Deserialize)]
pub struct BordaTema {
    pub cor: Option<[Cor; 2]>,
    pub padrao: char,
    pub base: Option<char>,
    pub parede: Option<char>,
    pub canto_sup_esq: Option<char>,
    pub canto_sup_dir: Option<char>,
    pub canto_inf_esq: Option<char>,
    pub canto_inf_dir: Option<char>,
}

#[derive(Debug, Deserialize)]
pub struct CharsTema {
    pub padrao: [char; 2],
    pub fantasma: Option<[char; 2]>,
    pub t: Option<[char; 2]>,
    pub i: Option<[char; 2]>,
    pub o: Option<[char; 2]>,
    pub s: Option<[char; 2]>,
    pub z: Option<[char; 2]>,
    pub l: Option<[char; 2]>,
    pub j: Option<[char; 2]>,
}

#[derive(Debug, Deserialize)]
pub struct CoresTema {
    pub padrao: [Cor; 2],
    pub fantasma: Option<[Cor; 2]>,
    pub t: Option<[Cor; 2]>,
    pub i: Option<[Cor; 2]>,
    pub o: Option<[Cor; 2]>,
    pub s: Option<[Cor; 2]>,
    pub z: Option<[Cor; 2]>,
    pub l: Option<[Cor; 2]>,
    pub j: Option<[Cor; 2]>,
}

impl Default for Tema {
    fn default() -> Self {
        Tema {
            borda: BordaTema {
                cor: None,
                padrao: 'X',
                base: None,
                parede: None,
                canto_sup_esq: None,
                canto_sup_dir: None,
                canto_inf_esq: None,
                canto_inf_dir: None,
            },
            cores: CoresTema {
                padrao: [Cor::Vazio, Cor::Branco],
                fantasma: None,
                t: None,
                i: None,
                o: None,
                s: None,
                z: None,
                l: None,
                j: None,
            },
            chars: CharsTema {
                padrao: ['[', ']'],
                fantasma: None,
                t: None,
                i: None,
                o: None,
                s: None,
                z: None,
                l: None,
                j: None,
            },
        }
    }
}

impl Tema {
    pub fn visual_id(&self, id: u8) -> (char, char, Cor, Cor) {
        let [bg, fg] = match id {
            1 => self.cores.t,
            2 => self.cores.j,
            3 => self.cores.z,
            4 => self.cores.l,
            5 => self.cores.s,
            6 => self.cores.o,
            7 => self.cores.i,
            99 => self.cores.fantasma,
            _ => Some(self.cores.padrao),
        }
        .unwrap_or(self.cores.padrao);
        let [esq, dir] = match id {
            1 => self.chars.t,
            2 => self.chars.j,
            3 => self.chars.z,
            4 => self.chars.l,
            5 => self.chars.s,
            6 => self.chars.o,
            7 => self.chars.i,
            99 => self.chars.fantasma,
            _ => Some(self.chars.padrao),
        }
        .unwrap_or(self.chars.padrao);

        (esq, dir, bg, fg)
    }
    pub fn visual_peca(&self, peca: WrapperPeca) -> (char, char, Cor, Cor) {
        self.visual_id(peca.id())
    }

    pub fn carregar(arquivo: &str) -> Result<Tema, &str> {
        let texto_tema = read_to_string(&arquivo);

        if texto_tema.is_err() {
            return Err("Arquivo não encontrado");
        }

        let tema_parseado = toml::from_str(&texto_tema.unwrap());

        if let Ok(tema) = tema_parseado {
            Ok(tema)
        } else {
            Err("Tema mal formado")
        }
    }
}
