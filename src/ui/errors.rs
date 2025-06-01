use crate::entities::craft_repo::Message;
use egui::{Color32, RichText, Sense, Ui};
use log::debug;

const LOG_TARGET: &str = "ui";

pub fn show_errors(ui: &mut Ui, messages: &mut Vec<Message>) {
    if !messages.is_empty() {
        if ui.button("clean errors").clicked() {
            messages.clear();
        }
    }

    let texts = messages
        .iter()
        .map(|m| m.text.clone())
        .collect::<Vec<String>>();
    
    // Use a separate vector to track indices to remove
    let mut indices_to_remove = Vec::new();
    
    for (i, message) in texts.iter().enumerate() {
        let label = egui::Label::new(RichText::new(message).color(Color32::LIGHT_RED))
            .wrap(false)
            .sense(Sense::click());
        if ui.add(label).clicked() {
            indices_to_remove.push(i);
            debug!(target: LOG_TARGET, "marked error for removal");
        }
    }
    
    // Remove messages in reverse order to avoid index shifting
    for &index in indices_to_remove.iter().rev() {
        if index < messages.len() {
            messages.remove(index);
            debug!(target: LOG_TARGET, "removed error at index {}", index);
        }
    }
}
