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
    mod rotation;
    mod cropping {
        mod crop;
        mod crop_aspect;
        mod crop_pixels;
        mod crop_ratio;
        mod crop_square;
        mod inset;
        mod inset_ratio;
    }
}
mod resizing {
    mod resize;
}
