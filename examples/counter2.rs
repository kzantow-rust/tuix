extern crate tuix;
use tuix::*;

use tuix::button::Button;

static THEME: &'static str = include_str!("themes/counter_theme.css");

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CounterMessage {
    Increment,
    Decrement,
}

#[derive(Default)]
struct Counter {
    value: i32,
    label: Entity,
}

impl Counter {
    pub fn set_initial_value(mut self, val: i32) -> Self {
        self.value = val;
        self
    }
}

impl Widget for Counter {
    type Ret = Entity;

    // Build
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        Button::with_label("increment")
            //.on_press(Event::new(CounterMessage::Increment))
            .on_test(|button, state, button_entity| {
                state.insert_event(Event::new(CounterMessage::Increment).target(button_entity));
                println!("Test: {}", self.value.clone());
            })
            .build(state, entity, |builder| builder.class("increment"));

        Button::with_label("decrement")
            .on_press(Event::new(CounterMessage::Decrement))
            .build(state, entity, |builder| builder.class("decrement"));

        self.label = Label::new(&self.value.to_string()).build(state, entity, |builder| builder);

        entity.set_element(state, "counter")
    }

    // Events
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(counter_event) = event.message.downcast::<CounterMessage>() {
            match counter_event {
                CounterMessage::Increment => {
                    self.value += 1;
                    self.label.set_text(state, &self.value.to_string());
                }

                CounterMessage::Decrement => {
                    self.value -= 1;
                    self.label.set_text(state, &self.value.to_string());
                }
            }
        }  
    }
}

fn main() {
    // Create the app
    let app = Application::new(|win_desc, state, window| {
        state.add_theme(THEME);

        Counter::default()
            // Set local state
            .set_initial_value(50)
            // Build the widget
            .build(state, window, |builder| builder);

        // Set the window title
        win_desc.with_title("Counter").with_inner_size(400, 100)
    });

    app.run();
}
