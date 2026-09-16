// Geometry is in logical points. Kept independent of rendering for boundary tests.
pub(crate) fn preview_leaves_viewport(
    mouse: [f32; 2],
    offset: [f32; 2],
    preview: Option<[f32; 2]>,
    viewport: [f32; 2],
) -> bool {
    if mouse[0] < 0.0 || mouse[1] < 0.0 || mouse[0] > viewport[0] || mouse[1] > viewport[1] {
        return true;
    }
    let Some(size) = preview else {
        return false;
    };
    let origin = [mouse[0] - offset[0], mouse[1] - offset[1]];
    origin[0] < 0.0
        || origin[1] < 0.0
        || origin[0] + size[0] > viewport[0]
        || origin[1] + size[1] > viewport[1]
}

#[cfg(test)]
mod tests {
    use super::preview_leaves_viewport as leaves;

    #[test]
    fn promotes_before_pointer_crosses_titlebar() {
        assert!(leaves(
            [218.5, 1.14],
            [126.48, 21.6],
            Some([260.0, 36.0]),
            [1100.0, 760.0]
        ));
    }

    #[test]
    fn fully_visible_preview_and_exact_edge_stay_internal() {
        assert!(!leaves(
            [126.0, 22.0],
            [126.0, 22.0],
            Some([260.0, 36.0]),
            [1100.0, 760.0]
        ));
        assert!(!leaves(
            [600.0, 100.0],
            [126.0, 22.0],
            Some([260.0, 36.0]),
            [1100.0, 760.0]
        ));
    }

    #[test]
    fn detects_left_right_and_bottom_overflow() {
        for mouse in [[10.0, 100.0], [1090.0, 100.0], [400.0, 755.0]] {
            assert!(leaves(
                mouse,
                [126.0, 22.0],
                Some([260.0, 36.0]),
                [1100.0, 760.0]
            ));
        }
    }

    #[test]
    fn unmeasured_preview_uses_pointer_fallback() {
        assert!(!leaves([10.0, 10.0], [126.0, 22.0], None, [1100.0, 760.0]));
        assert!(leaves([10.0, -1.0], [126.0, 22.0], None, [1100.0, 760.0]));
    }
}
