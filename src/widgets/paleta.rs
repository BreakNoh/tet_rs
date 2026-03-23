use std::collections::HashMap;

use ratatui::style::Style;

use crate::core::peca::Bloco;

pub trait Paleta {
    fn estilo_padrao(&self) -> Style {
        Style::default()
    }

    fn estilo_de(&self, bloco: Bloco) -> Style;
}

pub struct PaletaPadrao {
    estilos: HashMap<Bloco, Style>,
}
use crate::core::peca::pecas::blocos::*;

impl PaletaPadrao {
    fn sem_fundo() -> Self {
        PaletaPadrao {
            estilos: HashMap::from([
                (T, Style::new().magenta().not_dim()),
                (I, Style::new().cyan().not_dim()),
                (O, Style::new().yellow().not_dim()),
                (L, Style::new().light_red().not_dim()),
                (J, Style::new().blue().not_dim()),
                (S, Style::new().green().not_dim()),
                (Z, Style::new().red().not_dim()),
                (PREVIA, Style::new().dim()),
            ]),
        }
    }
    fn com_fundo() -> Self {
        PaletaPadrao {
            estilos: HashMap::from([
                (T, Style::new().light_magenta().on_magenta().not_dim()),
                (I, Style::new().light_cyan().on_cyan().not_dim()),
                (O, Style::new().light_yellow().on_yellow().not_dim()),
                (L, Style::new().light_yellow().on_light_red().not_dim()),
                (J, Style::new().light_blue().on_blue().not_dim()),
                (S, Style::new().light_green().on_green().not_dim()),
                (Z, Style::new().light_red().on_red().not_dim()),
                (PREVIA, Style::new().dim()),
            ]),
        }
    }
    fn teste() -> Self {
        PaletaPadrao {
            estilos: HashMap::from([
                (T, Style::new().black().on_magenta().not_dim()),
                (I, Style::new().black().on_cyan().not_dim()),
                (O, Style::new().black().on_yellow().not_dim()),
                (L, Style::new().black().on_light_red().not_dim()),
                (J, Style::new().black().on_blue().not_dim()),
                (S, Style::new().black().on_green().not_dim()),
                (Z, Style::new().black().on_red().not_dim()),
                (PREVIA, Style::new().dim()),
            ]),
        }
    }
}

impl Default for PaletaPadrao {
    fn default() -> Self {
        PaletaPadrao::com_fundo()
    }
}

impl Paleta for PaletaPadrao {
    fn estilo_de(&self, bloco: Bloco) -> Style {
        self.estilos
            .get(&bloco)
            .cloned()
            .unwrap_or(self.estilo_padrao())
    }
}
