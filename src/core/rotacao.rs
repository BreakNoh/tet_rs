use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Rotacao {
    Leste = 0,
    Sul = 1,
    Oeste = 2,
    Norte = 3,
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
