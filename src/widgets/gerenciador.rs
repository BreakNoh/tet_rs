use ratatui::{
    layout::{Offset, Spacing},
    widgets::{Block, Clear, Padding, Paragraph},
};

use crate::core::{
    bag::{Bag, BagPecas},
    gerenciador::Gerenciador,
    peca::{Blocos, Peca, PecaBlocos},
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
        for (dx, _blo) in lin
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

fn renderizar_peca_centralizada(peca: &Peca, area: Rect, buf: &mut Buffer) {
    let dim = peca.dimensoes();
    let (l, _a) = (area.width as i32, area.height as i32);
    let (x, y) = (l / 2 - dim.x, (dim.y as f32 / 2.).ceil() as i32);

    let blocos = peca.blocos_rotacao(Rotacao::Norte);
    let tam = peca.tamanho();
    let area = area.offset(Offset { x, y });

    renderizar_blocos(blocos, IVec2::ZERO, tam, false, area, buf);
}

fn renderizar_proximas_pecas(ger: &Gerenciador<Bag>, area: Rect, buf: &mut Buffer) {
    let area = area.resize(Size::new(12, (2 + 1) * 5 + 3));

    let borda_proximas = Block::bordered()
        .border_set(BORDA)
        .title_top(Line::from("próximas").centered());
    let area_proximas = borda_proximas.inner(area);

    borda_proximas.render(area, buf);

    let linhas = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(2); 5])
        .spacing(Spacing::Space(1))
        .areas::<5>(area_proximas);

    if let Some(proximas) = ger.bag.espiar::<5>() {
        for (i, p) in proximas.iter().enumerate() {
            renderizar_peca_centralizada(p, linhas[i], buf);
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

    let borda_guardada = Block::bordered()
        .border_set(BORDA)
        .title_top(Line::from("guardada").centered());
    let area_guardada = borda_guardada.inner(linhas[0]);

    borda_guardada.render(linhas[0], buf);

    if let Some(guardada) = &ger.peca_guardada {
        renderizar_peca_centralizada(guardada, area_guardada, buf);
    }

    Paragraph::new(ger.nivel.to_string())
        .right_aligned()
        .block(
            Block::bordered()
                .border_set(BORDA)
                .title_top(Line::from("nível").centered()),
        )
        .render(linhas[2], buf);
    Paragraph::new(ger.pontos.to_string())
        .right_aligned()
        .block(
            Block::bordered()
                .border_set(BORDA)
                .title_top(Line::from("pontos").centered()),
        )
        .render(linhas[3], buf);
    Paragraph::new(ger.linhas_limpas.to_string())
        .right_aligned()
        .block(
            Block::bordered()
                .border_set(BORDA)
                .title_top(Line::from("linhas").centered()),
        )
        .render(linhas[4], buf);
}

fn renderizar_pausado(area: Rect, buf: &mut Buffer) {
    let linhas = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(5),
            Constraint::Fill(2),
        ])
        .areas::<3>(area);

    Clear.render(linhas[1], buf);

    let bloco = Block::bordered()
        .border_set(BORDA)
        .padding(Padding::vertical(1))
        .style(Style::default());

    let texto = Paragraph::new("pausado").centered().block(bloco);

    texto.render(linhas[1], buf);
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

        if self.pausado {
            renderizar_pausado(area_grid, buf);
        }

        renderizar_proximas_pecas(self, colunas[3], buf);
        renderizar_guardada_e_infos(self, colunas[1], buf);
    }
}
