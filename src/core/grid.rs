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

                let (x, y) = pos_checada.into();
                self.posicoes[y as usize][x as usize] = *bloco;
            }
        }
    }
}
