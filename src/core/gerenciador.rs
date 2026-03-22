use crate::widgets::gerenciador;
use ratatui::widgets::Widget;
use std::time::{Duration, Instant};

use crossterm::event::{self, KeyCode, KeyEvent, KeyModifiers};

use super::*;

// trait GerenciadorJogo<G, B, P, S>
// where
//     S: SRS + Copy,
//     P: PecaBlocos<S> + Clone,
//     G: GridBlocos,
//     B: BagPecas<S, P>,
// {
//     fn peca_atual(&self) -> &P;
//     fn peca_atual_mut(&mut self) -> &mut P;
//     fn set_peca_atual(&mut self, peca: P);
//
//     fn peca_guardada(&self) -> Option<&P>;
//     fn set_peca_guardada(&mut self, peca: Option<P>);
//
//     fn grid(&self) -> &G;
//     fn grid_mut(&mut self) -> &mut G;
//
//     fn set_nivel(&mut self, nivel: i32);
//     fn set_pontos(&mut self, pontos: i32);
//     fn set_linhas_limpas(&mut self, linhas: i32);
//
//     fn ja_trocou(&self) -> bool;
//     fn set_ja_trocou(&mut self, v: bool);
//
//     fn bag(&self) -> &B;
//     fn bag_mut(&mut self) -> &mut B;
//
//     fn tick(&mut self) {
//         let prox_pos = self.peca_atual().posicao() + IVec2::new(0, 1);
//
//         if self.peca_atual().pode_mover_para(prox_pos, self.grid()) {
//             self.peca_atual_mut().set_posicao(prox_pos);
//         } else {
//             let peca = self.peca_atual().clone();
//             self.grid_mut().posicionar_peca(&peca);
//
//             self.limpar_linhas_e_pontuar();
//             self.trocar_pra_proxima_peca();
//             self.set_ja_trocou(true);
//         }
//     }
//
//     fn trocar_pra_proxima_peca(&mut self) {
//         let mut prox_peca = self.bag_mut().proxima_peca();
//         prox_peca.set_posicao(ORIGEM_PECA);
//         prox_peca.set_rotacao(Rotacao::Norte);
//         self.set_peca_atual(prox_peca);
//     }
//
//     fn guardar_peca(&mut self) {
//         if self.ja_trocou() {
//             return;
//         }
//         self.set_ja_trocou(true);
//
//         let a_ser_guardada = self.peca_atual().clone();
//
//         if let Some(peca_guarda) = self.peca_guardada() {
//             let mut a_ser_atual = peca_guarda.clone();
//
//             a_ser_atual.set_posicao(ORIGEM_PECA);
//             a_ser_atual.set_rotacao(Rotacao::Norte);
//
//             self.set_peca_atual(a_ser_atual);
//         } else {
//             self.trocar_pra_proxima_peca();
//         }
//
//         self.set_peca_guardada(Some(a_ser_guardada));
//     }
//
//     pub fn limpar_linhas_e_pontuar(&mut self) {
//         let linhas = self.grid.limpar_e_cair();
//         self.set_linhas_limpas(self.linhas);
//         self.nivel = self.linhas_limpas / 10;
//
//         self.pontos += (self.nivel + 1)
//             * match linhas {
//                 1 => 100,
//                 2 => 300,
//                 3 => 500,
//                 4 => 800,
//                 _ => 0,
//             }
//     }
//
//     pub fn tentar_rotacionar(&mut self, rot: Rotacao) {
//         if let ResultadoSRS::Valida(offset) = self.peca_atual.rotacionar_para(rot, &self.grid) {
//             let pos_corrigida = self.peca_atual.posicao() + offset;
//
//             self.peca_atual.set_posicao(pos_corrigida);
//         }
//     }
//
//     pub fn derrubar_direto(&mut self) {
//         let pos_queda = self.peca_atual.onde_vai_cair(&self.grid);
//         self.grid.posicionar_peca_em(&self.peca_atual, pos_queda);
//         self.limpar_linhas_e_pontuar();
//         self.trocar_pra_proxima_peca();
//     }
//
//     pub fn processar_input(&mut self, ev: event::Event) {
//         if let event::Event::Key(ev) = ev {
//             let ctrl = ev.modifiers.contains(KeyModifiers::CONTROL);
//             let rot = self.peca_atual.rotacao();
//             let pos = self.peca_atual.posicao();
//
//             match ev.code {
//                 KeyCode::Char('c') if ctrl => self.parar = true,
//
//                 KeyCode::Char('z') => self.tentar_rotacionar(rot.rot90ant()),
//                 KeyCode::Char('x') => self.tentar_rotacionar(rot.rot90hor()),
//                 KeyCode::Char('c') => self.guardar_peca(),
//
//                 KeyCode::Left => self
//                     .peca_atual
//                     .tentar_mover_para(pos + IVec2::new(-1, 0), &self.grid),
//                 KeyCode::Right => self
//                     .peca_atual
//                     .tentar_mover_para(pos + IVec2::new(1, 0), &self.grid),
//                 KeyCode::Up => self.derrubar_direto(),
//                 _ => (),
//             };
//         }
//     }
// }

