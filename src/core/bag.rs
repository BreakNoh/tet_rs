use std::collections::VecDeque;

use rand::seq::SliceRandom;

use super::*;

pub trait BagPecas<S: SRS + Copy, P: PecaBlocos<S> + Clone> {
    fn tamanho(&self) -> usize;
    fn proxima_peca(&mut self) -> P;
    fn espiar_enesima(&self, n: usize) -> &P;
    fn adicionar_peca(&mut self, peca: P);

    fn espiar<const N: usize>(&mut self) -> [&P; N] {
        while self.tamanho() <= N {
            self.recarregar();
        }

        std::array::from_fn(|i| self.espiar_enesima(i))
    }

    fn recarregar(&mut self);
}

struct Bag<P> {
    pecas_possiveis: Vec<P>,
    pecas: VecDeque<P>,
}

impl<S: SRS + Copy, P: PecaBlocos<S> + Clone> BagPecas<S, P> for Bag<P> {
    fn tamanho(&self) -> usize {
        self.pecas.len()
    }

    fn proxima_peca(&mut self) -> P {
        if self.pecas.is_empty() {
            self.recarregar();
        }
        self.pecas
            .pop_front()
            .expect("Bag deveria ter uma peca pelo menos, .recarregar() falhou")
    }

    fn espiar_enesima(&self, n: usize) -> &P {
        &self.pecas[n]
    }

    fn adicionar_peca(&mut self, peca: P) {
        self.pecas.push_back(peca);
    }

    fn recarregar(&mut self) {
        let mut novo_lote = self.pecas_possiveis.clone();
        novo_lote.shuffle(&mut rand::rng());

        self.pecas.extend(novo_lote);
    }
}
