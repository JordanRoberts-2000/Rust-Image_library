mod encoding {
    mod jpeg;
    mod png;
    mod webp;
    mod webp_lossy;
}
mod filters {
    mod blur;
}
mod geometry {
    mod inset;
    mod inset_ratio;
    mod rotate;
    mod crop {
        mod pixels {
            mod crop_bottom;
            mod crop_left;
            mod crop_right;
            mod crop_top;
            mod crop_x;
            mod crop_y;
        }
        mod ratio {
            mod crop_bottom_ratio;
            mod crop_left_ratio;
            mod crop_right_ratio;
            mod crop_top_ratio;
            mod crop_x_ratio;
            mod crop_y_ratio;
        }
        mod crop;
        mod crop_aspect;
        mod crop_square;
    }
}
mod resizing {
    mod resize;
}
