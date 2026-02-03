use std::{isize, usize};

use rand::seq::SliceRandom;
const LARGURA: usize = 10;
const ALTURA: usize = 20;

const PECA_T: Peca = Peca::Tri([[0, 1, 0], [1, 1, 1], [0; 3]]);
const PECA_O: Peca = Peca::Bi([[2; 2]; 2]);
const PECA_LE: Peca = Peca::Tri([[3, 0, 0], [3, 3, 3], [0; 3]]);
const PECA_LD: Peca = Peca::Tri([[0, 0, 4], [4, 4, 4], [0; 3]]);
const PECA_SE: Peca = Peca::Tri([[5, 5, 0], [0, 5, 5], [0; 3]]);
const PECA_SD: Peca = Peca::Tri([[0, 6, 6], [6, 6, 0], [0; 3]]);
const PECA_I: Peca = Peca::Quad([[0; 4], [7; 4], [0; 4], [0; 4]]);

#[derive(Clone, Debug)]
enum Peca {
    Bi([[u8; 2]; 2]),
    Tri([[u8; 3]; 3]),
    Quad([[u8; 4]; 4]),
}

#[derive(Default, Debug, Clone)]
enum Rotacao {
    #[default]
    _0,
    _90,
    _180,
    _270,
}

struct Estado {
    peca: Peca,
    rotacao: Rotacao,
    id_rot: usize,
    x: isize,
    y: usize,
    grid: [[u8; LARGURA]; ALTURA],
}

fn main() {
    let mut pecas = [PECA_T, PECA_SD, PECA_SE, PECA_O, PECA_LE, PECA_LD, PECA_I];
    let grid: [[u8; LARGURA]; ALTURA] = [[0; LARGURA]; ALTURA];
    let rotacoes = [Rotacao::_0, Rotacao::_90, Rotacao::_180, Rotacao::_270];
    pecas.shuffle(&mut rand::rng());
    let stdin = std::io::stdin();

    let mut estado = Estado {
        peca: pecas[0].clone(),
        rotacao: Rotacao::_0,
        id_rot: 0,
        x: 3,
        y: 1,
        grid,
    };

    // estado.grid = posicionar_peca(&estado);
    loop {
        let mut buff = String::new();
        let _ = stdin.read_line(&mut buff);

        if let Some(ch) = buff.chars().next() {
            match ch {
                'e' => {
                    estado.id_rot = (estado.id_rot + 1) % 4;
                    estado.rotacao = rotacoes[estado.id_rot].clone();
                }
                'q' => {
                    estado.id_rot = if estado.id_rot == 0 {
                        3
                    } else {
                        estado.id_rot - 1
                    };
                    estado.rotacao = rotacoes[estado.id_rot].clone();
                }
                'a' => {
                    estado.x -= 1;
                }
                'd' => {
                    estado.x += 1;
                }
                _ => {}
            }
        }
        estado.y += 1;
        print_grid(&estado);

        if checar_colisao(&estado) {
            trocar_peca(&mut estado, &mut pecas);
        }
    }
}

fn dentro_peca(estado: &Estado, x: usize, y: usize) -> bool {
    let n = tam_peca(&estado.peca);
    let dentro_x = (x as isize) >= estado.x && (x as isize) < estado.x + n as isize;
    let dentro_y = y >= estado.y && y < estado.y + n;

    dentro_x && dentro_y
}

fn print_grid(estado: &Estado) {
    let Estado {
        grid,
        peca,
        rotacao,
        ..
    } = estado;
    let peca_r = rotacionar(peca, rotacao);
    // let n = tam_peca(&estado.peca);
    let mut buffer = String::new();

    for (y, linha) in grid.iter().enumerate() {
        for (x, valor_grid) in linha.iter().enumerate() {
            let valor = if dentro_peca(estado, x, y) {
                let val = ler_peca(&peca_r, x, y);
                if val == 0 { *valor_grid } else { val }
            } else {
                *valor_grid
            };
            // let valor = if x >= estado.x && x < estado.x + n && y >= estado.y && y < estado.y + n {
            //     let valor = ler_peca(&peca_r, x - estado.x, y - estado.y);
            //     if valor == 0 { *valor_grid } else { valor }
            // } else {
            //     *valor_grid
            // };

            buffer.push_str(&format!("{valor:?} "));
        }
        buffer.push('\n');
    }

    println!("{buffer}")
}

