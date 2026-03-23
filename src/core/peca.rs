pub mod pecas;
use super::*;

const TAM_MAX_BLOCOS: usize = 4;

pub type Bloco = u8;
pub type Blocos = [[Bloco; TAM_MAX_BLOCOS]; TAM_MAX_BLOCOS];

pub const BLOCOS_BASE: Blocos = [[0; TAM_MAX_BLOCOS]; TAM_MAX_BLOCOS];

pub trait PecaBlocos {
    fn tamanho(&self) -> usize;
    fn dimensoes(&self) -> IVec2;

    fn rotacao(&self) -> Rotacao;
    fn set_rotacao(&mut self, rot: Rotacao);

    fn posicao(&self) -> IVec2;
    fn set_posicao(&mut self, pos: IVec2);

    fn blocos_rotacao(&self, rot: Rotacao) -> Blocos;

    fn blocos(&self) -> Blocos {
        self.blocos_rotacao(self.rotacao())
    }

    fn onde_vai_cair(&self, grid: &impl GridBlocos) -> IVec2 {
        let mut dy = 0;
        let pos = self.posicao();
        let blocos = self.blocos();
        let tam = self.tamanho();

        while grid.pode_posicionar(blocos, tam, pos + IVec2::new(0, dy)) {
            dy += 1;
        }

        let ajuste = if dy > 0 { dy - 1 } else { 0 }; // dy só para dentro de algo ou passando do chao

        IVec2::new(pos.x, pos.y + ajuste)
    }

    fn tentar_mover_para(&mut self, pos: IVec2, grid: &impl GridBlocos) {
        if grid.pode_posicionar(self.blocos(), self.tamanho(), pos) {
            self.set_posicao(pos);
        }
    }

    fn rotacionar_para<S: SRS>(
        &mut self,
        rot: Rotacao,
        grid: &impl GridBlocos,
        srs: &S,
    ) -> ResultadoSRS {
        let blocos = self.blocos_rotacao(rot);
        let pos = self.posicao();
        let trans = (self.rotacao(), rot);

        let resultado_srs = srs.validar_rotacao(self, rot, grid);

        if let ResultadoSRS::Valida(offset) = resultado_srs {
            self.set_rotacao(rot);
            let nova_pos = self.posicao() + offset;
            self.set_posicao(nova_pos);
        }

        resultado_srs
    }

    fn pode_rotacionar<S: SRS>(&self, rot: Rotacao, grid: &impl GridBlocos, srs: &S) -> bool {
        let blocos = self.blocos_rotacao(rot);
        let pos = self.posicao();
        let trans = (self.rotacao(), rot);

        matches!(
            srs.validar_rotacao(self, rot, grid),
            ResultadoSRS::Valida(_)
        )
    }

    fn pode_mover_para(&self, pos: IVec2, grid: &impl GridBlocos) -> bool {
        grid.pode_posicionar(self.blocos(), self.tamanho(), pos)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Peca {
    posicao: IVec2,
    rotacao: Rotacao,
    tamanho: usize,
    dimensoes: IVec2,      // na rotacao norte
    rotacoes: [Blocos; 4], // sentido horario
    srs: SRSBasico,
}

pub const fn rot90(blocos: Blocos, tam: usize, vezes: usize) -> Blocos {
    let mut blocos_rotacionados = blocos;

    let mut i = 0;

    while i < vezes % 4 {
        let mut temp = BLOCOS_BASE;
        let mut l = 0;

        while l < tam {
            let mut c = 0;
            while c < tam {
                temp[c][tam - 1 - l] = blocos_rotacionados[l][c];
                c += 1;
            }
            l += 1;
        }

        blocos_rotacionados = temp;
        i += 1;
    }

    blocos_rotacionados
}

pub const fn gerar_rotacoes(blocos: Blocos, tam: usize) -> [Blocos; 4] {
    [
        blocos,
        rot90(blocos, tam, 1),
        rot90(blocos, tam, 2),
        rot90(blocos, tam, 3),
    ]
}

impl PecaBlocos for Peca {
    fn tamanho(&self) -> usize {
        self.tamanho
    }

    fn dimensoes(&self) -> IVec2 {
        self.dimensoes
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
        esperados[0][0][0] = 1; // norte
        esperados[1][0][1] = 1; // leste
        esperados[2][1][1] = 1; // sul
        esperados[3][1][0] = 1; // oeste

        let rotacoes = gerar_rotacoes(base, tam);

        for (res, esp) in rotacoes.into_iter().zip(esperados.into_iter()) {
            assert_eq!(res, esp);
        }
    }

    #[test]
    fn peca_rotaciona() {
        let mut peca = pecas::t();
        peca.set_posicao(IVec2::new(3, 5)); // area que a peça tem espaço para rodar sem problemas
        let grid = Grid::new();
        let srs = SRSBasico;

        assert_eq!(peca.blocos(), peca.blocos_rotacao(Rotacao::Norte));

        peca.rotacionar_para(Rotacao::Leste, &grid, &srs);
        assert_eq!(peca.blocos(), peca.blocos_rotacao(Rotacao::Leste));

        peca.rotacionar_para(Rotacao::Oeste, &grid, &srs);
        assert_eq!(peca.blocos(), peca.blocos_rotacao(Rotacao::Oeste));

        peca.rotacionar_para(Rotacao::Sul, &grid, &srs);
        assert_eq!(peca.blocos(), peca.blocos_rotacao(Rotacao::Sul));
    }

    #[test]
    fn peca_nao_pode_rotacionar_se_srs_falhar() {
        let mut peca = pecas::t();
        peca.set_posicao(IVec2::new(8, 0)); // peca com a base colada na parede
        peca.set_rotacao(Rotacao::Oeste); // ponta virada para a esquerda
        let grid = Grid::new();

        assert!(!peca.pode_rotacionar(Rotacao::Leste, &grid, &SRSBasico)) // ponta fora do grid
    }
}
