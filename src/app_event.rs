use crate::data_event::DataEvent;
use derive_more::From;

#[derive(From)]
pub enum AppEvent {
	#[from]
	TermEvent(crossterm::event::Event),

	#[from]
	DataEvent(DataEvent),
}
