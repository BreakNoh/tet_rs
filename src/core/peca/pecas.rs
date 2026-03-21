use super::*;

fn t() -> Peca<SRSBasico> {
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

fn l() -> Peca<SRSBasico> {
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

fn j() -> Peca<SRSBasico> {
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

fn s() -> Peca<SRSBasico> {
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

fn z() -> Peca<SRSBasico> {
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

fn o() -> Peca<SRSBasico> {
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
fn i() -> Peca<SRSBasico> {
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
