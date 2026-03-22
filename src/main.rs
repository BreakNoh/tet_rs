mod core;
mod widgets;

use crossterm::{self, event, terminal::EnterAlternateScreen};
use ratatui::DefaultTerminal;
use std::time::{Duration, Instant};

use crate::core::{
    bag::Bag,
    gerenciador::{self, Gerenciador},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ger = Gerenciador::new(Bag::default());

    ratatui::run(|t| iniciar(ger, t))
}

fn iniciar(
    mut ger: Gerenciador<Bag>,
    term: &mut DefaultTerminal,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut contagem_frames = 0;
    const FRAMES_POR_TICK: i32 = 60;
    const DURACAO_FRAME: Duration = Duration::from_millis(16); // 60 fps

    crossterm::execute!(std::io::stdout(), EnterAlternateScreen)?;

    loop {
        let inicio_frame = Instant::now();

        if event::poll(Duration::from_millis(1))? {
            ger.processar_input(event::read()?);
        }

        if !ger.pausado && contagem_frames >= FRAMES_POR_TICK {
            ger.tick();
            contagem_frames = 0;
        } else if !ger.pausado {
            contagem_frames += 1;
        }

        term.draw(|f| f.render_widget(&ger, f.area()))?;

        if ger.parar {
            break;
        }

        let tempo_restante = DURACAO_FRAME.saturating_sub(inicio_frame.elapsed());
        if event::poll(tempo_restante)? {
            ger.processar_input(event::read()?);
        }
    }

    crossterm::execute!(std::io::stdout(), EnterAlternateScreen)?;

    Ok(())
}
