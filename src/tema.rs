use crate::{pecas::WrapperPeca, visual::Cor};

struct Tema {
    borda: BordaTema,
    cores: CoresTema,
    chars: CharsTema,
}

struct BordaTema {
    base: char,
    parede: char,
    canto_sup_esq: char,
    canto_sup_dir: char,
    canto_inf_esq: char,
    canto_inf_dir: char,
}

struct CharsTema {
    padrao: [char; 2],
    fantasma: Option<[char; 2]>,
    t: Option<[char; 2]>,
    i: Option<[char; 2]>,
    o: Option<[char; 2]>,
    s: Option<[char; 2]>,
    z: Option<[char; 2]>,
    l: Option<[char; 2]>,
    j: Option<[char; 2]>,
}

struct CoresTema {
    padrao: [Cor; 2],
    fantasma: Option<[Cor; 2]>,
    t: Option<[Cor; 2]>,
    i: Option<[Cor; 2]>,
    o: Option<[Cor; 2]>,
    s: Option<[Cor; 2]>,
    z: Option<[Cor; 2]>,
    l: Option<[Cor; 2]>,
    j: Option<[Cor; 2]>,
}

impl Tema {
    pub fn visual_peca(&self, peca: WrapperPeca) -> (char, char, Cor, Cor) {
        let [bg, fg] = match peca.id() {
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
        let [esq, dir] = match peca.id() {
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
}
