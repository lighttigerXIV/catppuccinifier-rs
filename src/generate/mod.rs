use image::open;
use lutgen::{
    interpolation::{
        GaussianRemapper, GaussianSamplingRemapper, LinearRemapper, NearestNeighborRemapper,
        ShepardRemapper,
    },
    GenerateLut, identity::correct_image,
};
use std::path::PathBuf;
use lutgen_palettes::Palette;

const SEED: u64 = u64::from_be_bytes(*b"42080085");

#[derive(Debug)]
pub enum Flavor {
    Latte,
    Frappe,
    Macchiato,
    Mocha,
    Oled,
}


#[derive(Debug)]
pub enum Algorithm {
    ShepardsMethod,
    GaussianRBF,
    LinearRBF,
    GaussianSampling,
    NearestNeighbor,
}


pub struct GenerateProperties {
    hald_level: u8,
    luminosity: f64,
    algorithm: Algorithm,
    shape: f64,
    nearest: usize,
    mean: f64,
    std: f64,
    iterations: usize,
    power: f64,
}

pub fn generate_image(
    properties: GenerateProperties,
    flavor: Flavor,
    image_path: PathBuf,
    save_path: PathBuf,
) -> Result<(), String> {


    if !image_path.exists() {
        return Err("Image doesn't exist".to_owned());
    }


    let palette = match flavor {
        Flavor::Latte => Palette::CatppuccinLatte.get(),
        Flavor::Frappe => Palette::CatppuccinFrappe.get(),
        Flavor::Macchiato => Palette::CatppuccinMacchiato.get(),
        Flavor::Mocha => Palette::CatppuccinMocha.get(),
        Flavor::Oled => Palette::CatppuccinOled.get(),
    };

    let hald_clut = match properties.algorithm {
        Algorithm::GaussianRBF => GaussianRemapper::new(
            &palette,
            properties.shape,
            properties.nearest,
            properties.luminosity,
        )
        .generate_lut(properties.hald_level),

        Algorithm::GaussianSampling => GaussianSamplingRemapper::new(
            &palette,
            properties.mean,
            properties.std,
            properties.iterations,
            properties.luminosity,
            SEED,
        )
        .generate_lut(properties.hald_level),

        Algorithm::LinearRBF => {
            LinearRemapper::new(&palette, properties.nearest, properties.luminosity)
                .generate_lut(properties.hald_level)
        }

        Algorithm::ShepardsMethod => ShepardRemapper::new(
            &palette,
            properties.power,
            properties.nearest,
            properties.luminosity,
        )
        .generate_lut(properties.hald_level),

        _ => NearestNeighborRemapper::new(&palette, properties.luminosity)
            .generate_lut(properties.hald_level),
    };

    let lut_was_generated = match hald_clut.save(save_path.to_owned()) {
        Ok(_) => true,
        Err(_) => false
    };

    
    if !lut_was_generated{
        return Err("Error generating image lut".to_owned())
    }


    let mut generated_image = open(image_path).unwrap().to_rgb8();
    correct_image(&mut generated_image, &hald_clut);

    return match generated_image.save(save_path.to_owned()) {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string())
    };
}
