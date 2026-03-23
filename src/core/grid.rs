use super::*;

pub trait GridBlocos {
    fn dimensoes(&self) -> IVec2;
    fn posicao_ocupada(&self, pos: IVec2) -> bool;
    fn cair_linhas(&mut self);
    fn limpar_linhas(&mut self) -> i32;
    fn posicoes(&self) -> Vec<Vec<Bloco>>;
    fn limpar(&mut self);

    fn bloco_em(&self, pos: IVec2) -> Bloco;

    fn limpar_e_cair(&mut self) -> i32 {
        let limpas = self.limpar_linhas();
        self.cair_linhas();
        limpas
    }

    fn posicionar_peca<S: SRS + Copy>(&mut self, peca: &impl PecaBlocos<S>) {
        self.posicionar_blocos(peca.blocos(), peca.tamanho(), peca.posicao());
    }

    fn posicionar_peca_em<S: SRS + Copy>(&mut self, peca: &impl PecaBlocos<S>, pos: IVec2) {
        self.posicionar_blocos(peca.blocos(), peca.tamanho(), pos);
    }

    fn fora_dos_limites(&self, pos: IVec2) -> bool {
        let dim = self.dimensoes();

        pos.x < 0 || pos.x >= dim.x || pos.y < 0 || pos.y >= dim.y
    }

    fn pode_posicionar(&self, blocos: Blocos, tam: usize, pos: IVec2) -> bool {
        for (dy, linha) in blocos.into_iter().enumerate().take(tam) {
            for (dx, _) in linha
                .into_iter()
                .take(tam)
                .enumerate()
                .filter(|(_, b)| *b != 0)
            {
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

    fn posicionar_blocos(&mut self, blocos: Blocos, tam: usize, pos: IVec2) {
        for (dy, linha) in blocos.into_iter().enumerate().take(tam) {
            for (dx, bloco) in linha
                .into_iter()
                .take(tam)
                .enumerate()
                .filter(|(_, b)| *b != 0)
            {
                let pos_checada = pos + IVec2::new(dx as i32, dy as i32);

                if self.fora_dos_limites(pos_checada) {
                    continue;
                }

                self.posicionar_bloco(bloco, pos_checada);
            }
        }
    }
}

const TAM_MAX_GRID: usize = 50;
const LARGURA_GRID: usize = 10;
const ALTURA_GRID: usize = 20;

type Posicoes = [[Bloco; LARGURA_GRID]; ALTURA_GRID];
const POSICOES_BASE: Posicoes = [[0; LARGURA_GRID]; ALTURA_GRID];

#[derive(Debug, Default)]
pub struct Grid {
    pub posicoes: Posicoes,
}

impl Grid {
    pub const fn new() -> Self {
        Grid {
            // largura,
            // altura,
            posicoes: POSICOES_BASE,
        }
    }
}

impl GridBlocos for Grid {
    fn limpar(&mut self) {
        self.posicoes.fill([0; LARGURA_GRID]);
    }
    fn posicoes(&self) -> Vec<Vec<Bloco>> {
        self.posicoes.iter().map(Vec::from).collect()
    }
    fn limpar_linhas(&mut self) -> i32 {
        let mut limpas = 0;

        for lin in self
            .posicoes
            .iter_mut()
            .filter(|l| l.iter().all(|b| *b != 0))
        {
            limpas += 1;
            lin.fill(0);
        }

        limpas
    }

    fn bloco_em(&self, pos: IVec2) -> Bloco {
        if self.fora_dos_limites(pos) {
            0
        } else {
            self.posicoes[pos.y as usize][pos.x as usize]
        }
    }

    fn cair_linhas(&mut self) {
        let mut resultado = POSICOES_BASE;

        for (i, lin) in self
            .posicoes
            .iter_mut()
            .filter(|l| l.iter().any(|b| *b != 0))
            .rev()
            .enumerate()
        {
            resultado[ALTURA_GRID - 1 - i] = *lin
        }

        self.posicoes = resultado
    }

    fn dimensoes(&self) -> IVec2 {
        IVec2 {
            x: LARGURA_GRID as i32,
            y: ALTURA_GRID as i32,
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
        let mut grid = Grid::new();
        let mut esperado = POSICOES_BASE;
        let mut bloco = BLOCOS_BASE;
        let tam_bloco = 3;

        for i in 0..tam_bloco {
            // cria uma peça 000 111 000
            bloco[1][i] = 1;
            esperado[1][i] = 1;
        }

        grid.posicionar_blocos(bloco, tam_bloco, IVec2::new(0, 0));

        assert_eq!(grid.posicoes, esperado)
    }

    #[test]
    fn nao_deixar_posicionar_fora_do_grid() {
        let grid = Grid::new();
        let mut bloco = BLOCOS_BASE;
        let tam_bloco = 3;

        (0..tam_bloco).for_each(|i| {
            // cria uma peça 000 111 000
            bloco[1][i] = 1;
        });

        let pode_posicionar = grid.pode_posicionar(bloco, tam_bloco, IVec2::new(10, 20));

        assert!(!pode_posicionar)
    }

    #[test]
    fn nao_deixar_posicionar_encima_de_bloco() {
        let mut grid = Grid::new();
        let mut bloco = BLOCOS_BASE;
        let tam_bloco = 3;

        (0..tam_bloco).for_each(|i| {
            // cria uma peça 000 111 000
            bloco[1][i] = 1;
        });

        grid.posicoes[1][1] = 2;

        let pode_posicionar = grid.pode_posicionar(bloco, tam_bloco, IVec2::new(0, 0));

        assert!(!pode_posicionar)
    }

    #[test]
    fn deixar_posicionar() {
        let grid = Grid::new();
        let mut bloco = BLOCOS_BASE;
        let tam_bloco = 3;

        (0..tam_bloco).for_each(|i| {
            // cria uma peça 000 111 000
            bloco[1][i] = 1;
        });

        let pode_posicionar = grid.pode_posicionar(bloco, tam_bloco, IVec2::new(0, 0));

        assert!(pode_posicionar)
    }

    #[test]
    fn deixar_posicionar_com_padding() {
        let grid = Grid::new();
        let mut bloco = BLOCOS_BASE;
        let tam_bloco = 4;

        (1..tam_bloco).for_each(|i| {
            // cria uma peça 0000 0111 0000 0000
            bloco[1][i] = 1;
        });

        // grid.posicoes[1][1] = 2;

        let pode_posicionar = grid.pode_posicionar(bloco, tam_bloco, IVec2::new(-1, 0));

        assert!(pode_posicionar)
    }

    #[test]
    fn limpar_e_cair_linhas() {
        let mut grid = Grid::new();

        grid.posicoes[ALTURA_GRID - 4][0] = 1; // linha com dois bloco na esquerda
        grid.posicoes[ALTURA_GRID - 4][1] = 1; // pra testar se não tem overlap

        grid.posicoes[ALTURA_GRID - 2][0] = 1; // linha com um bloco na esquerda

        grid.posicoes[ALTURA_GRID - 1] = [1; LARGURA_GRID]; // linha cheia na base
        grid.posicoes[ALTURA_GRID - 3] = [1; LARGURA_GRID]; // linha cheia mais acima 

        let limpas = grid.limpar_e_cair();
        assert_eq!(limpas, 2);

        assert_eq!(
            grid.posicoes[ALTURA_GRID - 1],
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        );
        assert_eq!(
            grid.posicoes[ALTURA_GRID - 2],
            [1, 1, 0, 0, 0, 0, 0, 0, 0, 0]
        )
    }
}
