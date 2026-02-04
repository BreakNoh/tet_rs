use std::isize;

use rand::seq::SliceRandom;

use crate::{
    grid::{self, Colisao, Grid},
    pecas::{self, ANGULOS, Peca, WrapperPeca},
    visual::renderizar,
};

const ORIGEM_X: isize = 4;
const ORIGEM_Y: isize = 1;
const NUMERO_PECAS: usize = 7;
const NUMERO_ANGULOS: usize = 4;

#[derive(Debug, Clone, Copy)]
pub struct Estado {
    pub x: isize,
    pub y: isize,

    angulo: usize,
    indice_peca: usize,

    pub fantasma: (WrapperPeca, isize, isize),

    pub peca_atual: WrapperPeca,
    pub peca_guardada: Option<WrapperPeca>,

    trocou: bool,

    pecas: [WrapperPeca; 7],
    pub grid: Grid,
}

impl Estado {
    pub fn new() -> Self {
        let mut pecas = pecas::PECAS;
        pecas.shuffle(&mut rand::rng());
        let mut estado = Estado {
            x: ORIGEM_X,
            y: ORIGEM_Y,
            angulo: 0,
            indice_peca: 0,
            peca_atual: pecas[0],
            pecas,
            grid: grid::Grid::default(),
            fantasma: (WrapperPeca::P3(pecas::T), 0, 0),
            peca_guardada: None,
            trocou: false,
        };
        estado.atualizar_fantasma();
        estado
    }

    pub fn derrubar_direto(&mut self) {
        self.trocar_peca_bruto(self.fantasma.0, self.fantasma.1, self.fantasma.2);
    }

    fn trocar_peca(&mut self) {
        self.trocar_peca_bruto(self.peca_atual(), self.x, self.y);
    }

    pub fn guardar_peca(&mut self) {
        if self.trocou {
            return;
        }

        self.angulo = 0;
        self.x = ORIGEM_X;
        self.y = ORIGEM_Y;
        self.trocou = true;

        if let Some(p) = self.peca_guardada {
            let carry = self.peca_atual;
            self.peca_atual = p;
            self.peca_guardada = Some(carry);
            self.atualizar_fantasma();
        } else {
            self.peca_guardada = Some(self.peca_atual);
            self.alterar_peca();
        }
    }

    fn alterar_peca(&mut self) {
        self.angulo = 0;
        self.x = ORIGEM_X;
        self.y = ORIGEM_Y;

        self.indice_peca += 1;
        if self.indice_peca == NUMERO_PECAS {
            self.indice_peca = 0;
            self.pecas.shuffle(&mut rand::rng());
        }
        self.peca_atual = self.pecas[self.indice_peca];
        self.atualizar_fantasma();
    }

    fn trocar_peca_bruto(&mut self, peca: WrapperPeca, x: isize, y: isize) {
        self.grid.posicionar_peca(peca, x, y);
        self.grid.limpar_completas();
        self.alterar_peca();
        self.trocou = false;
    }

    pub fn tick(&mut self) {
        if !self.cair() {
            self.trocar_peca();
        }
    }

    pub fn render(&self, offset_horizontal: u16) -> String {
        let render = renderizar(*self, offset_horizontal);
        render
    }

    fn atualizar_fantasma(&mut self) {
        let mut dy = self.y;

        loop {
            if self
                .grid
                .checar_colisao(self.peca_atual(), self.x, dy + 1, false)
                == Colisao::Base
            {
                self.fantasma = (self.peca_atual(), self.x, dy);
                return;
            } else {
                dy += 1;
            }
        }
    }

    pub fn cair(&mut self) -> bool {
        if self
            .grid
            .checar_colisao(self.peca_atual(), self.x, self.y + 1, false)
            != Colisao::Base
        {
            self.y += 1;
            true
        } else {
            false
        }
    }

    pub fn mover(&mut self, dir_x: isize) {
        if self
            .grid
            .checar_colisao(self.peca_atual(), self.x + dir_x, self.y, true)
            != Colisao::Parede
        {
            self.x += dir_x;
            self.atualizar_fantasma();
        }
    }

    pub fn girar(&mut self, sentido: isize) {
        match sentido {
            s if s > 0 => {
                self.angulo = (self.angulo + 1) % NUMERO_ANGULOS;
                self.atualizar_fantasma();
            }
            s if s < 0 => {
                self.angulo = if self.angulo == 0 {
                    NUMERO_ANGULOS - 1
                } else {
                    self.angulo - 1
                };
                self.atualizar_fantasma();
            }
            _ => (),
        }
    }

    pub fn peca_atual(&self) -> WrapperPeca {
        self.peca_atual.rotacionar(ANGULOS[self.angulo])
    }
}
