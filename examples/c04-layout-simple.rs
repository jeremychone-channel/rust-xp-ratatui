use crossterm::event::{self, KeyCode, KeyEventKind};
use ratatui::DefaultTerminal;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::Stylize;
use ratatui::widgets::{Block, Paragraph};
use std::io;

// Layout doc: https://ratatui.rs/concepts/layout/

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
			let layout = Layout::default()
				.direction(Direction::Vertical)
				.constraints(vec![Constraint::Length(3), Constraint::Percentage(100)])
				.split(frame.area());

			// -- Add header
			let session_header_block = Block::bordered().title(" Session ");
			let header_v = Paragraph::new(format!("Some session information ({c})"))
				.white()
				.block(session_header_block);
			frame.render_widget(header_v, layout[0]);

			// -- Add body
			let body_block = Block::bordered().title(" Runs "); // By default in the top left corner
			frame.render_widget(body_block, layout[1]);
		})?;

		if let event::Event::Key(key) = event::read()?
			&& key.kind == KeyEventKind::Press
			&& key.code == KeyCode::Char('q')
		{
			return Ok(());
		}
	}
}
