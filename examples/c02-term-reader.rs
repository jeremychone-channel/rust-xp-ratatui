use crossterm::event::{Event, EventStream, KeyCode, KeyEventKind};
use futures::{FutureExt, StreamExt};
use futures_timer::Delay;
use ratatui::prelude::CrosstermBackend;
use ratatui::style::Stylize;
use ratatui::widgets::Paragraph;
use ratatui::{DefaultTerminal, Terminal};
use std::io::Stdout;
use std::time::Duration;
use tokio::select;
use tokio::sync::mpsc::Sender;
use tokio::sync::mpsc::{Receiver, channel};
use tokio::task::JoinHandle;

pub type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() -> Result<()> {
	// -- init terminal
	let terminal = ratatui::init();

	let _ = exec_app(terminal).await;

	ratatui::restore();

	Ok(())
}

async fn exec_app(mut terminal: Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
	terminal.clear()?;

	// -- Create term channels
	let (term_tx, term_rx) = channel::<Event>(100);

	// -- Running the term_reader tasks
	let tin_read_handle = run_term_read(term_tx.clone())?;

	// -- Running Tui application
	let tui_handle = run_tui(terminal, term_rx)?;

	tui_handle.await?;
	tin_read_handle.await?;

	Ok(())
}

fn run_tui(mut terminal: DefaultTerminal, mut term_rx: Receiver<Event>) -> Result<JoinHandle<()>> {
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

			if let Some(Event::Key(key)) = term_rx.recv().await
				&& key.kind == KeyEventKind::Press
				&& key.code == KeyCode::Char('q')
			{
				return;
			}
		}
	});

	Ok(handle)
}

pub fn run_term_read(term_tx: Sender<Event>) -> Result<JoinHandle<()>> {
	let handle = tokio::spawn(async move {
		let mut reader = EventStream::new();

		loop {
			let delay = Delay::new(Duration::from_millis(200)).fuse();
			let event = reader.next().fuse();

			select! {
				_ = delay => {  },
				maybe_event = event => {
					match maybe_event {
						Some(Ok(event)) => {
							if let Err(err) = term_tx.send(event).await {
								println!("Cannot send term_txt.send {err}");
								break;
							}
						}
						Some(Err(e)) => println!("Error: {e:?}\r"),
						None => break,
					}
				}
			};
		}
	});

	Ok(handle)
}
