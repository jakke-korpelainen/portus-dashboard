use once_cell::sync::Lazy;
use std::env;

pub struct Config {
    pub digitransit_subscription_key: String,
}

pub fn get_env_value(env_name: &str) -> String {
    let env_value = match env::var(env_name) {
        Ok(val) => {
            println!("{env_name}: {val:?}");
            val
        }
        Err(e) => {
            println!("couldn't interpret {env_name}: {e}");
            "".to_string()
        }
    };
    env_value
}

const ENV_DIGITRANSIT_SUBSCRIPTION_KEY_NAME: &str = "DIGITRANSIT_SUBSCRIPTION_KEY";

pub static CONFIG: Lazy<Config> = Lazy::new(|| Config {
    digitransit_subscription_key: get_env_value(ENV_DIGITRANSIT_SUBSCRIPTION_KEY_NAME),
});
