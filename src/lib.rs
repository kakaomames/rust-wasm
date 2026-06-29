use wasm_bindgen::prelude::*;
use wgpu::util::DeviceExt;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// ログ用関数
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
    #[wasm_bindgen(constructor)]
    pub async fn new(canvas: web_sys::HtmlCanvasElement) -> Self {
        mission_log("ACTION", "WASMエンジン初期化開始！");
        
        let instance = wgpu::Instance::default();
        let surface = instance.create_surface_from_canvas(&canvas).unwrap();
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions::default()).await.unwrap();
        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor::default(), None).await.unwrap();

        mission_log("SYSTEM", "GPUデバイス接続完了！");
        
        Self { device, queue, surface }
    }

    pub fn render(&self) {
        // ここで空を塗りつぶす等の描画コマンドを発行する
        mission_log("RENDER", "フレーム描画処理実行中...");
    }
}
