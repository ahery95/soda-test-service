use crate::domain::AppState;
use hyper::{Method, Request, Response,Error,service::Service};
use futures::{future, Future,};

pub struct Auth;

impl Service <AppState> for Auth {

    fn call(&self, req: &Request<AppState>) -> Self::Future {
        // The credentials are configured in the main.
        let auth_user = &req.state().auth_user;
        let auth_pwd = &req.state().auth_pwd;

        // If the username and the password are empty, we consider that the test service has
        // been configured to run without the Basic-Auth.
        if auth_user.is_empty() && auth_pwd.is_empty() {
            return Ok(Started::Done);
        }

        // The realm configuration.
        let mut config = Config::default();
        config.realm("SODA Test Service");

        let auth = BasicAuth::from_request(&req, &config)?;

        // Check auth information.
        if auth.username() == auth_user && auth.password() == Some(&auth_pwd) {
            Ok(Started::Done)
        } else {
            Err(AuthenticationError::from(config).into())
        }
    }
}
