use super::*;

const TAM_MAX_BLOCOS: usize = 5;

pub type Bloco = u8;
pub type Blocos = [[Bloco; TAM_MAX_BLOCOS]; TAM_MAX_BLOCOS];

pub const BLOCOS_BASE: Blocos = [[0; TAM_MAX_BLOCOS]; TAM_MAX_BLOCOS];

pub trait PecaBlocos {
    fn tamanho(&self) -> usize;

    fn rotacao(&self) -> Rotacao;
    fn set_rotacao(&mut self, rot: Rotacao);

    fn posicao(&self) -> IVec2;
    fn set_posicao(&mut self, pos: IVec2);

    fn blocos_rotacao(&self, rot: Rotacao) -> Blocos;
    fn teste_srs(&self, rot: Rotacao) -> &impl SRS;

    fn blocos(&self) -> Blocos {
        self.blocos_rotacao(self.rotacao())
    }

    fn rotacionar_para(&mut self, rot: Rotacao, grid: &impl GridBlocos) {
        let teste = self.teste_srs(rot);
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
        let teste = self.teste_srs(rot);
        let blocos = self.blocos_rotacao(rot);
        let pos = self.posicao();
        let trans = (self.rotacao(), rot);

        matches!(
            teste.validar_rotacao(blocos, self.tamanho(), pos, trans, grid),
            ResultadoSRS::Valida(_)
        )
    }
}

pub struct Peca {
    rotacao: Rotacao,
    tamanho: usize,
    rotacoes: [Blocos; 4], // sentido horario
    tabela_srs: (),
}
