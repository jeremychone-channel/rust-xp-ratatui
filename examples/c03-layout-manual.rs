use crossterm::event::{self, KeyCode, KeyEventKind};
use ratatui::layout::Rect;
use ratatui::style::Stylize;
use ratatui::widgets::{Block, Paragraph};
use ratatui::DefaultTerminal;
use std::io;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let mut terminal = ratatui::init();
	terminal.clear()?;
	let _app_result = run(terminal);
	ratatui::restore();

	Ok(())
}

fn run(mut terminal: DefaultTerminal) -> io::Result<()> {
	let mut c = 0;
	loop {
		c += 1;
		terminal.draw(|frame| {
			let frame_width = frame.area().width;
			let frame_height = frame.area().height;

			// -- Add header
			let last_el_height = 3;
			let session_header_area = Rect::new(0, 0, frame_width, last_el_height);
			let session_header_block = Block::bordered().title("Session").on_red();
			let header_v = Paragraph::new(format!("Counter ({c})"))
				.white()
				.on_blue()
				.block(session_header_block);
			frame.render_widget(header_v, session_header_area);

			// -- Add body
			let body_area = Rect::new(0, last_el_height, frame_width, frame_height - last_el_height - 1);
			let body_block = Block::bordered().title("This is a big title"); // By default in the top left corner

			frame.render_widget(body_block, body_area);
		})?;

		if let event::Event::Key(key) = event::read()? {
			if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
				return Ok(());
			}
		}
	}
}
