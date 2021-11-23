use clap::Parser;

#[derive(Parser)]
pub struct Config {
    #[clap(
        long,
        default_value = "localhost",
        env = "HOST",
        about = "The host the server will listen."
    )]
    pub server_host: String,
    #[clap(
        long,
        default_value = "3000",
        env = "PORT",
        about = "The port the server will listen."
    )]
    pub server_port: String,
}

impl Config {
    pub fn server_address(&self) -> String {
        format!("{}:{}", self.server_host, self.server_port)
    }
}

#[cfg(test)]
mod tests {
    use super::Config;

    #[test]
    fn create_server_address() {
        let cfg = Config {
            server_host: "foo".to_owned(),
            server_port: "1234".to_owned(),
        };
        assert_eq!(cfg.server_address(), "foo:1234");
    }
}
