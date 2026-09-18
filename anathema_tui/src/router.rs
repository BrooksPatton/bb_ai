use std::fmt::Display;

#[derive(Debug, Default)]
pub enum Route {
    #[default]
    Splash,
    Home,
}

impl Display for Route {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let route = match self {
            Route::Splash => "Splash",
            Route::Home => "Home",
        };

        write!(f, "{route}")
    }
}