#[derive(Debug)]
pub struct Gerenciador<B> {
    pub grid: Grid,
    pub bag: B,
    pub peca_atual: Peca,
    pub peca_guardada: Option<Peca>,

    pub pontos: i32,
    pub nivel: i32,
    pub linhas_limpas: i32,

    pub ja_trocou: bool,

    pub parar: bool,
}

const ORIGEM_PECA: IVec2 = IVec2::new(3, 0);

impl<B: BagPecas<SRSBasico, Peca>> Gerenciador<B> {
    pub fn new(mut bag: B) -> Self {
        let peca_atual = bag.proxima_peca();
        Gerenciador {
            grid: Grid::new(),
            bag,
            peca_atual,
            peca_guardada: None,
            pontos: 0,
            nivel: 0,
            linhas_limpas: 0,
            ja_trocou: false,
            parar: false,
        }
    }

    pub fn tick(&mut self) {
        let prox_pos = self.peca_atual.posicao() + IVec2::new(0, 1);
        if self.peca_atual.pode_mover_para(prox_pos, &self.grid) {
            self.peca_atual.set_posicao(prox_pos);
        } else {
            self.grid.posicionar_peca(&self.peca_atual);
            self.limpar_linhas_e_pontuar();
            self.trocar_pra_proxima_peca();
            self.ja_trocou = true
        }
    }

    pub fn trocar_pra_proxima_peca(&mut self) {
        let mut prox_peca = self.bag.proxima_peca();
        prox_peca.set_posicao(ORIGEM_PECA);
        prox_peca.set_rotacao(Rotacao::Norte);
        self.peca_atual = prox_peca;
    }

    pub fn guardar_peca(&mut self) {
        if self.ja_trocou {
            return;
        }
        self.ja_trocou = true;

        let a_ser_guardada = self.peca_atual.clone();

        if let Some(mut a_ser_atual) = self.peca_guardada.take() {
            a_ser_atual.set_posicao(ORIGEM_PECA);
            a_ser_atual.set_rotacao(Rotacao::Norte);

            self.peca_atual = a_ser_atual;
        } else {
            self.trocar_pra_proxima_peca();
        }

        self.peca_guardada = Some(a_ser_guardada);
    }

    pub fn limpar_linhas_e_pontuar(&mut self) {
        let linhas = self.grid.limpar_e_cair();
        self.linhas_limpas += linhas;
        self.nivel = self.linhas_limpas / 10;

        self.pontos += (self.nivel + 1)
            * match linhas {
                1 => 100,
                2 => 300,
                3 => 500,
                4 => 800,
                _ => 0,
            }
    }

    pub fn tentar_rotacionar(&mut self, rot: Rotacao) {
        if let ResultadoSRS::Valida(offset) = self.peca_atual.rotacionar_para(rot, &self.grid) {
            let pos_corrigida = self.peca_atual.posicao() + offset;

            self.peca_atual.set_posicao(pos_corrigida);
        }
    }

    pub fn derrubar_direto(&mut self) {
        let pos_queda = self.peca_atual.onde_vai_cair(&self.grid);
        self.grid.posicionar_peca_em(&self.peca_atual, pos_queda);
        self.limpar_linhas_e_pontuar();
        self.trocar_pra_proxima_peca();
    }

    pub fn processar_input(&mut self, ev: event::Event) {
        if let event::Event::Key(ev) = ev {
            let ctrl = ev.modifiers.contains(KeyModifiers::CONTROL);
            let rot = self.peca_atual.rotacao();
            let pos = self.peca_atual.posicao();

            match ev.code {
                KeyCode::Char('c') if ctrl => self.parar = true,

                KeyCode::Char('z') => self.tentar_rotacionar(rot.rot90ant()),
                KeyCode::Char('x') => self.tentar_rotacionar(rot.rot90hor()),
                KeyCode::Char('c') => self.guardar_peca(),

                KeyCode::Left => self
                    .peca_atual
                    .tentar_mover_para(pos + IVec2::new(-1, 0), &self.grid),
                KeyCode::Right => self
                    .peca_atual
                    .tentar_mover_para(pos + IVec2::new(1, 0), &self.grid),
                KeyCode::Up => self.derrubar_direto(),
                _ => (),
            };
        }
    }
}

