pub trait ImGuiAddOn {
    fn render(&mut self, ui: &mut imgui::Ui);
}
