use crate::app_event::AppEvent;
use crate::Result;
use crossterm::event::{Event, EventStream};
use futures::{FutureExt, StreamExt};
use futures_timer::Delay;
use std::time::Duration;
use tokio::select;
use tokio::sync::mpsc::Sender;
use tokio::task::JoinHandle;

pub fn run_tin_read(app_tx: Sender<AppEvent>) -> Result<JoinHandle<()>> {
	let handle = tokio::spawn(async move {
		let mut reader = EventStream::new();

		loop {
			let mut delay = Delay::new(Duration::from_millis(200)).fuse();
			let mut event = reader.next().fuse();

			select! {
				_ = delay => {  },
				maybe_event = event => {
					match maybe_event {
						Some(Ok(event)) => {
							if app_tx.send(event.into()).await.is_err() {
								break;
							}
						}
						Some(Err(e)) => println!("Error: {:?}\r", e),
						None => break,
					}
				}
			};
		}
	});

	Ok(handle)
}
