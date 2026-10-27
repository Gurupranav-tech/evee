use crate::{Input, event::event::EventCategory};

pub trait Layer {
    fn on_attach(&mut self);
    fn on_detach(&mut self);
    fn on_update(&mut self, input_manager: &dyn Input);
    fn on_event(&mut self, event: &mut EventCategory);

    fn name(&self) -> &str {
        "None"
    }
}

pub struct LayerStack {
    pub layers: Vec<Box<dyn Layer>>,
    layer_index: usize,
}

impl LayerStack {
    pub fn new() -> Self {
        Self {
            layers: Vec::new(),
            layer_index: 0,
        }
    }

    pub fn push_layer(&mut self, mut layer: Box<dyn Layer>) {
        layer.on_attach();
        self.layers.push(layer);
    }

    pub fn push_overlay(&mut self, mut overlay: Box<dyn Layer>) {
        overlay.on_attach();
        self.layers.insert(self.layer_index, overlay);
        self.layer_index += 1;
    }

    pub fn size(&self) -> usize {
        self.layers.len()
    }
}

impl Drop for LayerStack {
    fn drop(&mut self) {
        for layer in &mut self.layers {
            layer.on_detach();
        }
    }
}
