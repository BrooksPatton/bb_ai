use std::fmt::Display;

#[derive(Debug, Default)]
pub enum Route {
    Splash,
    Home,
    #[default]
    Config,
}

impl Display for Route {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let route = match self {
            Route::Splash => "Splash",
            Route::Home => "Home",
            Route::Config => "Config",
        };

        write!(f, "{route}")
    }
}
