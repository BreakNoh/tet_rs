pub struct Peca {
    pub variante: char,
    blocos: Vec<Vec<bool>>,
}

impl Peca {
    fn bloco_em(&self, x: usize, y: usize) -> bool {
        let n = self.blocos.len();

        if x >= n || y >= n {
            false
        } else {
            self.blocos[y][x]
        }
    }
    fn rot90(&mut self, horario: bool) {
        let n = self.blocos.len();
        let mut mat_rot = vec![vec![false; n]; n];

        for i in 0..n {
            for j in 0..n {
                if horario {
                    mat_rot[j][n - 1 - i] = self.blocos[i][j];
                } else {
                    mat_rot[n - 1 - j][i] = self.blocos[i][j];
                }
            }
        }

        self.blocos = mat_rot;
    }
}
