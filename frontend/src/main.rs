use gpui::{App, AppContext, Application, Bounds, WindowBounds, WindowOptions, px, size};

use crate::work::Parlour;

mod work;

fn main() {
    Application::new().run(|cx: &mut App| {
	let bounds = Bounds::centered(None, size(px(500.), px(500.)), cx);
	cx.open_window(
	    WindowOptions {
		window_bounds: Some(WindowBounds::Windowed(bounds)),
		..Default::default()
	    },
	    |_, cx| {
		cx.new(|_| Parlour {
		})
	    }
	).unwrap();
    });
}
