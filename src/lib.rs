use wasm_bindgen::prelude::*;
use wgpu::SurfaceTarget; // 重要！

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
        
        // 修正ポイント：Instance::create_surface に SurfaceTarget を渡す
        let surface = instance.create_surface(
            SurfaceTarget::Canvas(canvas) // ここでラップする！
        ).map_err(|e| JsValue::from_str(&e.to_string()))?;
        
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions::default())
            .await
            .ok_or_else(|| JsValue::from_str("Failed to find adapter"))?;
            
        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor::default(), None)
            .await
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        
        Ok(Self { device, queue, surface })
    }
}
