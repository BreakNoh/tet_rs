use super::*;

pub(super) enum Rotacao {
    _0 = 0,
    _90 = 1,
    _180 = 2,
    _270 = 3,
}

pub(super) type TestesSRS = [IVec2; 5];

pub(super) trait SRS {
    fn testar<const M: usize, const N: usize, const O: usize>(
        &self,
        blocos: &Blocos<O>,
        pos: IVec2,
        grid: &Posicoes<M, N>,
    ) -> Option<IVec2>;
}

impl SRS for &TestesSRS {
    fn testar<const M: usize, const N: usize, const O: usize>(
        &self,
        blocos: &Blocos<O>,
        pos: IVec2,
        grid: &Posicoes<M, N>,
    ) -> Option<IVec2> {
        for t in self.iter() {}
        None
    }
}
