use rand::seq::SliceRandom;

use super::*;

pub trait BagPecas<S: SRS + Copy, P: PecaBlocos<S> + Clone> {
    fn tamanho(&self) -> usize;
    fn proxima_peca(&mut self) -> P;
    fn recarregar(&mut self);

    fn espiar_enesima(&self, n: usize) -> &P;

    fn espiar<const N: usize>(&self) -> Option<[&P; N]> {
        if self.tamanho() >= N {
            Some(std::array::from_fn(|i| self.espiar_enesima(i)))
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct Bag {
    pub pecas_possiveis: Vec<Peca>,
    pub pecas: Vec<Peca>,
}

impl Bag {
    pub fn new() -> Self {
        let mut bag = Bag {
            pecas_possiveis: vec![
                pecas::i(),
                pecas::o(),
                pecas::j(),
                pecas::l(),
                pecas::s(),
                pecas::z(),
                pecas::t(),
            ],
            pecas: vec![],
        };

        bag.recarregar();

        bag
    }
}

impl Default for Bag {
    fn default() -> Self {
        Self::new()
    }
}

impl BagPecas<SRSBasico, Peca> for Bag {
    fn tamanho(&self) -> usize {
        self.pecas.len()
    }

    fn proxima_peca(&mut self) -> Peca {
        if self.pecas.len() <= 4 {
            self.recarregar();
        }
        self.pecas.remove(0)
    }

    fn espiar_enesima(&self, n: usize) -> &Peca {
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
    pub fila: Vec<Peca>,
}

#[cfg(test)]
impl BagPecas<SRSBasico, Peca> for BagTeste {
    fn tamanho(&self) -> usize {
        self.fila.len()
    }

    fn proxima_peca(&mut self) -> Peca {
        if self.fila.is_empty() {
            self.recarregar();
        }
        self.fila.remove(0)
    }

    fn espiar_enesima(&self, n: usize) -> &Peca {
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

    // #[test]
    // fn espiar_peca_quando_nao_tem_suficiente() {
    //     let mut bag = BagTeste {
    //         fila: vec![pecas::o(), pecas::i()],
    //     };
    //
    //     let espiadas = bag.espiar::<3>();
    //
    //     assert_eq!(*espiadas[2], pecas::t())
    // }
}
