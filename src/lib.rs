use wasm_bindgen::prelude::*;
use wgpu::Instance;

#[wasm_bindgen]
pub struct Engine {
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface<'static>,
}

#[wasm_bindgen]
impl Engine {
    pub async fn new(canvas: web_sys::HtmlCanvasElement) -> Result<Engine, JsValue> {
        let instance = wgpu::Instance::default();
        
        // 修正：Web専用の変換を行うためのメソッドを使う
        // 最新版の wgpu では、Canvas自体を渡して変換する
        let surface = instance
            .create_surface_from_canvas(&canvas) // これが一番確実なパスだ！
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions::default())
            .await
            .ok_or_else(|| JsValue::from_str("Failed to find adapter"))?;
            
        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor::default(), None)
            .await
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        
        Ok(Self { device, queue, surface })
    }
}
