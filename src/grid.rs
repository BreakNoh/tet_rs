use crate::pecas::WrapperPeca;

const ALTURA_GRID: usize = 20;
const LARGURA_GRID: usize = 10;

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
    pub fn posicionar_peca(&mut self, peca: WrapperPeca, x: isize, y: isize) {
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
                    self.posicoes[y][x] = bloco_peca;
                }
            }
        }
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
}
