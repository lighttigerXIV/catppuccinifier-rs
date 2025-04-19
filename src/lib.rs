use std::{error::Error, path::Path};

use generation::{Algorithm, Flavor, Properties};
use image::{open, RgbaImage};
use lutgen::{
    identity::correct_image,
    interpolation::{
        GaussianRemapper, GaussianSamplingRemapper, LinearRemapper, NearestNeighborRemapper,
        ShepardRemapper,
    },
    GenerateLut,
};
use lutgen_palettes::Palette;

pub mod generation;

const GENERATION_SEED: u64 = u64::from_be_bytes(*b"42080085");

pub fn catppuccinify<P: AsRef<Path>>(
    properties: &Properties,
    flavor: &Flavor,
    target_image_path: P,
    output_image_path: P,
) -> Result<(), Box<dyn Error>> {
    let target_path = target_image_path.as_ref();
    let output_path = output_image_path.as_ref();

    if !target_path.exists() {
        return Err("Couldn't find image".into());
    }

    let palette = match flavor {
        Flavor::Latte => Palette::CatppuccinLatte.get(),
        Flavor::Frappe => Palette::CatppuccinFrappe.get(),
        Flavor::Macchiato => Palette::CatppuccinMacchiato.get(),
        Flavor::Mocha => Palette::CatppuccinMocha.get(),
        Flavor::Oled => Palette::CatppuccinOled.get(),
    };

    let lut = match properties.algorithm {
        Algorithm::GaussianRBF => GaussianRemapper::new(
            &palette,
            properties.shape,
            properties.nearest,
            properties.luminosity,
            true,
        )
        .generate_lut(properties.hald_level),

        Algorithm::GaussianSampling => GaussianSamplingRemapper::new(
            &palette,
            properties.mean,
            properties.std,
            properties.iterations,
            properties.luminosity,
            GENERATION_SEED,
        )
        .generate_lut(properties.hald_level),

        Algorithm::LinearRBF => {
            LinearRemapper::new(&palette, properties.nearest, properties.luminosity, true)
                .generate_lut(properties.hald_level)
        }

        Algorithm::ShepardsMethod => ShepardRemapper::new(
            &palette,
            properties.power,
            properties.nearest,
            properties.luminosity,
            true,
        )
        .generate_lut(properties.hald_level),

        _ => NearestNeighborRemapper::new(&palette, properties.luminosity)
            .generate_lut(properties.hald_level),
    };

    lut.save(output_path)?;

    let buffer = open(target_path)?;
    let mut target_rgba: RgbaImage = buffer.to_rgba8();

    correct_image(&mut target_rgba, &lut);

    return match target_rgba.save(output_path) {
        Ok(_) => Ok(()),
        Err(e) => Err(e.into()),
    };
}
