use glam::usize;

use super::*;

const TAM_MAX_BLOCOS: usize = 5;

pub(super) type Bloco = u8;
pub(super) type Blocos = [[Bloco; TAM_MAX_BLOCOS]; TAM_MAX_BLOCOS];

pub(super) struct Peca {
    rotacao: Rotacao,
    tamanho: usize,
    rotacoes: [Blocos; 4], // sentido horario
    tabela_srs: (),
}
