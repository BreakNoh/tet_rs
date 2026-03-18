use std::{collections::HashMap, fs};
use toml;

type Matiz = Vec<Vec<bool>>;

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
enum Rotacao {
    Leste, // 0
    Norte, // 90
    Oeste, // 180
    Sul,   // 270
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Peca {
    pub variante: String,
    rotacao: Rotacao,
    blocos: HashMap<Rotacao, Matiz>, // cada rotação é guardada e só é acessada
}

const ROTACOES: [Rotacao; 4] = [Rotacao::Leste, Rotacao::Sul, Rotacao::Oeste, Rotacao::Norte];

#[derive(Debug, PartialEq, Eq, Clone)]
enum Sentido {
    Horario,
    AntiHorario,
    CentoEOitenta,
}

impl Peca {
    pub fn rotacionar(&mut self, sentido: Sentido) {
        let idx = ROTACOES.iter().position(|r| *r == self.rotacao);

        if let Some(i) = idx {
            match sentido {
                Sentido::Horario => self.rotacao = ROTACOES[(i + 1) % 4],
                Sentido::AntiHorario => self.rotacao = ROTACOES[(4 + i - 1) % 4],
                Sentido::CentoEOitenta => self.rotacao = ROTACOES[(i + 2) % 4],
            }
        }
    }

    pub fn bloco_em(&self, x: usize, y: usize) -> bool {
        let mat = match self.blocos.get(&self.rotacao) {
            Some(m) => m,
            None => return false,
        };

        let n = mat.len();

        if x >= n || y >= n { false } else { mat[y][x] }
    }
}

fn rot90(mat: Matiz, horario: bool) -> Matiz {
    let n = mat.len();
    let mut mat_rot = vec![vec![false; n]; n];

    for i in 0..n {
        for j in 0..n {
            if horario {
                mat_rot[j][n - 1 - i] = mat[i][j];
            } else {
                mat_rot[n - 1 - j][i] = mat[i][j];
            }
        }
    }

    mat_rot
}

fn str_pra_mat(linhas: Vec<String>) -> Option<Matiz> {
    if !linhas.iter().all(|l| l.len() == linhas.len()) {
        return None;
    }

    Some(
        linhas
            .iter()
            .map(|linha| linha.chars().map(|ch| ch == '1').collect())
            .collect(),
    )
}

fn gerar_rotacoes(mat: Matiz) -> HashMap<Rotacao, Matiz> {
    HashMap::from([
        (Rotacao::Leste, mat.clone()),
        (Rotacao::Norte, rot90(mat.clone(), false)),
        (Rotacao::Sul, rot90(mat.clone(), true)),
        (Rotacao::Oeste, rot90(rot90(mat, true), true)),
    ])
}

fn carregar_pecas() -> Option<Vec<Peca>> {
    let raw = fs::read_to_string("../temas/pecas.toml").ok()?;
    let pecas_raw: HashMap<String, Vec<String>> = toml::from_str(&raw).ok()?;

    let pecas: Vec<Peca> = pecas_raw
        .into_iter()
        .filter_map(|(k, v)| {
            str_pra_mat(v).map(|mat| Peca {
                variante: k,
                rotacao: Rotacao::Leste,
                blocos: gerar_rotacoes(mat),
            })
        })
        .collect();

    Some(pecas)
}
