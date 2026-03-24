mod core;
mod widgets;

use crossterm::{self, event};
use ratatui::DefaultTerminal;
use std::time::{Duration, Instant};

use crate::{
    core::{
        bag::{Bag, BagPecas},
        gerenciador::Gerenciador,
        peca::Peca,
        rotacao::{SRS, SRSBasico, SRSOficial},
    },
    widgets::paleta::PaletaPadrao,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ger = Gerenciador::new(Bag::default(), SRSOficial {});

    ratatui::run(|t| iniciar(ger, t))
}

fn calcular_gravidade(nivel: i32) -> Duration {
    let nivel_f = nivel as f64;
    let base_ms = 800.0;
    let decaimento = 0.90_f64;

    let mut ms = base_ms * decaimento.powf(nivel_f);

    if ms < 16.67 {
        ms = 16.67;
    }

    Duration::from_millis(ms as u64)
}

fn iniciar<B: BagPecas<Peca>, S: SRS>(
    mut ger: Gerenciador<B, S>,
    term: &mut DefaultTerminal,
) -> Result<(), Box<dyn std::error::Error>> {
    const DURACAO_FRAME: Duration = Duration::from_millis(16); // 60 fps

    let mut delta_tick = Duration::ZERO;
    let mut duracao_tick = calcular_gravidade(ger.nivel);
    let mut ult_nivel = ger.nivel;

    let mut ultimo_frame = Instant::now();
    let mut paleta = PaletaPadrao::default();

    loop {
        let tempo_frame = ultimo_frame.elapsed();
        ultimo_frame = Instant::now();

        if event::poll(Duration::from_millis(1))? {
            ger.processar_input(event::read()?);
        }

        if !ger.pausado {
            delta_tick += tempo_frame;

            while delta_tick >= duracao_tick {
                ger.tick();

                if ger.parar {
                    break;
                }
                delta_tick -= duracao_tick;

                if ger.nivel != ult_nivel {
                    duracao_tick = calcular_gravidade(ger.nivel);
                    ult_nivel = ger.nivel;
                }
            }
        }

        if ger.parar {
            break;
        }

        term.draw(|f| f.render_stateful_widget(&ger, f.area(), &mut paleta))?;

        let tempo_processado = ultimo_frame.elapsed();
        let tempo_restante = DURACAO_FRAME.saturating_sub(tempo_processado);
        if event::poll(tempo_restante)? {
            ger.processar_input(event::read()?);
        }
    }

    Ok(())
}
