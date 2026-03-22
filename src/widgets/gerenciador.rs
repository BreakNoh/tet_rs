use ratatui::layout::Offset;

use crate::core::{
    bag::Bag,
    gerenciador::Gerenciador,
    peca::{Blocos, PecaBlocos},
};

use super::*;

fn renderizar_blocos(
    blocos: Blocos,
    pos: IVec2,
    tam: usize,
    previa: bool,
    area: Rect,
    buf: &mut Buffer,
) {
    for (dy, lin) in blocos.into_iter().take(tam).enumerate() {
        for (dx, blo) in lin
            .into_iter()
            .take(tam)
            .enumerate()
            .filter(|(_, b)| *b != 0)
        {
            let dx = dx as i32;
            let dy = dy as i32;

            let pos_bloco = pos + IVec2::new(dx, dy);

            let x_tela = area.x + pos_bloco.x as u16 * 2;
            let y_tela = area.y + pos_bloco.y as u16;

            if x_tela + 1 < area.right() && y_tela < area.bottom() {
                buf.set_string(
                    x_tela,
                    y_tela,
                    if !previa { "[]" } else { "::" },
                    Style::default(),
                );
            }
        }
    }
}

fn renderizar_peca_e_previa(ger: &Gerenciador<Bag>, area: Rect, buf: &mut Buffer) {
    let tam_peca = ger.peca_atual.tamanho();
    let pos_previa = ger.peca_atual.onde_vai_cair(&ger.grid);
    let pos_peca = ger.peca_atual.posicao();
    let blocos = ger.peca_atual.blocos();

    renderizar_blocos(blocos, pos_previa, tam_peca, true, area, buf);
    renderizar_blocos(blocos, pos_peca, tam_peca, false, area, buf);
}

impl Widget for &Gerenciador<Bag> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let area_grid = area.resize(Size::new(20 + 2, 20 + 2));

        self.grid.render(area_grid, buf);
        renderizar_peca_e_previa(self, area_grid.offset(Offset::new(1, 1)), buf);
    }
}
