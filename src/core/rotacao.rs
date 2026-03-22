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
    fn offsets(&self, trans: (Rotacao, Rotacao)) -> Option<[IVec2; 5]>;

    fn validar_rotacao(
        &self,
        blocos: Blocos,
        tam: usize,
        pos: IVec2,
        trans: (Rotacao, Rotacao),
        grid: &impl GridBlocos,
    ) -> ResultadoSRS {
        if let Some(offsets) = self.offsets(trans) {
            for offset in offsets.into_iter() {
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
pub struct SRSBasico;

impl SRS for SRSBasico {
    fn offsets(&self, _: (Rotacao, Rotacao)) -> Option<[IVec2; 5]> {
        Some([IVec2::ZERO; 5])
    }
}
