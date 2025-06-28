use crate::data_event::DataEvent;
use derive_more::From;

/// The main application event enum.
///
/// This enum encapsulates all possible events that can occur in the application,
/// serving as a central point for event handling. It includes terminal UI events
/// and custom application-specific data events.
///
/// The `#[derive(From)]` allows for convenient conversion from its variant types
/// into `AppEvent`.
#[derive(From)]
pub enum AppEvent {
	// -- UI Events
	#[from]
	TermEvent(crossterm::event::Event),

	// -- Data Events
	#[from]
	DataEvent(DataEvent),
}
