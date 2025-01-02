use log::info;
use proxy_wasm::traits::*;
use proxy_wasm::types::*;

proxy_wasm::main! {{
    proxy_wasm::set_log_level(LogLevel::Trace);
    proxy_wasm::set_root_context(|_| -> Box<dyn RootContext> {
        Box::new(ABTestingHeaderRoot)
    });
}}

struct Config {
    header_name: String,
    header_value_control: String,
    header_value_variation: String,
    variation_weight: i8,
}

struct ABTestingHeader;

impl Context for ABTestingHeader {}

impl HttpContext for ABTestingHeader {}

struct ABTestingHeaderRoot;

impl Context for ABTestingHeaderRoot {}

impl RootContext for ABTestingHeaderRoot {
    fn on_configure(&mut self, _: usize) -> bool {
        if let Some(config_bytes) = self.get_plugin_configuration() {
            // TODO: impl
        }
        true
    }

    fn create_http_context(&self, _: u32) -> Option<Box<dyn HttpContext>> {
        Some(Box::new(ABTestingHeader))
    }

    fn get_type(&self) -> Option<ContextType> {
        Some(ContextType::HttpContext)
    }
}
