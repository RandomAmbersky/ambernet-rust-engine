use rs_amberskynet::gfx::AsnTexture;
use std::num::NonZeroU32;
use wgpu::{Device, Label, Queue};

pub struct Size2D<T> {
    pub width: T,
    pub height: T,
}

pub struct Array2D<S, T> {
    pub size: Size2D<S>,
    pub bytes: Vec<T>,
}
