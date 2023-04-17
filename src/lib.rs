use wasm_bindgen::prelude::*;
pub mod logging;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub struct GeoLocationBuilder {
    app_name: String,
    app_version: String,
}

#[wasm_bindgen]
pub struct Session {
    user_id: String,
    session_id: String,
    user_auth_token: String,
}

#[wasm_bindgen]
pub struct AppDomain {
    product: String,
    region: String,
}

pub struct Environment {
    typ: String,
}

#[wasm_bindgen]
pub struct GeoLocationConfig {
    client_auth_token: String,
    installer_key: String,
    app_version: String,
    app_name: String,
    environment: Environment,
    app_domain: AppDomain,
    session: Session,
}

#[wasm_bindgen]
pub struct ConfiguredGeoLocation {
    app: GeoLocationBuilder,
    config: GeoLocationConfig,
    callback: Option<js_sys::Function>,
}

#[wasm_bindgen]
impl GeoLocationBuilder {
    pub fn new(app_name: String, app_version: String) -> Self {
        GeoLocationBuilder {
            app_name,
            app_version,
        }
    }
    pub fn configure(self, config: GeoLocationConfig) -> ConfiguredGeoLocation {
        ConfiguredGeoLocation {
            app: self,
            config,
            callback: None,
        }
    }
}

#[wasm_bindgen]
impl ConfiguredGeoLocation {
    pub fn set_callback(mut self, callback: js_sys::Function) -> Self {
        self.callback = Some(callback);
        self
    }
    pub fn place_bet(self, reason: String, force: bool) -> Self {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::{AppDomain, Environment, GeoLocationBuilder, GeoLocationConfig, Session};

    #[test]
    fn config_set() {
        let c = GeoLocationConfig {
            client_auth_token: "client_auth_token".into(),
            app_domain: AppDomain {
                product: "product".into(),
                region: "region".into(),
            },
            app_name: "app_name".into(),
            app_version: "app_version".into(),
            environment: Environment { typ: "type".into() },
            installer_key: "installer_key".into(),
            session: Session {
                user_id: "user_id".into(),
                session_id: "session_id".into(),
                user_auth_token: "auth_token".into(),
            },
        };

        let res = GeoLocationBuilder::new("appname".into(), "1.0".into())
            .configure(c)
            .place_bet("reason".into(), false);
        assert_eq!(res.config.app_name, "app_name");
    }
}
