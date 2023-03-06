use config::ConfigError;
use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;

#[derive(Deserialize, Clone)]
struct ImageSettingsImpl {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    aspect_ratio: f64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    height: u32,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    samples_per_pixel: u32,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    max_depth: u32,
    format: ImageFormat,
    path: String,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ImageFormat {
    Jpg,
    Png,
    Ppm,
    Tiff,
}

#[derive(Clone)]
pub struct ImageSettings {
    pub aspect_ratio: f64,
    pub width: u32,
    pub height: u32,
    pub samples_per_pixel: u32,
    pub max_depth: u32,
    pub format: ImageFormat,
    pub path: String,
}

impl ImageSettings {
    fn new(settings: ImageSettingsImpl) -> ImageSettings {
        ImageSettings {
            aspect_ratio: settings.aspect_ratio,
            width: (settings.height as f64 * settings.aspect_ratio) as u32,
            height: settings.height,
            samples_per_pixel: settings.samples_per_pixel,
            max_depth: settings.max_depth,
            format: settings.format,
            path: settings.path,
        }
    }
}

pub fn load_configuration() -> Result<ImageSettings, ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    eprintln!("{}", base_path.to_str().unwrap());
    let configuration_directory = base_path.join("configuration");
    eprintln!("{}", configuration_directory.to_str().unwrap());
    let f = config::File::from(configuration_directory.join("base.yaml"));
    let settings = config::Config::builder().add_source(f).build()?;

    settings
        .try_deserialize::<ImageSettingsImpl>()
        .map(ImageSettings::new)
}
