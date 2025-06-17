mod encoding;
mod filters {
    mod adjust_contrast;
    mod blur;
    mod grayscale;
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
mod resizing;
