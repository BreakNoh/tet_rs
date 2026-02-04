use crate::pecas::WrapperPeca;

pub const ALTURA_GRID: usize = 20;
pub const LARGURA_GRID: usize = 10;

#[derive(PartialEq, Eq, Clone, Copy, Debug, Default)]
pub enum Colisao {
    #[default]
    Nada,
    Parede,
    Base,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Grid {
    pub posicoes: [[u8; LARGURA_GRID]; ALTURA_GRID],
}

fn esta_dentro_x(x: isize) -> bool {
    x >= 0 && (x as usize) < LARGURA_GRID
}
fn esta_dentro_y(y: isize) -> bool {
    y >= 0 && (y as usize) < ALTURA_GRID
}

impl Grid {
    pub fn posicionar_peca_forcado(
        &mut self,
        peca: WrapperPeca,
        x: isize,
        y: isize,
        valor_forcado: Option<u8>,
    ) {
        let n = peca.tamanho() as isize;

        for dy in 0..n {
            if !esta_dentro_y(y + dy) {
                continue;
            }
            for dx in 0..n {
                if !esta_dentro_x(x + dx) {
                    continue;
                }
                let bloco_peca = peca.ler_bloco(dx as usize, dy as usize);

                if bloco_peca != 0 {
                    let x = (x + dx) as usize;
                    let y = (y + dy) as usize;
                    self.posicoes[y][x] = valor_forcado.unwrap_or(bloco_peca);
                }
            }
        }
    }

    pub fn posicionar_peca(&mut self, peca: WrapperPeca, x: isize, y: isize) {
        self.posicionar_peca_forcado(peca, x, y, None);
    }

    pub fn checar_colisao(
        &self,
        peca: WrapperPeca,
        x: isize,
        y: isize,
        horizontal: bool,
    ) -> Colisao {
        let n = peca.tamanho() as isize;

        for dy in 0..n {
            for dx in 0..n {
                let bloco_peca = peca.ler_bloco(dx as usize, dy as usize);

                if bloco_peca != 0 {
                    if !esta_dentro_y(y + dy) {
                        return Colisao::Base;
                    }
                    if !esta_dentro_x(x + dx) {
                        return Colisao::Parede;
                    }

                    let x = (x + dx) as usize;
                    let y = (y + dy) as usize;

                    if self.posicoes[y][x] != 0 {
                        return match horizontal {
                            true => Colisao::Parede,
                            false => Colisao::Base,
                        };
                    }
                }
            }
        }
        Colisao::Nada
    }

    pub fn limpar_completas(&mut self) {
        let saltos = self.checar_linhas();

        if saltos.iter().any(|v| *v != 0) {
            self.derrubar_linhas(saltos);
        }
    }

    fn checar_linhas(&self) -> [usize; ALTURA_GRID] {
        let mut movimento = [0; ALTURA_GRID];
        let mut salto = 0;

        for (i, linha) in self.posicoes.iter().enumerate().rev() {
            if linha.iter().all(|v| *v != 0) {
                salto += 1;
            } else {
                movimento[i] = salto;
            }
        }

        movimento
    }

    fn derrubar_linhas(&mut self, movimento: [usize; ALTURA_GRID]) {
        let mut novo_grid = [[0; LARGURA_GRID]; ALTURA_GRID];

        for (i, salto) in movimento.iter().enumerate().rev() {
            let destino = *salto + i;

            if destino < ALTURA_GRID {
                novo_grid[destino] = self.posicoes[i];
            }
        }

        self.posicoes = novo_grid;
    }
}
