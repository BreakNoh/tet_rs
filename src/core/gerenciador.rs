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
    pub fn iniciar(&mut self) {
        todo!()
    }

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
        self.peca_atual = self.bag.proxima_peca();
        self.peca_atual.set_posicao(ORIGEM_PECA);
    }

    pub fn guardar_peca(&mut self) {
        if self.ja_trocou {
            return;
        }
        self.ja_trocou = true;

        let a_ser_guardada = self.peca_atual.clone();

        if let Some(mut a_ser_atual) = self.peca_guardada.take() {
            a_ser_atual.set_posicao(ORIGEM_PECA);

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
            KeyCode::Up => {
                let pos_queda = self.peca_atual.onde_vai_cair(&self.grid);
                self.grid.posicionar_peca_em(&self.peca_atual, pos_queda);
                self.limpar_linhas_e_pontuar();
                self.trocar_pra_proxima_peca();
            }
            _ => (),
        };
    }
}
