use crossterm::event::{self, KeyCode, KeyEventKind};
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::Stylize;
use ratatui::widgets::{Block, List, ListItem, Paragraph, Widget};
use ratatui::DefaultTerminal;
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
	loop {
		terminal.draw(|frame| {
			// -- Layout
			let layout = Layout::default()
				.direction(Direction::Vertical)
				.constraints(vec![Constraint::Length(3), Constraint::Percentage(100)])
				.split(frame.area());
			let header_a = layout[0];
			let body_a = layout[1];

			// -- Add header
			let session_w = SessionWidget;
			frame.render_widget(session_w, header_a);

			// -- Add body
			let runs_w = RunsWidget::new();
			frame.render_widget(runs_w, body_a);
		})?;

		if let event::Event::Key(key) = event::read()? {
			if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
				return Ok(());
			}
		}
	}
}

// region:    --- Session Widget

pub struct SessionWidget;

impl Widget for SessionWidget {
	fn render(self, area: Rect, buf: &mut Buffer) {
		let session_header_block = Block::bordered().title(" Session ");
		let header_v = Paragraph::new("Some session information ()".to_string())
			.white()
			.block(session_header_block);

		header_v.render(area, buf);
	}
}
// endregion: --- Session Widget

// region:    --- Runs Widget

pub struct RunsWidget {
	items: Vec<&'static str>,
}

impl RunsWidget {
	#[allow(clippy::new_without_default)]
	pub fn new() -> Self {
		let items = vec!["Item 0", "Item 1", "Item 2"];
		RunsWidget { items }
	}
}

impl Widget for RunsWidget {
	fn render(self, area: Rect, buf: &mut Buffer) {
		// -- Draw The outer block
		let runs_block = Block::bordered().title(" Runs ");
		runs_block.render(area, buf);

		// -- Make the main content layout
		let main_layout = Layout::default()
			.direction(Direction::Horizontal)
			.constraints(vec![Constraint::Percentage(25), Constraint::Percentage(75)])
			.margin(1)
			.split(area);
		let nav_a = main_layout[0];
		let content_a = main_layout[1];

		// -- Display the list/nav
		let items: Vec<ListItem> = self.items.into_iter().map(ListItem::from).collect();
		let list_block = Block::new();
		let list_w = List::new(items).block(list_block);
		list_w.render(nav_a, buf);

		// -- Display the Content block
		let content_block = Block::bordered().title(" Content ");
		content_block.render(content_a, buf);
	}
}

// endregion: --- Runs Widget
