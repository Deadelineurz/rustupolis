use crate::engine::drawable::Drawable;
use crate::roads::road_graph::Rect;

pub fn intersection(r1: &dyn Drawable, r2: &dyn Drawable) -> Option<Rect> {
    let tolerance: i16 = 1;

    let x1 = r1.x().max(r2.x());
    let y1 = r1.y().max(r2.y());
    let x2 = (r1.x() + r1.width() as i16).min(r2.x() + r2.width() as i16);
    let y2 = (r1.y() + r1.height() as i16).min(r2.y() + r2.height() as i16);

    // Allow for tolerance of 1 pixel in both x and y directions
    if x1 < x2 + tolerance && y1 < y2 + tolerance {
        Some(Rect {
            x: x1,
            y: y1,
            width: ((x2 - x1).max(0) + tolerance) as u8, // ensure non-negative, add tolerance
            height: ((y2 - y1).max(0) + tolerance) as u8,
        })
    } else {
        None
    }
}