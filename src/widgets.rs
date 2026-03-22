use crate::core::*;
use glam::IVec2;
use ratatui::prelude::*;

mod grid;

#[cfg(test)]
mod tests {
    use crate::core::{
        grid::{Grid, GridBlocos},
        peca::pecas,
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
}
