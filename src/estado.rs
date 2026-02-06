use std::isize;

use rand::seq::SliceRandom;

use crate::{
    grid::{self, Colisao, Grid},
    pecas::{self, ANGULOS, WrapperPeca},
    tema::Tema,
    visual::Desenhavel,
};

const ORIGEM_X: isize = 4;
const ORIGEM_Y: isize = 0;
const NUMERO_PECAS: usize = 7;
const NUMERO_ANGULOS: usize = 4;
const LINHAS_PARA_PASSAR_NIVEL: u32 = 10;

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

    pub nivel: u32,
    pub pontuacao: u32,
    pub linhas_limpas: u32,

    pecas: [WrapperPeca; 7],
    pecas_prox: [WrapperPeca; 7],
    pub grid: Grid,
}

impl Estado {
    pub fn new() -> Self {
        let mut pecas = pecas::PECAS;
        pecas.shuffle(&mut rand::rng());
        let mut pecas_prox = pecas::PECAS;
        pecas_prox.shuffle(&mut rand::rng());

        let mut estado = Estado {
            x: ORIGEM_X,
            y: ORIGEM_Y,
            angulo: 0,
            indice_peca: 0,
            peca_atual: pecas[0],
            nivel: 0,
            pontuacao: 0,
            linhas_limpas: 0,
            pecas,
            pecas_prox,
            grid: grid::Grid::default(),
            fantasma: (WrapperPeca::P3(pecas::T), 0, 0),
            peca_guardada: None,
            trocou: false,
        };
        estado.atualizar_fantasma();
        estado
    }

    pub fn derrubar_direto(&mut self) {
        self.fantasma.0.mudar_id(self.peca_atual.id());
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

    pub fn prox_peca(&mut self, ind: usize) -> WrapperPeca {
        let indice_real = self.indice_peca + ind;

        if indice_real < NUMERO_PECAS {
            self.pecas[indice_real]
        } else {
            self.pecas_prox[indice_real % NUMERO_PECAS]
        }
    }

    fn alterar_peca(&mut self) {
        self.angulo = 0;
        self.x = ORIGEM_X;
        self.y = ORIGEM_Y;

        self.indice_peca += 1;
        if self.indice_peca == NUMERO_PECAS {
            self.pecas = self.pecas_prox;

            self.indice_peca = 0;
            self.pecas_prox.shuffle(&mut rand::rng());
        }

        self.peca_atual = self.pecas[self.indice_peca];
        self.atualizar_fantasma();
    }

    fn trocar_peca_bruto(&mut self, peca: WrapperPeca, x: isize, y: isize) {
        self.grid.posicionar_peca(peca, x, y);

        if self.checar_se_perdeu() {
            self.perder();
            return;
        }

        let [linhas, pontos] = self.grid.limpar_completas();
        self.pontuacao += pontos * (self.nivel + 1);

        let meta = (self.nivel + 1) * LINHAS_PARA_PASSAR_NIVEL;

        if (self.linhas_limpas + linhas) >= meta && self.linhas_limpas < meta && linhas != 0 {
            self.nivel += 1;
        }

        self.linhas_limpas += linhas;

        self.alterar_peca();
        self.trocou = false;
    }

    pub fn tick(&mut self) {
        if !self.cair() {
            self.trocar_peca();
        }
    }

    pub fn checar_se_perdeu(&self) -> bool {
        for linha in self.grid.posicoes.iter().take(2) {
            if linha.iter().any(|bloco| *bloco != 0) {
                return true;
            }
        }
        false
    }

    pub fn perder(&mut self) {
        *self = Estado::new();
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
                self.fantasma.0.mudar_id(99);

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
        let colisao = self
            .grid
            .checar_colisao(self.peca_atual(), self.x + dir_x, self.y, true);

        match colisao {
            Colisao::Parede(_) | Colisao::Leteral(_) => (),
            Colisao::Nada | Colisao::Base => {
                self.x += dir_x;
                self.atualizar_fantasma();
            }
        }
    }

    pub fn girar(&mut self, sentido: isize) {
        let angulo_novo = match sentido {
            s if s > 0 => (self.angulo + 1) % NUMERO_ANGULOS,
            s if s < 0 && self.angulo == 0 => NUMERO_ANGULOS - 1,
            s if s < 0 && self.angulo > 0 => self.angulo - 1,
            _ => self.angulo,
        };

        let mut posicoes_teste = [[0, 0], [1, 0], [-1, 0], [0, -1]];
        if self.peca_atual.id() == 7 {
            // I
            posicoes_teste[2] = [-2, 0];
            posicoes_teste[1] = [2, 0];
        }

        let mut pos_valida = None;

        for [x, y] in posicoes_teste {
            let peca_rodada = self.peca_atual.rotacionar(ANGULOS[angulo_novo]);
            let colisao = self
                .grid
                .checar_colisao(peca_rodada, self.x + x, self.y + y, true);

            match colisao {
                Colisao::Nada => {
                    pos_valida = Some([self.x + x, self.y + y]);
                    break;
                }
                _ => (),
            }
        }

        if let Some([novo_x, novo_y]) = pos_valida {
            self.x = novo_x;
            self.y = novo_y;
            self.angulo = angulo_novo;
            self.atualizar_fantasma();
        }
    }

    pub fn peca_atual(&self) -> WrapperPeca {
        self.peca_atual.rotacionar(ANGULOS[self.angulo])
    }
}

impl Desenhavel for Estado {
    fn frame(&self, tema: &Tema) -> crate::visual::Frame {
        let mut frame = self.grid.frame(tema);

        frame.desenhar(
            self.fantasma.0,
            self.fantasma.1 * 2,
            self.fantasma.2,
            true,
            tema,
        );
        frame.desenhar(self.peca_atual(), self.x * 2, self.y, true, tema);

        frame
    }
}
