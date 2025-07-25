use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Flavor {
    Latte,
    Frappe,
    Macchiato,
    Mocha,
    Oled,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Algorithm {
    ShepardsMethod,
    GaussianRBF,
    LinearRBF,
    GaussianSampling,
    NearestNeighbour,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Properties {
    pub hald_level: u8,
    pub luminosity: f64,
    pub algorithm: Algorithm,
    pub shape: f64,
    pub nearest: usize,
    pub mean: f64,
    pub std: f64,
    pub iterations: usize,
    pub power: f64,
}
