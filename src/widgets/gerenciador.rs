use crate::core::{
    bag::Bag,
    gerenciador::Gerenciador,
    grid::Grid,
    peca::{Peca, PecaBlocos},
    rotacao::SRSBasico,
};

use super::*;

impl Widget for &Gerenciador<Bag> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.grid.render(area, buf);

        let tam_peca = self.peca_atual.tamanho();
        let pos_previa = self.peca_atual.onde_vai_cair(&self.grid);
        let pos_peca = self.peca_atual.posicao();
        let blocos = self.peca_atual.blocos();

        for (dy, lin) in blocos.into_iter().take(tam_peca).enumerate() {
            for (dx, blo) in lin
                .into_iter()
                .take(tam_peca)
                .enumerate()
                .filter(|(_, b)| *b != 0)
            {
                let dx = dx as i32;
                let dy = dy as i32;

                let pos_bloco = pos_peca + IVec2::new(dx, dy);
                let pos_bloco_previa = pos_previa + IVec2::new(dx, dy);

                let x_tela = area.x + pos_bloco.x as u16 * 2;
                let x_tela_previa = area.x + pos_bloco_previa.x as u16 * 2;
                let y_tela = area.y + pos_bloco.y as u16;
                let y_tela_previa = area.y + pos_bloco_previa.y as u16;

                if x_tela_previa + 1 < area.right() && y_tela_previa < area.bottom() {
                    buf.set_string(x_tela_previa, y_tela_previa, "ZZ", Style::default());
                }
                if x_tela + 1 < area.right() && y_tela < area.bottom() {
                    buf.set_string(x_tela, y_tela, "[]", Style::default());
                }
            }
        }
    }
}
