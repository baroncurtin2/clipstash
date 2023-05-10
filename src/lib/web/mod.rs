pub use hitcounter::HitCounter;

pub mod api;
pub mod ctx;
pub mod form;
pub mod hitcounter;
pub mod http;
pub mod renderer;

pub const PASSWORD_COOKIE: &str = "password";

#[derive(rocket::Responder)]
pub enum PageError {
    #[response(status = 500)]
    Serialization(String),
    #[response(status = 500)]
    Render(String),
    #[response(status = 404)]
    NotFound(String),
    #[response(status = 500)]
    Internal(String),
}

impl From<handlebars::RenderError> for PageError {
    fn from(err: handlebars::RenderError) -> Self {
        PageError::Render(format!("{}", err))
    }
}

impl From<serde_json::Error> for PageError {
    fn from(err: serde_json::Error) -> Self {
        PageError::Serialization(format!("{}", err))
    }
}

#[cfg(test)]
pub mod test {
    use rocket::local::blocking::Client;

    use crate::test::async_runtime;
    use crate::web::renderer::Renderer;
    use crate::web::HitCounter;
    use crate::RocketConfig;

    pub fn config() -> RocketConfig {
        let rt = async_runtime();
        let renderer = Renderer::new("templates/".into());
        let database = crate::data::test::new_db(rt.handle());
        let maintenance = crate::domain::maintenance::Maintenance::spawn(
            database.get_pool().clone(),
            rt.handle().clone(),
        );
        let hit_counter = HitCounter::new(database.get_pool().clone(), rt.handle().clone());

        RocketConfig {
            renderer,
            database,
            hit_counter,
            maintenance,
        }
    }

    pub fn client() -> Client {
        let config = config();

        Client::tracked(crate::rocket(config)).expect("failed to build rocket instance")
    }
}
