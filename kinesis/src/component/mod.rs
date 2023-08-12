mod controller;
mod identifier;

pub use controller::*;
use web_sys::Event;

pub use self::identifier::Identifier;
use crate::dom::{renderable::Renderable, EventType};

/// Possible render types when rendering a component.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum RenderType {
    /// Render the root portion of the component.
    Root,

    /// Render a partial piece of the component, generally in response to the component being
    /// updated.
    Partial(usize),
}

/// Trait that represents a renderable component
pub trait Component {
    /// Handle an incomming event, allowing for mutation of the component's state.
    fn handle_event(
        &mut self,
        id: Identifier,
        event_type: EventType,
        event: Event,
    ) -> Option<Vec<usize>>;

    /// Renders the component for a given state. Can optionally not render anything.
    fn render(&self, update_type: RenderType) -> Option<Vec<Box<dyn Renderable>>>;
}
