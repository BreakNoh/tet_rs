use super::{grid::Grid, pecas::WrapperPeca};

pub fn renderizar(
    mut grid: Grid,
    peca_segurada: Option<WrapperPeca>,
    x: isize,
    y: isize,
) -> String {
    let mut render = String::new();

    if let Some(peca) = peca_segurada {
        grid.posicionar_peca(peca, x, y);
    }

    for linha in grid.posicoes.iter() {
        for bloco in linha.iter() {
            render.push_str(&format!("{bloco} "));
        }
        render.push('\n');
    }

    render
}
