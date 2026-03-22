use glam::IVec2;
use ratatui::prelude::*;

mod gerenciador;
mod grid;

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use crate::core::{
        bag::Bag,
        gerenciador::{GERENCIADOR_MOCK, GERENCIADOR_MOCK_BAG, GerenciadorJogo},
        grid::{Grid, GridBlocos},
        peca::{PecaBlocos, pecas},
        rotacao::Rotacao,
    };

    use super::*;
    use ratatui::backend::TestBackend;

    #[test]
    fn grid() {
        let backend = TestBackend::new(20, 20);
        let mut terminal = Terminal::new(backend).unwrap();

        let mut grid = Grid::new();
        grid.posicionar_peca_em(&pecas::t(), IVec2::new(3, 2));
        grid.posicionar_peca_em(&pecas::o(), IVec2::new(5, 5));
        grid.posicionar_peca_em(&pecas::z(), IVec2::new(2, 16));
        terminal.draw(|f| f.render_widget(&grid, f.area())).unwrap();

        println!("--------- TESTE GRID ---------");
        println!("{:?}", terminal.backend().buffer());
        println!("------------------------------");
    }

    #[test]
    fn gerenciador() {
        let backend = TestBackend::new(20, 20);
        let mut terminal = Terminal::new(backend).unwrap();

        let mut grid = Grid::new();
        let mut ger = GERENCIADOR_MOCK_BAG;

        grid.posicionar_peca_em(&pecas::t(), IVec2::new(3, 17));
        grid.posicionar_peca_em(&pecas::o(), IVec2::new(5, 15));
        grid.posicionar_peca_em(&pecas::z(), IVec2::new(2, 16));
        ger.grid = grid;
        ger.peca_atual.set_posicao(IVec2::new(3, 2));
        ger.peca_atual.set_rotacao(Rotacao::Oeste);

        terminal.draw(|f| f.render_widget(&ger, f.area())).unwrap();

        println!("--------- TESTE GERENCIADOR ---------");
        println!("{:?}", terminal.backend().buffer());
        println!("------------------------------");
    }
}
