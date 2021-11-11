#[macro_export]
macro_rules! offset {
    (+; $arg:expr, $camera:expr) => {{
        let mut dest = $arg;
        dest.offset($camera.x, $camera.y);
        dest
    }};
    (-; $arg:expr, $camera:expr) => {{
        let mut dest = $arg;
        dest.offset(-$camera.x, -$camera.y);
        dest
    }};
}
#[macro_export]
macro_rules! minimap_scaled_rect {
    ($rect:expr, $minimap:expr) => {{
        let x = (($rect.left() - $minimap.min_x)as f32 / $minimap.ratio) as i32 + $minimap.offset_x;
        let y = (($rect.top()  - $minimap.min_y)as f32 / $minimap.ratio) as i32 + $minimap.offset_y;
        let w = ($rect.width() as f32 / $minimap.ratio) as u32;
        let h = ($rect.height() as f32 / $minimap.ratio) as u32;
        Rect::new(x, y, w, h)
    }};
}