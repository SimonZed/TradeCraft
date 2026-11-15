//! Text label and anchored text rendering

use crate::drawings::domain::Drawing;
use crate::drawings::rendering::ChartView;
use crate::styles::typography::font_size;
use crate::tokens::DESIGN_TOKENS;
use egui::epaint::StrokeKind;

fn pine_label_origin(style: &str, pos: egui::Pos2, size: egui::Vec2) -> egui::Pos2 {
    let anchor = match style {
        "style_label_up" => egui::Align2::CENTER_TOP,
        "style_label_down" => egui::Align2::CENTER_BOTTOM,
        "style_label_left" => egui::Align2::LEFT_CENTER,
        "style_label_right" => egui::Align2::RIGHT_CENTER,
        "style_label_upper_left" => egui::Align2::LEFT_TOP,
        "style_label_upper_right" => egui::Align2::RIGHT_TOP,
        "style_label_lower_left" => egui::Align2::LEFT_BOTTOM,
        "style_label_lower_right" => egui::Align2::RIGHT_BOTTOM,
        _ => egui::Align2::CENTER_CENTER,
    };
    anchor.align_size_within_rect(size, egui::Rect::from_min_max(pos, pos)).min
}

impl Drawing {
    /// Renders Text and Anchored text with the text's top-left corner on the first anchor,
    /// surrounded by a background and border when enabled. Anchored text differs only in
    /// how its anchor follows the pane (`Drawing::update_pane_coords`). Wrapped text breaks at
    /// word boundaries, and inside a word wider than the wrap width. Text that follows the zoom
    /// scales its font, wrap width and box padding with the bar spacing.
    pub(crate) fn render_text_label(&self, painter: &egui::Painter, view: &ChartView) {
        let Some(&pos) = self.points.first() else {
            return;
        };
        let colors = self.text_box_colors(painter.ctx());
        let text = self.display_text(painter.ctx());
        let mut galley = self.annotation_galley(painter, text, colors.text);
        // The font stays within the sizes text is laid out at, and the box keeps its proportions.
        let set = self.annotation_font().size;
        let font = font_size(set * self.text_zoom(view.bar_to_x, pos.x));
        let scale = font / set;
        if self.text_wrap || scale != 1.0 {
            let mut job = (*galley.job).clone();
            for section in &mut job.sections {
                section.format.font_id.size = font;
            }
            if self.text_wrap {
                job.wrap.max_width = self.text_wrap_width * scale;
            }
            galley = painter.layout_job(job);
        }
        let pos = match self.custom_settings.get("pine_label_style").and_then(|value| value.as_str()) {
            Some(style) => pine_label_origin(style, pos, galley.size()),
            None => pos,
        };
        if colors.fill.a() > 0 || !colors.border.is_empty() {
            let padding = egui::vec2(
                DESIGN_TOKENS.sizing.annotation.label_padding_x,
                DESIGN_TOKENS.sizing.annotation.text_padding_y,
            ) * scale;
            let box_rect = egui::Rect::from_min_size(pos - padding, galley.size() + padding * 2.0);
            painter.rect(
                box_rect,
                DESIGN_TOKENS.rounding.md * scale,
                colors.fill,
                colors.border,
                StrokeKind::Outside,
            );
        }
        painter.galley(pos, galley, colors.text);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pine_labels_are_placed_on_the_requested_side() {
        let point = egui::pos2(100.0, 100.0);
        let size = egui::vec2(40.0, 20.0);
        assert_eq!(pine_label_origin("style_label_down", point, size), egui::pos2(80.0, 80.0));
        assert_eq!(pine_label_origin("style_label_up", point, size), egui::pos2(80.0, 100.0));
        assert_eq!(pine_label_origin("style_label_left", point, size), egui::pos2(100.0, 90.0));
        assert_eq!(pine_label_origin("style_label_right", point, size), egui::pos2(60.0, 90.0));
    }
}
