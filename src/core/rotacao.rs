use super::*;

pub enum Rotacao {
    _0 = 0,
    _90 = 1,
    _180 = 2,
    _270 = 3,
}

pub trait SRS {
    fn testar(&self, blocos: &Blocos, pos: IVec2, grid: &Grid) -> Option<IVec2>;
}
