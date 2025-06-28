use crate::app_event::AppEvent;
use crate::Result;
use crossterm::event::EventStream;
use futures::{FutureExt, StreamExt};
use futures_timer::Delay;
use std::time::Duration;
use tokio::select;
use tokio::sync::mpsc::Sender;
use tokio::task::JoinHandle;

pub fn run_term_read(app_tx: Sender<AppEvent>) -> Result<JoinHandle<()>> {
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
							if let Err(_err) = app_tx.send(event.into()).await {
								println!("Cannot send app_txt.send");
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
