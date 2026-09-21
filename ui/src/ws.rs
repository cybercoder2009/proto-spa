use common::ws::ServerMsg;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{CloseEvent, ErrorEvent, MessageEvent, WebSocket};

/// 建立与后端的 WebSocket 连接
pub struct WsService {
    ws: WebSocket,
    _on_open: Closure<dyn FnMut()>,
    _on_message: Closure<dyn FnMut(MessageEvent)>,
    _on_error: Closure<dyn FnMut(ErrorEvent)>,
    _on_close: Closure<dyn FnMut(CloseEvent)>,
}

impl WsService {
    pub fn connect(token: &str) -> Result<Self, String> {
        let window = web_sys::window().ok_or("No window object")?;
        let location = window.location();
        let host = location.host().map_err(|e| format!("{:?}", e))?;
        let protocol = location.protocol().map_err(|e| format!("{:?}", e))?;
        let ws_protocol = if protocol == "https:" { "wss:" } else { "ws:" };

        let url = format!("{}//{}/ws?token={}", ws_protocol, host, token);
        let ws = WebSocket::new(&url).map_err(|e| format!("Failed to create WebSocket: {:?}", e))?;

        // 连接成功
        let on_open = Closure::wrap(Box::new(move || {
            web_sys::console::log_1(&"[WS] Connected successfully".into());
        }) as Box<dyn FnMut()>);
        ws.set_onopen(Some(on_open.as_ref().unchecked_ref()));

        // 接收消息：解析服务端推送的 ServerMsg
        let on_message = Closure::wrap(Box::new(move |e: MessageEvent| {
            if let Ok(text) = e.data().dyn_into::<js_sys::JsString>() {
                let text_str: String = text.into();
                web_sys::console::log_1(&format!("[WS] Received: {}", text_str).into());
                if let Ok(ServerMsg::Echo(payload)) = serde_json::from_str::<ServerMsg>(&text_str) {
                    web_sys::console::log_1(&format!("[WS] Server pushed echo: {}", payload).into());
                }
            }
        }) as Box<dyn FnMut(MessageEvent)>);
        ws.set_onmessage(Some(on_message.as_ref().unchecked_ref()));

        // 异常
        let on_error = Closure::wrap(Box::new(move |e: ErrorEvent| {
            web_sys::console::error_1(&format!("[WS] Error: {:?}", e.message()).into());
        }) as Box<dyn FnMut(ErrorEvent)>);
        ws.set_onerror(Some(on_error.as_ref().unchecked_ref()));

        // 关闭
        let on_close = Closure::wrap(Box::new(move |e: CloseEvent| {
            web_sys::console::log_1(&format!("[WS] Closed: code={}, reason={}", e.code(), e.reason()).into());
        }) as Box<dyn FnMut(CloseEvent)>);
        ws.set_onclose(Some(on_close.as_ref().unchecked_ref()));

        Ok(Self {
            ws,
            _on_open: on_open,
            _on_message: on_message,
            _on_error: on_error,
            _on_close: on_close,
        })
    }
}

impl Drop for WsService {
    fn drop(&mut self) {
        let _ = self.ws.close();
    }
}
