use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use super::*;

#[derive(Debug)]
struct GerenciadorJogo<G, B, P, S>
where
    S: SRS + Copy,
    P: PecaBlocos<S> + Clone,
    G: GridBlocos,
    B: BagPecas<S, P>,
{
    grid: G,
    bag: B,
    peca_atual: P,
    peca_guardada: Option<P>,

    pontos: i32,
    nivel: i32,
    linhas_limpas: i32,

    ja_trocou: bool,

    parar: bool,

    _srs: std::marker::PhantomData<S>,
}

const ORIGEM_PECA: IVec2 = IVec2::new(3, 0);

impl<G, B, P, S> GerenciadorJogo<G, B, P, S>
where
    G: GridBlocos,
    B: BagPecas<S, P>,
    P: PecaBlocos<S> + Clone,
    S: SRS + Copy,
{
    pub fn tick(&mut self) {
        let prox_pos = self.peca_atual.posicao() + IVec2::new(0, 1);
        if self.peca_atual.pode_mover_para(prox_pos, &self.grid) {
            self.peca_atual.set_posicao(prox_pos);
        } else {
            self.grid.posicionar_peca(&self.peca_atual);
            self.limpar_linhas_e_pontuar();
            self.trocar_pra_proxima_peca();
            self.ja_trocou = true
        }
    }

    pub fn trocar_pra_proxima_peca(&mut self) {
        let mut prox_peca = self.bag.proxima_peca();
        prox_peca.set_posicao(ORIGEM_PECA);
        prox_peca.set_rotacao(Rotacao::Norte);
        self.peca_atual = prox_peca;
    }

    pub fn guardar_peca(&mut self) {
        if self.ja_trocou {
            return;
        }
        self.ja_trocou = true;

        let a_ser_guardada = self.peca_atual.clone();

        if let Some(mut a_ser_atual) = self.peca_guardada.take() {
            a_ser_atual.set_posicao(ORIGEM_PECA);
            a_ser_atual.set_rotacao(Rotacao::Norte);

            self.peca_atual = a_ser_atual;
        } else {
            self.trocar_pra_proxima_peca();
        }

        self.peca_guardada = Some(a_ser_guardada);
    }

    pub fn limpar_linhas_e_pontuar(&mut self) {
        let linhas = self.grid.limpar_e_cair();
        self.linhas_limpas += linhas;
        self.nivel = self.linhas_limpas / 10;

        self.pontos += (self.nivel + 1)
            * match linhas {
                1 => 100,
                2 => 300,
                3 => 500,
                4 => 800,
                _ => 0,
            }
    }

    pub fn tentar_rotacionar(&mut self, rot: Rotacao) {
        if let ResultadoSRS::Valida(offset) = self.peca_atual.rotacionar_para(rot, &self.grid) {
            let pos_corrigida = self.peca_atual.posicao() + offset;

            self.peca_atual.set_posicao(pos_corrigida);
        }
    }

    pub fn derrubar_direto(&mut self) {
        let pos_queda = self.peca_atual.onde_vai_cair(&self.grid);
        self.grid.posicionar_peca_em(&self.peca_atual, pos_queda);
        self.limpar_linhas_e_pontuar();
        self.trocar_pra_proxima_peca();
    }

    fn processar_input(&mut self, ev: KeyEvent) {
        let ctrl = ev.modifiers.contains(KeyModifiers::CONTROL);
        let rot = self.peca_atual.rotacao();
        let pos = self.peca_atual.posicao();

        match ev.code {
            KeyCode::Char('c') if ctrl => self.parar = true,

            KeyCode::Char('z') => self.tentar_rotacionar(rot.rot90ant()),
            KeyCode::Char('x') => self.tentar_rotacionar(rot.rot90hor()),
            KeyCode::Char('c') => self.guardar_peca(),

            KeyCode::Left => self
                .peca_atual
                .tentar_mover_para(pos + IVec2::new(-1, 0), &self.grid),
            KeyCode::Right => self
                .peca_atual
                .tentar_mover_para(pos + IVec2::new(1, 0), &self.grid),
            KeyCode::Up => self.derrubar_direto(),
            _ => (),
        };
    }
}

