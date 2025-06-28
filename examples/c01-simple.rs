use crossterm::event::{self, KeyCode, KeyEventKind};
use ratatui::style::Stylize;
use ratatui::widgets::Paragraph;
use ratatui::DefaultTerminal;
use std::io;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	println!("Hello, world!");

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
			let greeting = Paragraph::new(format!("Hello Ratatui! (press 'q' to quit) ({c})"))
				.white()
				.on_blue();
			frame.render_widget(greeting, frame.area());
		})?;

		if let event::Event::Key(key) = event::read()? {
			if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
				return Ok(());
			}
		}
	}
}
