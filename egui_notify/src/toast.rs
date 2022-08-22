use crate::{Anchor, TOAST_HEIGHT, TOAST_WIDTH};
use egui::{Rect, Pos2, pos2, vec2};

#[derive(Default)]
pub enum ToastLevel {
    #[default]
    Info,
    Warning,
    Error
}

impl ToastLevel {
    #[inline]
    pub fn is_info(&self) -> bool {
        matches!(self, Self::Info)
    }
    
    #[inline]
    pub fn is_warning(&self) -> bool {
        matches!(self, Self::Warning)
    }

    #[inline]
    pub fn is_error(&self) -> bool {
        matches!(self, Self::Error)
    }
}

pub struct Toast {
    pub(crate) level: ToastLevel,
    pub(crate) caption: String,
    pub(crate) duration: Option<f32>,
    pub(crate) height: f32,
    pub(crate) width: f32,
    pub(crate) closable: bool,
}

impl Toast {
    pub fn new(caption: impl Into<String>, level: ToastLevel) -> Self {
        Self {
            caption: caption.into(),
            height: TOAST_HEIGHT,
            width: TOAST_WIDTH,
            duration: None,
            closable: true,
            level,
        }
    }

    pub fn info(caption: impl Into<String>) -> Self {
        Self::new(caption, ToastLevel::Info)
    }

    pub fn warning(caption: impl Into<String>) -> Self {
        Self::new(caption, ToastLevel::Warning)
    }

    pub fn error(caption: impl Into<String>) -> Self {
        Self::new(caption, ToastLevel::Error)
    }

    pub fn closable(mut self, closable: bool) -> Self {
        self.closable = closable;
        self
    }

    pub fn with_duration(mut self, seconds: f32) -> Self {
        self.duration = Some(seconds);
        self
    }

    pub fn with_height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    pub fn with_width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    pub(crate) fn calc_anchored_rect(&self, pos: Pos2, anchor: Anchor) -> Rect {
        match anchor {
            Anchor::TopRight => {
                Rect {
                    min: pos2(pos.x - self.width, pos.y),
                    max: pos2(pos.x, pos.y + self.height),
                }
            },
            Anchor::TopLeft => Rect {
                min: pos,
                max: pos + vec2(self.width, self.height),
            },
            Anchor::BottomRight => Rect {
                min: pos - vec2(self.width, self.height),
                max: pos,
            },
            Anchor::BottomLeft => Rect {
                min: pos2(pos.x, pos.y - self.height),
                max: pos2(pos.x + self.width, pos.y),
            },
        }
    }

    pub(crate) fn adjust_next_pos(&self, pos: &mut Pos2, anchor: Anchor, vertical: bool, spacing: f32) {
        match anchor {
            Anchor::TopRight | Anchor::TopLeft if vertical => pos.y += self.height + spacing,
            Anchor::TopRight if !vertical => pos.x -= self.width + spacing,
            Anchor::TopLeft if !vertical => pos.x += self.width + spacing,

            Anchor::BottomRight | Anchor::BottomLeft if vertical => pos.y -= self.height + spacing,
            Anchor::BottomRight if !vertical => pos.x -= self.width + spacing,
            Anchor::BottomLeft if !vertical => pos.x += self.width + spacing,
            _ => unreachable!()
        }
    }
}