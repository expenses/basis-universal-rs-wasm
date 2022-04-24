use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(module = "/basis_universal/webgl/transcoder/build/basis_transcoder.js")]
extern "C" {
    #[wasm_bindgen(js_name = default)]
    pub async fn init(map: wasm_bindgen::JsValue) -> wasm_bindgen::JsValue;

    #[wasm_bindgen]
    pub type Module;

    #[wasm_bindgen(js_name = initializeBasis, method)]
    pub fn initialize_basis(this: &Module);

    #[wasm_bindgen(js_name = BasisFile, method, getter)]
    pub fn basis_file_constructor(this: &Module) -> js_sys::Function;
}

impl Module {
    pub async fn new() -> Self {
        Self::new_with_wasm_bytes(include_bytes!("../basis_universal/webgl/transcoder/build/basis_transcoder.wasm")).await
    }

    pub async fn new_with_wasm_bytes(
        bytes: &[u8]
    ) -> Self {
        let array = unsafe {
            js_sys::Uint8Array::view(bytes)
        };

        let map = js_sys::Object::new();
        js_sys::Reflect::set(&map, &"wasmBinary".into(), &array.into()).unwrap();

        init(map.into()).await.into()
    }

    pub fn create_basis_file(&self, array: &js_sys::Uint8Array) -> BasisFile {
        js_sys::Reflect::construct(&self.basis_file_constructor(), &js_sys::Array::of1(&array)).unwrap().into()
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    pub type BasisFile;

    #[wasm_bindgen(constructor)]
    fn new(array: &js_sys::Uint8Array) -> BasisFile;

    #[wasm_bindgen(js_name = getNumImages, method)]
    pub fn get_num_images(this: &BasisFile) -> u32;

    #[wasm_bindgen(js_name = getNumLevels, method)]
    pub fn get_num_levels(this: &BasisFile, image_index: u32) -> u32;

    #[wasm_bindgen(js_name = getHasAlpha, method)]
    pub fn get_has_alpha(this: &BasisFile) -> bool;

    #[wasm_bindgen(js_name = startTranscoding, method)]
    pub fn start_transcoding(this: &BasisFile) -> u32;

    #[wasm_bindgen(js_name = getImageWidth, method)]
    pub fn get_image_width(this: &BasisFile, image_index: u32, level_index: u32) -> u32;

    #[wasm_bindgen(js_name = getImageHeight, method)]
    pub fn get_image_height(this: &BasisFile, image_index: u32, level_index: u32) -> u32;

    #[wasm_bindgen(js_name = getImageTranscodedSizeInBytes, method)]
    pub fn get_image_transcoded_size_in_bytes(
        this: &BasisFile,
        image_index: u32,
        level_index: u32,
        format: u32,
    ) -> u32;

    #[wasm_bindgen(js_name = transcodeImage, method)]
    pub fn transcode_image(
        this: &BasisFile,
        dst: &js_sys::Uint8Array,
        image_index: u32,
        level_index: u32,
        format: u32,
        unused: u32,
        get_alpha_for_opaque_formats: u32,
    ) -> u32;

    #[wasm_bindgen(method)]
    pub fn close(this: &BasisFile);

    #[wasm_bindgen(method)]
    pub fn delete(this: &BasisFile);
}
