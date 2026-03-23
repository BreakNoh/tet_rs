use super::*;
use crate::{
    core::grid::{Grid, GridBlocos},
    widgets::paleta::{Paleta, PaletaPadrao},
};
use ratatui::widgets;

const BORDA: symbols::border::Set = symbols::border::Set {
    top_left: "|",
    top_right: "|",
    bottom_left: "+",
    bottom_right: "+",
    vertical_left: "|",
    vertical_right: "|",
    horizontal_top: " ",
    horizontal_bottom: "-",
};

impl StatefulWidget for &Grid {
    type State = PaletaPadrao;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let IVec2 { x: lar, y: alt } = self.dimensoes();
        let bloco = widgets::Block::bordered().border_set(BORDA);

        (&bloco).render(area, buf);

        let area = bloco.inner(area);

        for dy in 0..alt.min(area.height as i32) {
            for dx in 0..lar.min((area.width / 2) as i32) {
                // ajuste pois os blcos terão 2 chars
                let bloco = self.bloco_em(IVec2::new(dx, dy));

                if bloco == 0 {
                    continue;
                }

                let x_tela = area.x + dx as u16 * 2;
                let y_tela = area.y + dy as u16;

                if x_tela + 1 < area.right() && y_tela < area.bottom() {
                    buf.set_string(x_tela, y_tela, "[]", state.estilo_de(bloco));
                }
            }
        }
    }
}