#[cfg(test)]
pub const GERENCIADOR_MOCK: Gerenciador<BagTeste> = Gerenciador {
    grid: Grid::new(),
    bag: BagTeste { fila: vec![] },
    peca_atual: pecas::l(),
    peca_guardada: None,
    pontos: 0,
    nivel: 0,
    linhas_limpas: 0,
    ja_trocou: false,
    parar: false,
};

#[cfg(test)]
use std::collections::VecDeque;

#[cfg(test)]
pub const GERENCIADOR_MOCK_BAG: Gerenciador<Bag> = Gerenciador {
    grid: Grid::new(),
    bag: Bag {
        pecas: vec![],
        pecas_possiveis: vec![],
    },
    peca_atual: pecas::l(),
    peca_guardada: None,
    pontos: 0,
    nivel: 0,
    linhas_limpas: 0,
    ja_trocou: false,
    parar: false,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trocar_de_peca() {
        let mut ger = GERENCIADOR_MOCK;

        assert_eq!(ger.peca_atual, pecas::l());

        ger.trocar_pra_proxima_peca(); // bag test sempre recarregar [t, i, o]

        ger.peca_atual.set_posicao(IVec2::ZERO); // peca já vem na ORIGEM_PECA

        assert_eq!(ger.peca_atual, pecas::t());
    }

    #[test]
    fn queda_brusca() {
        let mut ger = GERENCIADOR_MOCK;

        ger.derrubar_direto();

        assert_eq!(ger.grid.bloco_em(IVec2::new(0, 19)), 1, "{:?}", ger.grid);
        assert_eq!(ger.grid.bloco_em(IVec2::new(1, 19)), 1);
        assert_eq!(ger.grid.bloco_em(IVec2::new(2, 19)), 1);
        assert_eq!(ger.grid.bloco_em(IVec2::new(2, 18)), 1);

        ger.peca_atual.set_posicao(IVec2::ZERO); // peca já vem na ORIGEM_PECA

        assert_eq!(ger.peca_atual, pecas::t());
    }

    #[test]
    fn limpar_e_pontuar() {
        let mut ger = GERENCIADOR_MOCK;

        for i in 3..10 {
            ger.grid.posicoes[19][i] = 1;
        }

        ger.derrubar_direto(); // já limpa e pontua

        assert_eq!(ger.grid.posicoes[19], [0, 0, 1, 0, 0, 0, 0, 0, 0, 0]);

        assert_eq!(ger.pontos, 100);
        assert_eq!(ger.linhas_limpas, 1);
    }

    #[test]
    fn pontuacao() {
        let mut ger = GERENCIADOR_MOCK;
        ger.linhas_limpas = 9;

        ger.grid.posicoes[19].fill(1);
        ger.limpar_linhas_e_pontuar();

        assert_eq!(ger.linhas_limpas, 10);
        assert_eq!(ger.nivel, 1);
        assert_eq!(ger.pontos, 200); // (1 + 1) * 100

        ger.grid.posicoes[19].fill(1);
        ger.grid.posicoes[18].fill(1);
        ger.limpar_linhas_e_pontuar();

        assert_eq!(ger.linhas_limpas, 12);
        assert_eq!(ger.nivel, 1);
        assert_eq!(ger.pontos, 800); // 200 + (1 + 1) * 300

        ger.grid.posicoes[19].fill(1);
        ger.grid.posicoes[18].fill(1);
        ger.grid.posicoes[17].fill(1);
        ger.limpar_linhas_e_pontuar();

        assert_eq!(ger.linhas_limpas, 15);
        assert_eq!(ger.nivel, 1);
        assert_eq!(ger.pontos, 1800); // 800 + (1 + 1) * 500

        ger.linhas_limpas += 1;
        ger.grid.posicoes[19].fill(1);
        ger.grid.posicoes[18].fill(1);
        ger.grid.posicoes[17].fill(1);
        ger.grid.posicoes[16].fill(1);

        ger.limpar_linhas_e_pontuar();

        assert_eq!(ger.linhas_limpas, 20);
        assert_eq!(ger.nivel, 2);
        assert_eq!(ger.pontos, 4200); // 1800 + (2 + 1) * 800
    }
}
