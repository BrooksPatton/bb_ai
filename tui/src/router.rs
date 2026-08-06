use anathema::state::Value;

#[derive(Debug, Copy, Clone)]
pub enum Route {
    Home,
    ModelChooser,
}

impl Route {
    pub fn as_value(&self) -> Value<String> {
        let route: String = self.clone().into();

        Value::new(route)
    }
}

impl From<Route> for String {
    fn from(value: Route) -> Self {
        match value {
            Route::Home => "Home",
            Route::ModelChooser => "Changing Model",
        }
        .to_owned()
    }
}
