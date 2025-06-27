use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::prelude::CrosstermBackend;
use ratatui::style::Stylize;
use ratatui::widgets::Paragraph;
use ratatui::{DefaultTerminal, Terminal};
use std::io::{self, Stdout};
use tokio::sync::mpsc::{channel, Receiver};
use tokio::task::JoinHandle;
use xp_ratatui::app_event::AppEvent;
use xp_ratatui::tin_reader::run_tin_read;
use xp_ratatui::Result;

#[tokio::main]
async fn main() -> Result<()> {
	// -- init terminal
	let mut terminal = ratatui::init();

	exec_app(terminal).await;

	ratatui::restore();

	Ok(())
}

async fn exec_app(mut terminal: Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
	terminal.clear()?;

	// -- Create channels
	let (app_tx, app_rx) = channel::<AppEvent>(100);

	// -- Start the tin_reader
	let tin_read_handle = run_tin_read(app_tx.clone())?;

	// -- Run Tui application
	let tui_handle = run_tui(terminal, app_rx)?;

	tui_handle.await;
	// tin_read_handle.await;

	Ok(())
}

fn run_tui(mut terminal: DefaultTerminal, mut app_rx: Receiver<AppEvent>) -> Result<JoinHandle<()>> {
	let handle = tokio::spawn(async move {
		let mut c = 0;
		loop {
			c += 1;
			terminal
				.draw(|frame| {
					let greeting = Paragraph::new(format!("Hello Ratatui! (press 'q' to quit) ({c})"))
						.white()
						.on_blue();
					frame.render_widget(greeting, frame.area());
				})
				.expect("cannot draw in terminal");

			match app_rx.recv().await {
				Some(AppEvent::TermEvent(Event::Key(key))) => {
					if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
						return;
					}
				}
				Some(AppEvent::DataEvent(data_event)) => {
					println!("DataEvent {data_event:?}")
				}
				None => (),
				_ => (),
			}
		}
	});

	Ok(handle)
}
