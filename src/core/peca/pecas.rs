use super::*;
pub mod blocos {
    use crate::core::peca::Bloco;

    pub const T: Bloco = 1;
    pub const I: Bloco = 2;
    pub const O: Bloco = 3;
    pub const S: Bloco = 4;
    pub const Z: Bloco = 5;
    pub const L: Bloco = 6;
    pub const J: Bloco = 7;
    pub const PREVIA: Bloco = 99;
}
use blocos::*;

pub const fn t() -> Peca {
    let mut blocos = BLOCOS_BASE;
    blocos[0][1] = T;
    blocos[1][0] = T;
    blocos[1][1] = T;
    blocos[1][2] = T;

    Peca {
        id: T,
        posicao: IVec2::ZERO,
        rotacao: Rotacao::Norte,
        tamanho: 3,
        dimensoes: IVec2::new(3, 2),
        rotacoes: gerar_rotacoes(blocos, 3),
    }
}

pub const fn l() -> Peca {
    let mut blocos = BLOCOS_BASE;
    blocos[0][2] = L;
    blocos[1][0] = L;
    blocos[1][1] = L;
    blocos[1][2] = L;

    Peca {
        id: L,
        posicao: IVec2::ZERO,
        rotacao: Rotacao::Norte,
        tamanho: 3,
        dimensoes: IVec2::new(3, 2),
        rotacoes: gerar_rotacoes(blocos, 3),
    }
}

pub const fn j() -> Peca {
    let mut blocos = BLOCOS_BASE;
    blocos[0][0] = J;
    blocos[1][0] = J;
    blocos[1][1] = J;
    blocos[1][2] = J;

    Peca {
        id: J,
        posicao: IVec2::ZERO,
        rotacao: Rotacao::Norte,
        tamanho: 3,
        dimensoes: IVec2::new(3, 2),
        rotacoes: gerar_rotacoes(blocos, 3),
    }
}

pub const fn s() -> Peca {
    let mut blocos = BLOCOS_BASE;
    blocos[0][1] = S;
    blocos[0][2] = S;
    blocos[1][0] = S;
    blocos[1][1] = S;

    Peca {
        id: S,
        posicao: IVec2::ZERO,
        rotacao: Rotacao::Norte,
        tamanho: 3,
        dimensoes: IVec2::new(3, 2),
        rotacoes: gerar_rotacoes(blocos, 3),
    }
}

pub const fn z() -> Peca {
    let mut blocos = BLOCOS_BASE;
    blocos[0][0] = Z;
    blocos[0][1] = Z;
    blocos[1][1] = Z;
    blocos[1][2] = Z;

    Peca {
        id: Z,
        posicao: IVec2::ZERO,
        rotacao: Rotacao::Norte,
        tamanho: 3,
        dimensoes: IVec2::new(3, 2),
        rotacoes: gerar_rotacoes(blocos, 3),
    }
}

pub const fn o() -> Peca {
    let mut blocos = BLOCOS_BASE;
    blocos[0][0] = O;
    blocos[1][0] = O;
    blocos[0][1] = O;
    blocos[1][1] = O;

    Peca {
        id: O,
        posicao: IVec2::ZERO,
        rotacao: Rotacao::Norte,
        tamanho: 2,
        dimensoes: IVec2::new(2, 2),
        rotacoes: gerar_rotacoes(blocos, 2),
    }
}
pub const fn i() -> Peca {
    let mut blocos = BLOCOS_BASE;
    blocos[1][0] = I;
    blocos[1][1] = I;
    blocos[1][2] = I;
    blocos[1][3] = I;

    Peca {
        id: I,
        posicao: IVec2::ZERO,
        rotacao: Rotacao::Norte,
        tamanho: 4,
        dimensoes: IVec2::new(4, 1),
        rotacoes: gerar_rotacoes(blocos, 4),
    }
}
