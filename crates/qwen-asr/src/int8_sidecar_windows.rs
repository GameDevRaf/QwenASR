use crate::config::QwenConfig;

pub enum WeightBuf<T: Copy> {
    Owned(Vec<T>),
}

impl<T: Copy> WeightBuf<T> {
    pub fn empty() -> Self { Self::Owned(Vec::new()) }
}

impl<T: Copy> From<Vec<T>> for WeightBuf<T> {
    fn from(value: Vec<T>) -> Self { Self::Owned(value) }
}

impl<T: Copy> std::ops::Deref for WeightBuf<T> {
    type Target = [T];
    fn deref(&self) -> &[T] {
        match self { Self::Owned(value) => value.as_slice() }
    }
}

pub struct SidecarLayout;

impl SidecarLayout {
    pub fn compute(_cfg: &QwenConfig) -> Self { Self }
    pub fn layer_buf(&self, _layer: usize, _buffer: usize) -> usize { 0 }
    pub fn lm_head_idx(&self, _layers: usize) -> usize { 0 }
}

pub struct SidecarMmap;

impl SidecarMmap {
    pub fn open_valid(
        _path: &str,
        _model_dir: &str,
        _cfg: &QwenConfig,
        _layout: &SidecarLayout,
    ) -> Option<Self> { None }

    pub fn i8_buf(&self, _index: usize) -> WeightBuf<i8> { WeightBuf::empty() }
    pub fn f32_buf(&self, _index: usize) -> WeightBuf<f32> { WeightBuf::empty() }
}

pub fn write_sidecar(
    _path: &str,
    _model_dir: &str,
    _cfg: &QwenConfig,
    _layout: &SidecarLayout,
    _bufs: &[&[u8]],
) -> bool { false }

pub fn sidecar_path(model_dir: &str) -> String {
    format!("{}/qwen-asr-int8.sidecar", model_dir)
}

pub fn enabled() -> bool { false }
