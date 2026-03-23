use gpui::{Render, div};

pub struct Parlour {
    
}

impl Render for Parlour {
    fn render(&mut self, window: &mut gpui::Window, cx: &mut gpui::Context<Self>) -> impl gpui::IntoElement {
        div()
    }
}
