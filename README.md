An experimental attempt at a web framework for Rust (an equivalent to Leptos), with the intention
to make heavy use of Rust macros to compute the reactive dependencies rather than relying on the
developer manually specifying them or passing signals around. This first iteration focused on the
rendering of the nodes themselves, so doesn't contain any macros and instead is representative of
the 'output' of the macros where all of the dependencies are pre-defined. Despite this, it contains
a basic level of reactivity, including:

- Rendering text nodes

- Rendering basic DOM nodes (`p`, `button`, etc.)

- Specifying render dependencies and re-rendering upon dependency change

- Basic DOM events

A simple component would look as follows:

```rust
// Every component is a struct, which contains any required state within
pub struct Counter(usize);

impl Counter {
    // An 'instance' of a component is actually two things:
    // 1. the component state (`component`)
    // 2. instructions to render the component (`fragment`)
    // These are combined into the `ComponentWrapper` struct for convinience
    pub fn new() -> ComponentWrapper<Self> {
        // Initialise the component with it's state
        let component = Rc::new(RefCell::new(Self(0)));

        // Generate the instructions required to render the component
        let fragment = Fragment::build()
            // We want a `p` tag
            .with_element("p", None)

            // Place text inside the `p` tag (`Some(0)` indicates that it should have a parent, and
            // that parent's offset should be `0`)
            .with_text("Counter value: ", Some(0))

            // Place an updatable fragment, which depends on `&[0]` (that is, it depends on a piece
            // of state which is the `0`th value in the component state)
            .with_updatable(&[0], Some(0), {
                let component = Rc::clone(&component);

                // Return a callback function that will produce a new fragment according to the
                // current state whenever called
                move || {
                    let component = component.borrow();

                    // A simple fragment with a single text node
                    Fragment::build().with_text(component.0.to_string(), None)
                }
            })

            // Place a button node that responds to the `click` DOM event by emitting event ID `0`
            .with_node(Node::element("button").with_event("click", 0), None)
            .with_text("Clear", Some(3));

        ComponentWrapper::new(component, fragment)
    }
}

impl Component for Counter {
    // Every component must be able to respond to any incomming DOM events
    fn handle_event(&mut self, event_id: usize, _event: Event) -> Option<Vec<usize>> {
        match event_id {
            // If event `0` is encountered (as per the `click` event handler defined above)...
            0 => {
                // ...increment the state of the counter component...
                self.0 += 1;

                // ...and notify the runtime that dependency `0` has been modified, so that it can
                // propagate those changes
                Some(vec![0])
            }

            // All other events can be ignored
            _ => None,
        }
    }
}
```

# Element Rendering

## Initial Render

Renderable gets passed `None`, causing it to create a new element, before creating all of the
required fields and properties. It returns the created element, which the controller appends to
the parent.

## Partial update

Dynamic renderable is passed a reference to its element according to the identifier. The renderable
detects that it has one passed in, so won't create a new element. It will continue with assigning
all of the properties and content as required. **This will work with simple dynamic content**

### Optional Element

Initial render, nothing is returned. Update render results in an element being created. This
element is returned to the controller, which it must insert in the appropriate place (maybe use
identifier to determine appropriate element to append to or insert after).

### Looped Elements

Initial render results i some amount of elements being returned. On an update, the render method
will be called for each of the elements. They will be passed an element reference corresponding to
their identifier (won't work properly for items that are re-ordered).

# Wishlist

 - [ ] DOM
    - [ ] Nested components
        - [ ] Props
        - [ ] Bi-directional binding
    - [x] Arrays
        - [x] DOM elements
        - [ ] Other components
    - [x] Unique element identifiers
    - [x] Optional children
    - [ ] Dynamic children content
    - [ ] Render as sibling
    - [ ] Portals
    - [ ] Slots
       - [ ] Single
       - [ ] Named
 - [ ] State
    - [ ] Derived state
 - [ ] Scoped CSS support
 - [ ] Async/future support
 - [ ] Error handling
    - [ ] Make panics usable
    - [ ] See if it's possible to setup `dbg`, `println`, ect to use console
    - [ ] Use proper `Error` enums instead of `JsValue`
 - [ ] Long term stuff
    - [ ] JS component interop
       - Way to render regular JS components within Rust components
       - Custom Elements?
       - Pass an element handle to JS?
    - [ ] Accompanying server framework
       - [ ] SSR
       - [ ] Routing
       - [ ] ect...

# Requirements

 - [ ] Macros
     - [ ] Macro to parse HTML-like syntax into Rust builder pattern
     - [ ] Proc macro to create `handle_event` method
 - [ ] Way to create DOM elements from Rust
 - [ ] Way to send events to Rust

<details>

<summay>Ramblings</summary>

# The Goal

Turn this:

```rust
struct Counter {
    count: usize,
}

impl Counter {
    pub fn handle_button_click(&mut self, event: Event) {
        self.count += 1;
    }

    pub fn render(&self) -> Option<DomNode> {
        html! {
            <p>The current count is {self.count}</p>
            <button on:click={self.handle_button_click}>Click to count!</button>
        }
    }
}
```

Into this:

```rust
impl CounterFull {
    pub fn handle_event(&mut self, event: Event, element: usize) {
        match (event, element) {
            (Event::Click, 1) => {
                self.count += 1;
            }
            _ => (),
        }
    }

    pub fn render(&self) -> Option<DomNode> {
        Some(&[
            DomNode::p().text_content(format!("The current count is {}", self.count)),
            DomNode::button().text_content("Click to count!"),
        ])
    }
}
```

Dynamic content as renderable??????
Invert the whole dynamic thing. Dynamic implements Renderable, allowing for whole blocks to be
dynamically rendered. This could potentially include moving the component render method into
`handle_update`. The `handle_update` method can then return actual content to be rendered within
some container.
Dynamic content can be placed in two places: In as a node child, or as an attribute (if text)

 - [x] Move handle_update function into render function (with render type enum)
 - [ ] Change rendering of text_content to be a text node
 - [ ] Make dynamic content implement Renderable (only for node children)
 - [ ] Work out how to make dynamic content work for both nodes and attributes (Vec<Box<dyn
       Renderable>> vs String)

**Note:** For a text node, it should only be a static value, and should be wrapped in a dynamic
renderable if it needs to change. Should find a good way to represent this in the type system.

Dynamic components: For every renderable, save the nodes that it produced. These can then be used
when re-rendering, so that it can be rendered in place and correctly remove the existing nodes.

</details>
