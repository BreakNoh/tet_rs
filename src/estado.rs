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

pub struct Estado {
    x: isize,
    y: isize,

    angulo: usize,
    peca_atual: usize,

    fantasma: (WrapperPeca, isize, isize),

    pecas: [WrapperPeca; 7],
    grid: Grid,
}

impl Estado {
    pub fn new() -> Self {
        let mut pecas = pecas::PECAS;
        pecas.shuffle(&mut rand::rng());
        let mut estado = Estado {
            x: ORIGEM_X,
            y: ORIGEM_Y,
            angulo: 0,
            peca_atual: 0,
            pecas,
            grid: grid::Grid::default(),
            fantasma: (WrapperPeca::P3(pecas::T), 0, 0),
        };
        estado.atualizar_fantasma();
        estado
    }

    pub fn trocar_peca(&mut self) {
        self.grid.posicionar_peca(self.peca_atual(), self.x, self.y);

        self.angulo = 0;
        self.x = ORIGEM_X;
        self.y = ORIGEM_Y;

        self.peca_atual += 1;
        if self.peca_atual == NUMERO_PECAS {
            self.peca_atual = 0;
            self.pecas.shuffle(&mut rand::rng());
        }
    }
    pub fn tick(&mut self) {
        if self
            .grid
            .checar_colisao(self.peca_atual(), self.x, self.y + 1, false)
            != Colisao::Base
        {
            self.y += 1;
        } else {
            self.trocar_peca();
            self.grid.limpar_completas();
            self.atualizar_fantasma();
        }
    }

    pub fn render(&self) {
        let render = renderizar(
            self.grid,
            Some(self.peca_atual()),
            self.x,
            self.y,
            Some(self.fantasma),
        );
        println!("{render}");
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

    fn peca_atual(&self) -> WrapperPeca {
        self.pecas[self.peca_atual].rotacionar(ANGULOS[self.angulo])
    }
}
