pub const T: Peca<3> = Peca {
    blocos: [[0, 1, 0], [1, 1, 1], [0; 3]],
};
pub const LE: Peca<3> = Peca {
    blocos: [[2, 0, 0], [2, 2, 2], [0; 3]],
};
pub const SE: Peca<3> = Peca {
    blocos: [[3, 3, 0], [0, 3, 3], [0; 3]],
};
pub const LD: Peca<3> = Peca {
    blocos: [[0, 0, 4], [4, 4, 4], [0; 3]],
};
pub const SD: Peca<3> = Peca {
    blocos: [[0, 5, 5], [5, 5, 0], [0; 3]],
};
pub const O: Peca<2> = Peca {
    blocos: [[6; 2]; 2],
};
pub const I: Peca<5> = Peca {
    blocos: [[0; 5], [0; 5], [7, 7, 7, 7, 0], [0; 5], [0; 5]],
};

pub const PECAS: [WrapperPeca; 7] = [
    WrapperPeca::P3(T),
    WrapperPeca::P3(LE),
    WrapperPeca::P3(SE),
    WrapperPeca::P3(LD),
    WrapperPeca::P3(SD),
    WrapperPeca::P2(O),
    WrapperPeca::P5(I),
];

pub const ANGULOS: [Angulo; 4] = [Angulo::Leste, Angulo::Sul, Angulo::Oeste, Angulo::Norte];

#[derive(Clone, Copy, Debug)]
pub enum Angulo {
    Leste = 0,
    Oeste = 180,
    Sul = 90,
    Norte = 270,
}

#[derive(Clone, Copy, Debug)]
pub struct Peca<const N: usize> {
    blocos: [[u8; N]; N],
}

fn mapear<const N: usize>(x: usize, y: usize, angulo: Angulo) -> (usize, usize) {
    match angulo {
        Angulo::Leste => (y, x),
        Angulo::Sul => (x, (N - 1) - y),
        Angulo::Oeste => ((N - 1) - y, (N - 1) - x),
        Angulo::Norte => ((N - 1 - x), y),
    }
}

#[derive(Clone, Copy, Debug)]
pub enum WrapperPeca {
    P2(Peca<2>),
    P3(Peca<3>),
    P5(Peca<5>),
}

impl WrapperPeca {
    pub fn rotacionar(&self, angulo: Angulo) -> Self {
        match self {
            WrapperPeca::P2(peca) => WrapperPeca::P2(peca.rotacionar(angulo)),
            WrapperPeca::P3(peca) => WrapperPeca::P3(peca.rotacionar(angulo)),
            WrapperPeca::P5(peca) => WrapperPeca::P5(peca.rotacionar(angulo)),
        }
    }
    pub fn ler_bloco(&self, x: usize, y: usize) -> u8 {
        match self {
            WrapperPeca::P2(peca) => peca.ler_bloco(x, y),
            WrapperPeca::P3(peca) => peca.ler_bloco(x, y),
            WrapperPeca::P5(peca) => peca.ler_bloco(x, y),
        }
    }
    pub fn tamanho(&self) -> usize {
        match self {
            WrapperPeca::P2(_) => 2,
            WrapperPeca::P3(_) => 3,
            WrapperPeca::P5(_) => 5,
        }
    }
}

impl<const N: usize> Peca<N> {
    fn rotacionar(&self, angulo: Angulo) -> Self {
        let mut blocos_rotacionados = [[0; N]; N];

        for y in 0..N {
            for x in 0..N {
                let (ry, rx) = mapear::<N>(x, y, angulo);
                blocos_rotacionados[ry][rx] = self.blocos[y][x];
            }
        }

        Peca {
            blocos: blocos_rotacionados,
        }
    }

    pub fn ler_bloco(&self, x: usize, y: usize) -> u8 {
        if x >= N || y >= N {
            return 0;
        }
        self.blocos[y][x]
    }
}
