use crate::core::peca::pecas::blocos::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Rotacao {
    Norte = 0,
    Leste = 1,
    Sul = 2,
    Oeste = 3,
}

impl Rotacao {
    pub fn rot90hor(&self) -> Self {
        match self {
            Rotacao::Norte => Rotacao::Leste,
            Rotacao::Leste => Rotacao::Sul,
            Rotacao::Sul => Rotacao::Oeste,
            Rotacao::Oeste => Rotacao::Norte,
        }
    }
    pub fn rot90ant(&self) -> Self {
        match self {
            Rotacao::Norte => Rotacao::Oeste,
            Rotacao::Oeste => Rotacao::Sul,
            Rotacao::Sul => Rotacao::Leste,
            Rotacao::Leste => Rotacao::Norte,
        }
    }
    pub fn rot180(&self) -> Self {
        match self {
            Rotacao::Norte => Rotacao::Sul,
            Rotacao::Sul => Rotacao::Norte,
            Rotacao::Oeste => Rotacao::Leste,
            Rotacao::Leste => Rotacao::Oeste,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ResultadoSRS {
    Valida(IVec2),
    Invalida,
}

pub trait SRS {
    fn offsets(
        &self,
        id_peca: u8,
        trans: (Rotacao, Rotacao),
    ) -> Option<impl IntoIterator<Item = IVec2>>;

    fn validar_rotacao<P: PecaBlocos + ?Sized>(
        &self,
        peca: &P,
        rot: Rotacao,
        grid: &impl GridBlocos,
    ) -> ResultadoSRS {
        let trans = (peca.rotacao(), rot);
        let blocos = peca.blocos();
        let tam = peca.tamanho();
        let pos = peca.posicao();

        if let Some(offsets) = self.offsets(peca.id(), trans) {
            for offset in offsets {
                let pos_deslocadaa = pos + offset;

                if grid.pode_posicionar(blocos, tam, pos_deslocadaa) {
                    return ResultadoSRS::Valida(offset);
                }
            }
        }

        ResultadoSRS::Invalida
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SRSOficial;

impl SRSOficial {
    fn calcular_kicks(&self, de: [IVec2; 5], para: [IVec2; 5]) -> [IVec2; 5] {
        let mut kicks = [IVec2::ZERO; 5];

        for i in 0..5 {
            kicks[i] = de[i] - para[i];
        }

        kicks
    }
    fn jklstz(&self, trans: (Rotacao, Rotacao)) -> [IVec2; 5] {
        use Rotacao::*;

        let get_offsets = |r: Rotacao| -> [IVec2; 5] {
            match r {
                Norte => [
                    IVec2::new(0, 0),
                    IVec2::new(0, 0),
                    IVec2::new(0, 0),
                    IVec2::new(0, 0),
                    IVec2::new(0, 0),
                ],
                Leste => [
                    IVec2::new(0, 0),
                    IVec2::new(1, 0),
                    IVec2::new(1, -1),
                    IVec2::new(0, 2),
                    IVec2::new(1, 2),
                ],
                Sul => [
                    IVec2::new(0, 0),
                    IVec2::new(0, 0),
                    IVec2::new(0, 0),
                    IVec2::new(0, 0),
                    IVec2::new(0, 0),
                ],
                Oeste => [
                    IVec2::new(0, 0),
                    IVec2::new(-1, 0),
                    IVec2::new(-1, -1),
                    IVec2::new(0, 2),
                    IVec2::new(-1, 2),
                ],
            }
        };

        let (de, para) = trans;
        self.calcular_kicks(get_offsets(de), get_offsets(para))
    }
    fn i(&self, trans: (Rotacao, Rotacao)) -> [IVec2; 5] {
        use Rotacao::*;

        let get_offsets = |r: Rotacao| -> [IVec2; 5] {
            match r {
                Norte => [
                    IVec2::new(0, 0),
                    IVec2::new(-1, 0),
                    IVec2::new(2, 0),
                    IVec2::new(-1, 0),
                    IVec2::new(2, 0),
                ],
                Leste => [
                    IVec2::new(-1, 0),
                    IVec2::new(0, 0),
                    IVec2::new(0, 0),
                    IVec2::new(0, 1),
                    IVec2::new(0, -2),
                ],
                Sul => [
                    IVec2::new(-1, 1),
                    IVec2::new(1, 1),
                    IVec2::new(-2, 1),
                    IVec2::new(1, 0),
                    IVec2::new(-2, 0),
                ],
                Oeste => [
                    IVec2::new(0, 1),
                    IVec2::new(0, 1),
                    IVec2::new(0, 1),
                    IVec2::new(0, -1),
                    IVec2::new(0, 2),
                ],
            }
        };

        let (de, para) = trans;

        self.calcular_kicks(get_offsets(de), get_offsets(para))
    }

    fn o(&self, trans: (Rotacao, Rotacao)) -> [IVec2; 5] {
        use Rotacao::*;

        let get_offsets = |r: Rotacao| -> [IVec2; 5] {
            match r {
                Norte => [IVec2::new(0, 0); 5],
                Leste => [IVec2::new(0, -1); 5],
                Sul => [IVec2::new(-1, -1); 5],
                Oeste => [IVec2::new(-1, 0); 5],
            }
        };

        let (de, para) = trans;

        let mut kicks = self.calcular_kicks(get_offsets(de), get_offsets(para));
        kicks[0] = IVec2::ZERO;
        kicks
    }
}

impl SRS for SRSOficial {
    fn offsets(
        &self,
        id: u8,
        trans: (Rotacao, Rotacao),
    ) -> Option<impl IntoIterator<Item = IVec2>> {
        match id {
            I => Some(self.i(trans)),
            O => Some(self.o(trans)),
            _ => Some(self.jklstz(trans)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SRSBasico;

impl SRS for SRSBasico {
    fn offsets(
        &self,
        _id: u8,
        _trans: (Rotacao, Rotacao),
    ) -> Option<impl IntoIterator<Item = IVec2>> {
        Some([IVec2::ZERO])
    }
}
