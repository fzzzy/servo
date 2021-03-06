import geom::size::Size2D;
import int::num;

enum format {
    fo_rgba_8888
    // TODO: RGB 565, others?
}

type image_surface = {
    size: Size2D<int>,
    format: format,
    buffer: ~[u8]
};

impl format for format {
    fn bpp() -> uint {
        alt self {
            fo_rgba_8888 => 32u 
        }
    }
}

fn image_surface(size: Size2D<int>, format: format) -> image_surface {
    {
        size: copy size,
        format: format,
        buffer: vec::from_elem((size.area() as uint) * format.bpp(), 0u8)
    }
}