fn checar_colisao(estado: &Estado) -> bool {
    let Estado {
        grid,
        x,
        y,
        peca,
        rotacao,
        ..
    } = estado;

    let peca_r = rotacionar(peca, rotacao);
    let n = tam_peca(peca);

    for dy in 0..n {
        for dx in 0..n {
            let ponto_peca = ler_peca(&peca_r, dx, dy);
            if y + dy + 1 >= ALTURA {
                if ponto_peca != 0 {
                    return true;
                }
                continue;
            }

            if grid[y + dy + 1][x + dx] != 0 && ponto_peca != 0 {
                return true;
            }
        }
    }

    false
}

fn trocar_peca(estado: &mut Estado, pecas: &mut [Peca; 7]) {
    posicionar_peca(estado);
    estado.x = 3;
    estado.y = 1;
    estado.rotacao = Rotacao::_0;
    pecas.shuffle(&mut rand::rng());
    estado.peca = pecas[0].clone();
}

fn mapear(n: usize, x: usize, y: usize, rot: &Rotacao) -> (usize, usize) {
    match rot {
        Rotacao::_0 => (y, x),
        Rotacao::_90 => (x, n - 1 - y),
        Rotacao::_180 => (n - 1 - y, n - 1 - x),
        Rotacao::_270 => (n - 1 - x, y),
    }
}

fn rotacionar(peca: &Peca, rot: &Rotacao) -> Peca {
    match peca {
        Peca::Bi(p) => {
            let n = 2;
            let mut peca_nova = [[0; 2]; 2];

            for y in 0..n {
                for x in 0..n {
                    let (ry, rx) = mapear(n, x, y, rot);
                    peca_nova[ry][rx] = p[y][x]
                }
            }
            Peca::Bi(peca_nova)
        }
        Peca::Tri(p) => {
            let n = 3;
            let mut peca_nova = [[0; 3]; 3];

            for y in 0..n {
                for x in 0..n {
                    let (ry, rx) = mapear(n, x, y, rot);
                    peca_nova[ry][rx] = p[y][x]
                }
            }
            Peca::Tri(peca_nova)
        }
        Peca::Quad(p) => {
            let n = 4;
            let mut peca_nova = [[0; 4]; 4];

            for y in 0..n {
                for x in 0..n {
                    let (ry, rx) = mapear(n, x, y, rot);
                    peca_nova[ry][rx] = p[y][x];
                }
            }
            Peca::Quad(peca_nova)
        }
    }
}

fn ler_peca(peca: &Peca, x: usize, y: usize) -> u8 {
    match peca {
        Peca::Bi(p) => p[y][x],
        Peca::Tri(p) => p[y][x],
        Peca::Quad(p) => p[y][x],
    }
}

fn tam_peca(peca: &Peca) -> usize {
    match peca {
        Peca::Bi(_) => 2,
        Peca::Tri(_) => 3,
        Peca::Quad(_) => 4,
    }
}

fn posicionar_peca(estado: &mut Estado) {
    let Estado {
        grid,
        peca,
        x,
        y,
        rotacao,
        ..
    } = estado;

    let peca_r = rotacionar(peca, rotacao);
    let mut grid_nova = grid.clone();

    let n = tam_peca(peca);
    for dy in 0..n {
        for dx in 0..n {
            let valor = ler_peca(&peca_r, dx, dy);
            if *y + dy >= ALTURA || *x + dx >= LARGURA || valor == 0 {
                continue;
            }
            grid_nova[*y + dy][*x + dx] = valor;
        }
    }

    estado.grid = grid_nova;
}
