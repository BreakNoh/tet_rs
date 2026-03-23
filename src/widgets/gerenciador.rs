use ratatui::{
    layout::{Offset, Spacing},
    widgets::{Block, Paragraph},
};

use crate::core::{
    bag::{Bag, BagPecas},
    gerenciador::Gerenciador,
    peca::{Blocos, PecaBlocos},
    rotacao::Rotacao,
};

use super::*;

const BORDA: symbols::border::Set = symbols::border::Set {
    top_left: "+",
    top_right: "+",
    bottom_left: "+",
    bottom_right: "+",
    vertical_left: "|",
    vertical_right: "|",
    horizontal_top: "-",
    horizontal_bottom: "-",
};

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

fn renderizar_proximas_pecas(ger: &Gerenciador<Bag>, area: Rect, buf: &mut Buffer) {
    let linhas = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(6),
            Constraint::Length(4),
            Constraint::Length(4),
            Constraint::Length(4),
        ])
        .spacing(Spacing::Space(1))
        .areas::<4>(area);

    let borda_proxima = Block::bordered().border_set(BORDA);
    let area_proxima = borda_proxima.inner(linhas[0]);
    borda_proxima.render(linhas[0], buf);

    if let Some(proximas) = ger.bag.espiar::<4>() {
        renderizar_blocos(
            proximas[0].blocos_rotacao(Rotacao::Norte),
            IVec2::ZERO,
            proximas[0].tamanho(),
            false,
            area_proxima,
            buf,
        );

        for (i, p) in proximas.iter().skip(1).enumerate() {
            renderizar_blocos(
                p.blocos_rotacao(Rotacao::Norte),
                IVec2::ZERO,
                p.tamanho(),
                false,
                linhas[i + 1],
                buf,
            );
        }
    }
}

fn renderizar_guardada_e_infos(ger: &Gerenciador<Bag>, area: Rect, buf: &mut Buffer) {
    let linhas = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(6),
            Constraint::Fill(1),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
        ])
        .spacing(Spacing::Space(1))
        .areas::<5>(area);

    let borda_guardada = Block::bordered().border_set(BORDA);
    let area_guardada = borda_guardada.inner(linhas[0]);

    borda_guardada.render(linhas[0], buf);

    if let Some(guardada) = &ger.peca_guardada {
        renderizar_blocos(
            guardada.blocos_rotacao(Rotacao::Norte),
            IVec2::ZERO,
            guardada.tamanho(),
            false,
            area_guardada,
            buf,
        );
    }

    Paragraph::new(ger.nivel.to_string())
        .block(Block::bordered().border_set(BORDA).title_top("nivel"))
        .render(linhas[2], buf);
    Paragraph::new(ger.pontos.to_string())
        .block(Block::bordered().border_set(BORDA).title_top("pontos"))
        .render(linhas[3], buf);
    Paragraph::new(ger.linhas_limpas.to_string())
        .block(Block::bordered().border_set(BORDA).title_top("linhas"))
        .render(linhas[4], buf);
}

impl Widget for &Gerenciador<Bag> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let linhas = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Fill(1),
                Constraint::Length(22),
                Constraint::Fill(2),
            ])
            .areas::<3>(area);
        let colunas = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Fill(1),
                Constraint::Length(12),
                Constraint::Length(22),
                Constraint::Length(12),
                Constraint::Fill(1),
            ])
            .spacing(Spacing::Space(1))
            .areas::<5>(linhas[1]);

        let area_grid = colunas[2].resize(Size::new(20 + 2, 20 + 2));

        self.grid.render(area_grid, buf);
        renderizar_peca_e_previa(self, area_grid.offset(Offset::new(1, 1)), buf);
        renderizar_proximas_pecas(self, colunas[3], buf);
        renderizar_guardada_e_infos(self, colunas[1], buf);
    }
}
