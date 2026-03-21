pub mod pecas;
use super::*;

const TAM_MAX_BLOCOS: usize = 4;

pub type Bloco = u8;
pub type Blocos = [[Bloco; TAM_MAX_BLOCOS]; TAM_MAX_BLOCOS];

pub const BLOCOS_BASE: Blocos = [[0; TAM_MAX_BLOCOS]; TAM_MAX_BLOCOS];

pub trait PecaBlocos<S: SRS + Copy> {
    fn tamanho(&self) -> usize;

    fn rotacao(&self) -> Rotacao;
    fn set_rotacao(&mut self, rot: Rotacao);

    fn posicao(&self) -> IVec2;
    fn set_posicao(&mut self, pos: IVec2);

    fn blocos_rotacao(&self, rot: Rotacao) -> Blocos;
    fn srs(&self) -> S;

    fn blocos(&self) -> Blocos {
        self.blocos_rotacao(self.rotacao())
    }

    fn rotacionar_para(&mut self, rot: Rotacao, grid: &impl GridBlocos) {
        let teste = self.srs();
        let blocos = self.blocos_rotacao(rot);
        let pos = self.posicao();
        let trans = (self.rotacao(), rot);

        if let ResultadoSRS::Valida(offset) =
            teste.validar_rotacao(blocos, self.tamanho(), pos, trans, grid)
        {
            self.set_rotacao(rot);
            let nova_pos = self.posicao() + offset;
            self.set_posicao(nova_pos);
        }
    }

    fn pode_rotacionar(&self, rot: Rotacao, grid: &impl GridBlocos) -> bool {
        let teste = self.srs();
        let blocos = self.blocos_rotacao(rot);
        let pos = self.posicao();
        let trans = (self.rotacao(), rot);

        matches!(
            teste.validar_rotacao(blocos, self.tamanho(), pos, trans, grid),
            ResultadoSRS::Valida(_)
        )
    }
}

pub struct Peca<S: SRS + Copy> {
    posicao: IVec2,
    rotacao: Rotacao,
    tamanho: usize,
    rotacoes: [Blocos; 4], // sentido horario
    srs: S,
}

fn rot90(blocos: Blocos, tam: usize, vezes: usize) -> Blocos {
    let mut blocos_rotacionados = blocos;

    for _ in 0..(vezes % 4) {
        let mut temp = BLOCOS_BASE;

        for l in 0..tam {
            for c in 0..tam {
                temp[c][tam - 1 - l] = blocos_rotacionados[l][c];
            }
        }

        blocos_rotacionados = temp;
    }

    blocos_rotacionados
}

fn gerar_rotacoes(blocos: Blocos, tam: usize) -> [Blocos; 4] {
    [
        blocos,
        rot90(blocos, tam, 1),
        rot90(blocos, tam, 2),
        rot90(blocos, tam, 3),
    ]
}

impl<S: SRS + Copy> PecaBlocos<S> for Peca<S> {
    fn tamanho(&self) -> usize {
        self.tamanho
    }

    fn rotacao(&self) -> Rotacao {
        self.rotacao
    }

    fn set_rotacao(&mut self, rot: Rotacao) {
        self.rotacao = rot
    }

    fn posicao(&self) -> IVec2 {
        self.posicao
    }

    fn set_posicao(&mut self, pos: IVec2) {
        self.posicao = pos
    }

    fn blocos_rotacao(&self, rot: Rotacao) -> Blocos {
        if rot as usize >= 4 {
            self.rotacoes[0]
        } else {
            self.rotacoes[rot as usize]
        }
    }

    fn srs(&self) -> S {
        self.srs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gera_rotacoes_corretamente() {
        let mut base = BLOCOS_BASE;
        let mut esperados = [BLOCOS_BASE; 4];
        let tam = 2;

        base[0][0] = 1;
        esperados[0][0][0] = 1; // leste
        esperados[1][0][1] = 1; // sul
        esperados[2][1][1] = 1; // oeste
        esperados[3][1][0] = 1; // norte

        let rotacoes = gerar_rotacoes(base, tam);

        for (res, esp) in rotacoes.into_iter().zip(esperados.into_iter()) {
            assert_eq!(res, esp);
        }
    }
}
