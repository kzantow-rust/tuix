use crate::entity::*;

pub struct Size {
    pub width: u32,
    pub height: u32,
}

impl Size {
    pub fn new(width: u32, height: u32) -> Self {
        Size { width, height }
    }
}

/// Passed to the window to set various window properties
pub struct WindowDescription {
    pub title: String,
    pub inner_size: Size,
    pub min_inner_size: Size,
    // Change this to resource id when the resource manager is working
    pub icon: Option<Vec<u8>>,
    pub icon_width: u32,
    pub icon_height: u32,
}

impl Default for WindowDescription {
    fn default() -> Self {
        Self {
            title: "Tuix Application".to_string(),
            inner_size: Size::new(800, 600),
            min_inner_size: Size::new(100, 100),
            icon: None,
            icon_width: 0,
            icon_height: 0,
        }
    }
}

impl WindowDescription {
    pub fn new() -> Self {
        WindowDescription {
            title: "Tuix Application".to_string(),
            inner_size: Size::new(800, 600),
            min_inner_size: Size::new(100, 100),
            icon: None,
            icon_width: 0,
            icon_height: 0,
        }
    }

    pub fn with_title(mut self, title: &str) -> Self {
        self.title = title.to_string();

        self
    }

    pub fn with_inner_size(mut self, width: u32, height: u32) -> Self {
        self.inner_size = Size::new(width, height);

        self
    }

    pub fn with_min_inner_size(mut self, width: u32, height: u32) -> Self {
        self.min_inner_size = Size::new(width, height);

        self
    }

    pub fn with_icon(mut self, icon: Vec<u8>, width: u32, height: u32) -> Self {
        self.icon = Some(icon);
        self.icon_width = width;
        self.icon_height = height;
        self
    }
}
