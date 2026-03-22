use super::*;
use crate::core::grid::{Grid, GridBlocos};

impl Widget for &Grid {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let IVec2 { x: lar, y: alt } = self.dimensoes();

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
                    buf.set_string(x_tela, y_tela, "[]", Style::default());
                }
            }
        }
    }
}
