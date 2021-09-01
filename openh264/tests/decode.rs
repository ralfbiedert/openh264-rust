use image::{ColorType, Rgb};
use openh264::{Decoder, DecoderConfig, Error};

#[test]
fn can_get_decoder() -> Result<(), Error> {
    let config = DecoderConfig::default();
    let _decoder = Decoder::with_config(&config)?;

    Ok(())
}

#[test]
#[rustfmt::skip]
#[ignore]
fn can_decode_files() -> Result<(), Error> {
    let sources = [
        &include_bytes!("data/single_1920x1080_cabac.h264")[..],
        &include_bytes!("data/single_512x512_cabac.h264")[..],
        &include_bytes!("data/single_512x512_cavlc.h264")[..],
    ];

    for (i, src) in sources.iter().enumerate() {
        let config = DecoderConfig::default();
        let mut decoder = Decoder::with_config(&config)?;

        let yuv = decoder.xxx_decode(src)?;

        let dim = yuv.dimension_rgb();
        let rgb_len = dim.0 * dim.1 * 3;
        let mut rgb = vec![0; rgb_len];

        yuv.write_rgb8(&mut rgb)?;

        let strides = yuv.strides_yuv();
        let dim_y = yuv.dimension_y();
        let dim_u = yuv.dimension_u();
        let dim_v = yuv.dimension_v();

        image::save_buffer(format!("{}_rgb.png", i), &rgb, dim.0 as u32, dim.1 as u32, ColorType::Rgb8).unwrap();
        image::save_buffer(format!("{}_y.png", i), yuv.y_with_stride(), strides.0 as u32, dim_y.1 as u32, ColorType::L8).unwrap();
        image::save_buffer(format!("{}_u.png", i), yuv.u_with_stride(), strides.1 as u32, dim_u.1 as u32, ColorType::L8).unwrap();
        image::save_buffer(format!("{}_v.png", i), yuv.v_with_stride(), strides.2 as u32, dim_v.1 as u32, ColorType::L8).unwrap();
    }

    Ok(())
}