use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

fn mission_log(cat: &str, msg: &str) {
    log(&format!("[{}] {}", cat, msg));
}

#[wasm_bindgen]
pub struct Engine {
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface<'static>,
}

#[wasm_bindgen]
impl Engine {
    // コンストラクタを同期的にして、JS側でPromiseを待つ設計にする
    pub async fn new(canvas: web_sys::HtmlCanvasElement) -> Result<Engine, JsValue> {
        mission_log("ACTION", "WASMエンジン初期化開始！");
        
        let instance = wgpu::Instance::default();
        // create_surfaceに変更
        let surface = instance.create_surface(&canvas).map_err(|e| JsValue::from_str(&e.to_string()))?;
        
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions::default())
            .await
            .ok_or_else(|| JsValue::from_str("Failed to find adapter"))?;
            
        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor::default(), None)
            .await
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        mission_log("SYSTEM", "GPUデバイス接続完了！");
        
        Ok(Self { device, queue, surface })
    }

    pub fn render(&self) {
        mission_log("RENDER", "フレーム描画処理実行中...");
    }
}
