use super::*;

pub const fn t() -> Peca {
    let mut blocos = BLOCOS_BASE;
    blocos[0][1] = 1;
    blocos[1][0] = 1;
    blocos[1][1] = 1;
    blocos[1][2] = 1;

    Peca {
        posicao: IVec2::ZERO,
        rotacao: Rotacao::Norte,
        tamanho: 3,
        rotacoes: gerar_rotacoes(blocos, 3),
        srs: SRSBasico,
    }
}

pub const fn l() -> Peca {
    let mut blocos = BLOCOS_BASE;
    blocos[0][2] = 1;
    blocos[1][0] = 1;
    blocos[1][1] = 1;
    blocos[1][2] = 1;

    Peca {
        posicao: IVec2::ZERO,
        rotacao: Rotacao::Norte,
        tamanho: 3,
        rotacoes: gerar_rotacoes(blocos, 3),
        srs: SRSBasico,
    }
}

pub const fn j() -> Peca {
    let mut blocos = BLOCOS_BASE;
    blocos[0][0] = 1;
    blocos[1][0] = 1;
    blocos[1][1] = 1;
    blocos[1][2] = 1;

    Peca {
        posicao: IVec2::ZERO,
        rotacao: Rotacao::Norte,
        tamanho: 3,
        rotacoes: gerar_rotacoes(blocos, 3),
        srs: SRSBasico,
    }
}

pub const fn s() -> Peca {
    let mut blocos = BLOCOS_BASE;
    blocos[0][1] = 1;
    blocos[0][2] = 1;
    blocos[1][0] = 1;
    blocos[1][1] = 1;

    Peca {
        posicao: IVec2::ZERO,
        rotacao: Rotacao::Norte,
        tamanho: 3,
        rotacoes: gerar_rotacoes(blocos, 3),
        srs: SRSBasico,
    }
}

pub const fn z() -> Peca {
    let mut blocos = BLOCOS_BASE;
    blocos[0][0] = 1;
    blocos[0][1] = 1;
    blocos[1][1] = 1;
    blocos[1][2] = 1;

    Peca {
        posicao: IVec2::ZERO,
        rotacao: Rotacao::Norte,
        tamanho: 3,
        rotacoes: gerar_rotacoes(blocos, 3),
        srs: SRSBasico,
    }
}

pub const fn o() -> Peca {
    let mut blocos = BLOCOS_BASE;
    blocos[0][0] = 1;
    blocos[1][0] = 1;
    blocos[0][1] = 1;
    blocos[1][1] = 1;

    Peca {
        posicao: IVec2::ZERO,
        rotacao: Rotacao::Norte,
        tamanho: 2,
        rotacoes: gerar_rotacoes(blocos, 2),
        srs: SRSBasico,
    }
}
pub const fn i() -> Peca {
    let mut blocos = BLOCOS_BASE;
    blocos[1][0] = 1;
    blocos[1][1] = 1;
    blocos[1][2] = 1;
    blocos[1][3] = 1;

    Peca {
        posicao: IVec2::ZERO,
        rotacao: Rotacao::Norte,
        tamanho: 4,
        rotacoes: gerar_rotacoes(blocos, 4),
        srs: SRSBasico,
    }
}