#[cfg(test)]
const GERENCIADOR_MOCK: GerenciadorJogo<Grid, BagTeste, Peca<SRSBasico>, SRSBasico> =
    GerenciadorJogo {
        grid: Grid::new(),
        bag: BagTeste { fila: vec![] },
        peca_atual: pecas::l(),
        peca_guardada: None,
        pontos: 0,
        nivel: 0,
        linhas_limpas: 0,
        ja_trocou: false,
        parar: false,
        _srs: std::marker::PhantomData,
    };

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trocar_de_peca() {
        let mut ger = GERENCIADOR_MOCK;

        assert_eq!(ger.peca_atual, pecas::l());

        ger.trocar_pra_proxima_peca(); // bag test sempre recarregar [t, i, o]

        ger.peca_atual.set_posicao(IVec2::ZERO); // peca já vem na ORIGEM_PECA

        assert_eq!(ger.peca_atual, pecas::t());
    }

    #[test]
    fn queda_brusca() {
        let mut ger = GERENCIADOR_MOCK;

        ger.derrubar_direto();

        assert_eq!(ger.grid.bloco_em(IVec2::new(0, 19)), 1, "{:?}", ger.grid);
        assert_eq!(ger.grid.bloco_em(IVec2::new(1, 19)), 1);
        assert_eq!(ger.grid.bloco_em(IVec2::new(2, 19)), 1);
        assert_eq!(ger.grid.bloco_em(IVec2::new(2, 18)), 1);

        ger.peca_atual.set_posicao(IVec2::ZERO); // peca já vem na ORIGEM_PECA

        assert_eq!(ger.peca_atual, pecas::t());
    }

    #[test]
    fn limpar_e_pontuar() {
        let mut ger = GERENCIADOR_MOCK;

        for i in 3..10 {
            ger.grid.posicoes[19][i] = 1;
        }

        ger.derrubar_direto(); // já limpa e pontua

        assert_eq!(ger.grid.posicoes[19], [0, 0, 1, 0, 0, 0, 0, 0, 0, 0]);

        assert_eq!(ger.pontos, 100);
        assert_eq!(ger.linhas_limpas, 1);
    }

    #[test]
    fn pontuacao() {
        let mut ger = GERENCIADOR_MOCK;
        ger.linhas_limpas = 9;

        ger.grid.posicoes[19].fill(1);
        ger.limpar_linhas_e_pontuar();

        assert_eq!(ger.linhas_limpas, 10);
        assert_eq!(ger.nivel, 1);
        assert_eq!(ger.pontos, 200); // (1 + 1) * 100

        ger.grid.posicoes[19].fill(1);
        ger.grid.posicoes[18].fill(1);
        ger.limpar_linhas_e_pontuar();

        assert_eq!(ger.linhas_limpas, 12);
        assert_eq!(ger.nivel, 1);
        assert_eq!(ger.pontos, 800); // 200 + (1 + 1) * 300

        ger.grid.posicoes[19].fill(1);
        ger.grid.posicoes[18].fill(1);
        ger.grid.posicoes[17].fill(1);
        ger.limpar_linhas_e_pontuar();

        assert_eq!(ger.linhas_limpas, 15);
        assert_eq!(ger.nivel, 1);
        assert_eq!(ger.pontos, 1800); // 800 + (1 + 1) * 500

        ger.linhas_limpas += 1;
        ger.grid.posicoes[19].fill(1);
        ger.grid.posicoes[18].fill(1);
        ger.grid.posicoes[17].fill(1);
        ger.grid.posicoes[16].fill(1);

        ger.limpar_linhas_e_pontuar();

        assert_eq!(ger.linhas_limpas, 20);
        assert_eq!(ger.nivel, 2);
        assert_eq!(ger.pontos, 4200); // 1800 + (2 + 1) * 800
    }
}
