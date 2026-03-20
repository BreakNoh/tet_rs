use super::*;

pub trait GridBlocos {
    fn dimensoes(&self) -> IVec2;
    fn posicao_ocupada(&self, pos: IVec2) -> bool;

    fn fora_dos_limites(&self, pos: IVec2) -> bool {
        let dim = self.dimensoes();

        pos.x < 0 || pos.x >= dim.x || pos.y < 0 || pos.y >= dim.y
    }

    fn pode_posicionar(&self, blocos: &Blocos, tam: usize, pos: IVec2) -> bool {
        for (dy, linha) in blocos.iter().enumerate().take(tam) {
            for (dx, _) in linha.iter().take(tam).enumerate().filter(|(_, b)| **b != 0) {
                // só itera pelas posições com bloco
                let pos_checada = pos + IVec2::new(dx as i32, dy as i32);

                if self.fora_dos_limites(pos_checada) {
                    return false;
                }
                if self.posicao_ocupada(pos_checada) {
                    return false;
                }
            }
        }

        true
    }

    fn posicionar_bloco(&mut self, bloco: Bloco, pos: IVec2);

    fn posicionar_blocos(&mut self, blocos: &Blocos, tam: usize, pos: IVec2) {
        for (dy, linha) in blocos.iter().enumerate().take(tam) {
            for (dx, bloco) in linha.iter().take(tam).enumerate().filter(|(_, b)| **b != 0) {
                let pos_checada = pos + IVec2::new(dx as i32, dy as i32);

                if self.fora_dos_limites(pos_checada) {
                    continue;
                }

                self.posicionar_bloco(*bloco, pos_checada);
            }
        }
    }
}

const TAM_MAX_GRID: usize = 50;

type Posicoes = [[Bloco; TAM_MAX_GRID]; TAM_MAX_GRID];
const POSICOES_BASE: Posicoes = [[0; TAM_MAX_GRID]; TAM_MAX_GRID];

pub struct Grid {
    posicoes: Posicoes,
    largura: usize,
    altura: usize,
}

impl Grid {
    fn new(altura: usize, largura: usize) -> Option<Self> {
        if altura > TAM_MAX_GRID || largura > TAM_MAX_GRID {
            None
        } else {
            Some(Grid {
                largura,
                altura,
                posicoes: POSICOES_BASE,
            })
        }
    }
}

impl GridBlocos for Grid {
    fn dimensoes(&self) -> IVec2 {
        IVec2 {
            x: self.largura as i32,
            y: self.altura as i32,
        }
    }
    fn posicao_ocupada(&self, pos: IVec2) -> bool {
        if self.fora_dos_limites(pos) {
            return false;
        }
        self.posicoes[pos.y as usize][pos.x as usize] != 0
    }

    fn posicionar_bloco(&mut self, bloco: Bloco, pos: IVec2) {
        if self.fora_dos_limites(pos) {
            return;
        }
        self.posicoes[pos.y as usize][pos.x as usize] = bloco;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn posicionar_blocos_dentro_grid() {
        let mut grid = Grid::new(3, 3).expect("grid deveria ter iniciado");
        let mut esperado = POSICOES_BASE;
        let mut bloco = BLOCOS_BASE;
        let tam_bloco = 3;

        for i in 0..tam_bloco {
            // cria uma peça 000 111 000
            bloco[1][i] = 1;
            esperado[1][i] = 1;
        }

        grid.posicionar_blocos(&bloco, tam_bloco, IVec2::new(0, 0));

        assert_eq!(grid.posicoes, esperado)
    }

    #[test]
    fn nao_deixar_posicionar_fora_do_grid() {
        let grid = Grid::new(3, 3).expect("grid deveria ter iniciado");
        let mut bloco = BLOCOS_BASE;
        let tam_bloco = 3;

        (0..tam_bloco).for_each(|i| {
            // cria uma peça 000 111 000
            bloco[1][i] = 1;
        });

        let pode_posicionar = grid.pode_posicionar(&bloco, tam_bloco, IVec2::new(1, 1));

        assert!(!pode_posicionar)
    }

    #[test]
    fn nao_deixar_posicionar_encima_de_bloco() {
        let mut grid = Grid::new(3, 3).expect("grid deveria ter iniciado");
        let mut bloco = BLOCOS_BASE;
        let tam_bloco = 3;

        (0..tam_bloco).for_each(|i| {
            // cria uma peça 000 111 000
            bloco[1][i] = 1;
        });

        grid.posicoes[1][1] = 2;

        let pode_posicionar = grid.pode_posicionar(&bloco, tam_bloco, IVec2::new(0, 0));

        assert!(!pode_posicionar)
    }

    #[test]
    fn deixar_posicionar() {
        let grid = Grid::new(3, 3).expect("grid deveria ter iniciado");
        let mut bloco = BLOCOS_BASE;
        let tam_bloco = 3;

        (0..tam_bloco).for_each(|i| {
            // cria uma peça 000 111 000
            bloco[1][i] = 1;
        });

        let pode_posicionar = grid.pode_posicionar(&bloco, tam_bloco, IVec2::new(0, 0));

        assert!(pode_posicionar)
    }

    #[test]
    fn deixar_posicionar_com_padding() {
        let grid = Grid::new(3, 3).expect("grid deveria ter iniciado");
        let mut bloco = BLOCOS_BASE;
        let tam_bloco = 4;

        (1..tam_bloco).for_each(|i| {
            // cria uma peça 0000 0111 0000 0000
            bloco[1][i] = 1;
        });

        // grid.posicoes[1][1] = 2;

        let pode_posicionar = grid.pode_posicionar(&bloco, tam_bloco, IVec2::new(-1, 0));

        assert!(pode_posicionar)
    }
}
