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

# Todo

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

