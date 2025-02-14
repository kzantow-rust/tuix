use tuix::*;

static THEME: &'static str = include_str!("../themes/counter_theme.css");

#[derive(Default, Lens)]
pub struct SomeOtherState {
    value: String,
}

impl Model for SomeOtherState {
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        
    }
}

// The state of the application
#[derive(Default, Lens)]
pub struct CounterState {
    value: i32,
}

// Messages for mutating the application state
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CounterMessage {
    Increment,
    Decrement,
}

// The Model allows the state to be mutated in response to messages
impl Model for CounterState {
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(counter_event) = event.message.downcast() {
            match counter_event {
                CounterMessage::Increment => {
                    self.value += 1;
                    entity.emit(state, BindEvent::Update);
                    event.consume();
                }

                CounterMessage::Decrement => {
                    self.value -= 1;
                    entity.emit(state, BindEvent::Update);
                    event.consume();
                }
            }            
        }
    }
}

// A widget for the counter, with 2 buttons and a label
#[derive(Default)]
struct CounterWidget {}

impl Widget for CounterWidget {
    type Ret = Entity;
    type Data = ();

    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
  
        Button::with_label("increment")
            .on_press(|_, state, button|{
                button.emit(state, CounterMessage::Increment);
            })
            .build(state, entity, |builder| builder.class("increment"));

        Button::with_label("decrement")
            .on_press(|_, state, button|{
                button.emit(state,  CounterMessage::Decrement);
            })
            .build(state, entity, |builder| builder.class("decrement"));

        // Using a lens, the label is bound to the value field of the app data
        Label::new("0")
            .bind(CounterState::value, |value| value.to_string())
            .build(state, entity, |builder| builder);

        entity.set_element(state, "counter").set_layout_type(state, LayoutType::Row)
    }
}

fn main() {

    let window_description = WindowDescription::new().with_title("Counter").with_inner_size(400, 100);
    let app = Application::new(window_description, |state, window| {

        state.add_theme(THEME);

        // Build the app data at the root of the visual tree
        let data_widget = CounterState::default().build(state, window);

        let some_other_data = SomeOtherState::default().build(state, data_widget);

        CounterWidget::default()
            .build(state, some_other_data, |builder| builder);
        
        // Another label is bound to the counter value, but with a conversion closure 
        // which converts the value to english text form
        Label::new("Zero")
            .bind(CounterState::value, |value| english_numbers::convert_all_fmt(*value as i64))
            .build(state, data_widget, |builder| 
                builder
                    .set_width(Stretch(1.0))
                    .set_space(Pixels(5.0))
            );
    });

    app.run();
}