use anathema::component::Component;

pub static NAME: &str = "top_nav";

pub struct TopNav;

impl Component for TopNav {
    type State = ();

    type Message = ();
}
