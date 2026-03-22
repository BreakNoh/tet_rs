use std::collections::VecDeque;

use rand::seq::SliceRandom;

use super::*;

pub trait BagPecas<S: SRS + Copy, P: PecaBlocos<S> + Clone> {
    fn tamanho(&self) -> usize;
    fn proxima_peca(&mut self) -> P;
    fn espiar_enesima(&self, n: usize) -> &P;

    fn espiar<const N: usize>(&mut self) -> [&P; N] {
        while self.tamanho() <= N {
            self.recarregar();
        }

        std::array::from_fn(|i| self.espiar_enesima(i))
    }

    fn recarregar(&mut self);
}

pub struct Bag<P> {
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

    fn recarregar(&mut self) {
        let mut novo_lote = self.pecas_possiveis.clone();
        novo_lote.shuffle(&mut rand::rng());

        self.pecas.extend(novo_lote);
    }
}

#[cfg(test)]
pub struct BagTeste {
    pub fila: Vec<Peca<SRSBasico>>,
}

#[cfg(test)]
impl BagPecas<SRSBasico, Peca<SRSBasico>> for BagTeste {
    fn tamanho(&self) -> usize {
        self.fila.len()
    }

    fn proxima_peca(&mut self) -> Peca<SRSBasico> {
        if self.fila.is_empty() {
            self.recarregar();
        }
        self.fila.remove(0)
    }

    fn espiar_enesima(&self, n: usize) -> &Peca<SRSBasico> {
        &self.fila[n]
    }

    fn recarregar(&mut self) {
        self.fila.push(pecas::t());
        self.fila.push(pecas::i());
        self.fila.push(pecas::o());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bag_recarrega() {
        let mut bag = BagTeste {
            fila: vec![pecas::o()],
        };

        bag.recarregar();

        assert_eq!(
            bag.fila,
            vec![pecas::o(), pecas::t(), pecas::i(), pecas::o()]
        )
    }

    #[test]
    fn pegar_peca() {
        let mut bag = BagTeste {
            fila: vec![pecas::o(), pecas::i()],
        };

        assert_eq!(bag.proxima_peca(), pecas::o())
    }

    #[test]
    fn espiar_peca_quando_nao_tem_suficiente() {
        let mut bag = BagTeste {
            fila: vec![pecas::o(), pecas::i()],
        };

        let espiadas = bag.espiar::<3>();

        assert_eq!(*espiadas[2], pecas::t())
    }
}
