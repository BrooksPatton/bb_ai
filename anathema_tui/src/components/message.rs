use anathema::component::Component;

pub const NAME: &str = "message";

pub struct Message;

impl Component for Message {
    type State = ();

    type Message = ();
}
