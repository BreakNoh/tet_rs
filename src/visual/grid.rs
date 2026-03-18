use crate::grid::Grid;
use ratatui::widgets::Widget;

impl Widget for Grid {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {}
}
