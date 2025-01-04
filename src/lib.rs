use core::fmt;

use log::{debug, info};
use proxy_wasm::traits::*;
use proxy_wasm::types::*;
use serde::Deserialize;

proxy_wasm::main! {{
    proxy_wasm::set_log_level(LogLevel::Trace);
    proxy_wasm::set_root_context(|_| -> Box<dyn RootContext> {
        Box::new(ABTestingHeaderRoot {
            config: Box::new(Config {
                header_name: String::new(),
                header_value_control: String::new(),
                header_value_variation: String::new(),
                variation_weight: 0,
            })
        })
    });
}}

#[derive(Clone, Deserialize)]
struct Config {
    header_name: String,
    header_value_control: String,
    header_value_variation: String,
    variation_weight: u8,
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "header_name: {}, header_value_control: {}, header_value_variation: {}, variation_weight: {}\n", self.header_name, self.header_value_control, self.header_value_variation, self.variation_weight)
    }
}

struct ABTestingHeader {
    config: Box<Config>,
}

impl Context for ABTestingHeader {}

impl HttpContext for ABTestingHeader {
    fn on_http_request_headers(&mut self, _num_headers: usize, _end_of_stream: bool) -> Action {
        let mut buf = [0u8; 1];
        getrandom::getrandom(&mut buf).unwrap();
        debug!("getrandom {:?}", buf[0]);

        let is_using_variation = if buf[0] >= 100 {
            let rand = buf[0] % 100;
            rand < self.config.variation_weight
        } else {
            buf[0] < self.config.variation_weight
        };

        let header_value = if is_using_variation {
            self.config.header_value_variation.as_str()
        } else {
            self.config.header_value_control.as_str()
        };

        debug!(
            "add http request header {}:{}",
            self.config.header_name, header_value
        );
        self.add_http_request_header(self.config.header_name.as_str(), header_value);
        Action::Continue
    }
}

struct ABTestingHeaderRoot {
    config: Box<Config>,
}

impl Context for ABTestingHeaderRoot {}

impl RootContext for ABTestingHeaderRoot {
    fn on_configure(&mut self, _: usize) -> bool {
        if let Some(config_bytes) = self.get_plugin_configuration() {
            info!("config_bytes: {:?}", &config_bytes);
            self.config = serde_json::from_slice(&config_bytes).unwrap();
            info!("Config {}", self.config);
        }
        true
    }

    fn create_http_context(&self, _: u32) -> Option<Box<dyn HttpContext>> {
        Some(Box::new(ABTestingHeader {
            config: self.config.clone(),
        }))
    }

    fn get_type(&self) -> Option<ContextType> {
        Some(ContextType::HttpContext)
    }
}
