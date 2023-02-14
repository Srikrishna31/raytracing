use config::ConfigError;
use serde_aux::field_attributes::deserialize_number_from_string;

#[derive(serde::Deserialize, Clone)]
struct ImageSettingsImpl {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    aspect_ratio: f64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    height: u32,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    samples_per_pixel: u32,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    max_depth: u32,
}

#[derive(Clone)]
pub struct ImageSettings {
    pub aspect_ratio: f64,
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) samples_per_pixel: u32,
    pub(crate) max_depth: u32,
}

impl ImageSettings {
    fn new(settings: ImageSettingsImpl) -> ImageSettings {
        ImageSettings {
            aspect_ratio: settings.aspect_ratio,
            width: (settings.height as f64 * settings.aspect_ratio) as u32,
            height: settings.height,
            samples_per_pixel: settings.samples_per_pixel,
            max_depth: settings.max_depth,
        }
    }
}

pub fn load_configuration() -> Result<ImageSettings, ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let configuration_directory = base_path.join("configuration");

    let f = config::File::from(configuration_directory.join("base.yaml"));
    let settings = config::Config::builder().add_source(f).build()?;

    settings
        .try_deserialize::<ImageSettingsImpl>()
        .map(ImageSettings::new)
}
