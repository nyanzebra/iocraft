use crate::{Component, ComponentRenderer, ComponentUpdater};
use crossterm::style::{Color, ContentStyle, PrintStyledContent, StyledContent};
use taffy::Size;

#[derive(Clone, Default)]
pub struct TextProps {
    pub color: Option<Color>,
    pub content: String,
}

pub struct Text {
    style: ContentStyle,
    content: String,
}

impl Component for Text {
    type Props = TextProps;

    fn new(_props: &Self::Props) -> Self {
        Self {
            style: ContentStyle::new(),
            content: "".to_string(),
        }
    }

    fn update(&mut self, props: &Self::Props, updater: &mut ComponentUpdater<'_>) {
        self.style.foreground_color = props.color;
        self.content = props.content.clone();
        let width = self.content.len() as f32;
        updater.set_measure_func(Box::new(move |_, _, _| Size { width, height: 1.0 }));
    }

    fn render(&self, renderer: &mut ComponentRenderer<'_>) {
        renderer.queue(PrintStyledContent(StyledContent::new(
            self.style,
            &self.content,
        )));
    }
}