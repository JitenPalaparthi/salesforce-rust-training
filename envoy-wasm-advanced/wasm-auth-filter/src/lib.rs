
use proxy_wasm::traits::*;
use proxy_wasm::types::*;
use std::collections::HashMap;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};


const AUTH_CLUSTER: &str = "auth_service";
const CACHE_TTL_SECS: u64 = 30;

static mut CACHE: Option<HashMap<String, (bool, u64)>> = None;

fn cache_get(token: &str, now: u64) -> Option<bool> {
    unsafe {
        let m = CACHE.get_or_insert_with(HashMap::new);
        if let Some((allowed, exp)) = m.get(token) {
            if now <= *exp {
                return Some(*allowed);
            }
        }
        None
    }
}

fn cache_put(token: String, allowed: bool, now: u64) {
    unsafe {
        let m = CACHE.get_or_insert_with(HashMap::new);
        m.insert(token, (allowed, now + CACHE_TTL_SECS));
    }
}

fn now_unix_secs() -> u64 {
    proxy_wasm::hostcalls::get_current_time()
        .ok()
        .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

proxy_wasm::main! {{
    proxy_wasm::set_log_level(LogLevel::Info);
    proxy_wasm::set_root_context(|_| Box::new(Root));
}}

struct Root;
impl Context for Root {}

impl RootContext for Root {
    fn get_type(&self) -> Option<ContextType> {
        Some(ContextType::HttpContext)
    }
    fn create_http_context(&self, _context_id: u32) -> Option<Box<dyn HttpContext>> {
        Some(Box::new(Filter {
            pending_callout_id: None,
            pending_token: None,
        }))
    }
}

struct Filter {
    pending_callout_id: Option<u32>,
    pending_token: Option<String>,
}

impl Context for Filter {
    fn on_http_call_response(
        &mut self,
        token_id: u32,
        _num_headers: usize,
        _body_size: usize,
        _num_trailers: usize,
    ) {
        if self.pending_callout_id != Some(token_id) {
            return;
        }

        let status = proxy_wasm::hostcalls::get_map(MapType::HttpCallResponseHeaders)
            .ok()
            .and_then(|hdrs| hdrs.into_iter().find(|(k, _)| k == ":status").map(|(_, v)| v))
            .unwrap_or_else(|| "500".to_string());

        let allowed = status.starts_with("2");

        if let Some(tok) = self.pending_token.take() {
            cache_put(tok, allowed, now_unix_secs());
        }

        if allowed {
            proxy_wasm::hostcalls::resume_http_request().ok();
        } else {
            proxy_wasm::hostcalls::send_http_response(
                401,
                vec![("content-type", "text/plain")],
                Some(b"Unauthorized (blocked by Wasm auth filter)\n"),
            )
            .ok();
        }
    }
}

impl HttpContext for Filter {
    fn on_http_request_headers(&mut self, _num_headers: usize, _end_of_stream: bool) -> Action {
        let token = self.get_http_request_header("authorization")
            .unwrap_or_default();

        if token.is_empty() {
            proxy_wasm::hostcalls::send_http_response(
                401,
                vec![("content-type", "text/plain")],
                Some(b"Missing Authorization header\n"),
            )
            .ok();
            return Action::Pause;
        }

        let now = now_unix_secs();
        if let Some(allowed) = cache_get(&token, now) {
            if allowed {
                self.set_http_request_header("x-auth-cache", Some("hit"));
                return Action::Continue;
            } else {
                proxy_wasm::hostcalls::send_http_response(
                    401,
                    vec![("content-type", "text/plain")],
                    Some(b"Unauthorized (cached)\n"),
                )
                .ok();
                return Action::Pause;
            }
        }

        self.set_http_request_header("x-auth-cache", Some("miss"));

        let headers = vec![
            (":method", "GET"),
            (":path", "/check"),
            (":authority", "auth"),
            ("authorization", token.as_str()),
        ];

        let callout_id = self
            .dispatch_http_call(
                AUTH_CLUSTER,
                headers,
                None,
                vec![],
                Duration::from_millis(500),
            )
            .unwrap();

        self.pending_callout_id = Some(callout_id);
        self.pending_token = Some(token);

        Action::Pause
    }
}
