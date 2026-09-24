use crate::{config::CONFIG, consts};
use anyhow::{Context, Result, anyhow};
use ocrs::{OcrEngine, OcrEngineParams};
use rten::Model;
use xcap::Window;
use enigo::{Enigo, Settings};

pub fn get_engine() -> Result<OcrEngine> {
    let detection_model = get_detection_model()?;
    let recognition_model = get_recognition_model()?;

    launch_engine(detection_model, recognition_model)
}


fn get_detection_model() -> Result<Model> {
    Model::load_static_slice(consts::DETECTION_MODEL).context("Failed to init Detection Model")
}


fn get_recognition_model() -> Result<Model> {
    Model::load_static_slice(consts::RECOGNITION_MODEL).context("Failed to init Recognition Model")
}


fn launch_engine(detection_model: Model, recognition_model: Model) -> Result<OcrEngine> {

    let engine = OcrEngine::new(OcrEngineParams {
        detection_model: Some(detection_model),
        recognition_model: Some(recognition_model),
        allowed_chars: Some(CONFIG.allowed_chars.to_string()),
        ..Default::default()
    })?;

    Ok(engine)
}


pub fn get_window() -> Result<Window> {
    let windows = Window::all().context("Failed to get list of monitors")?;
    let name = CONFIG.window_name.as_str();

    if name.len() == 0 {
        return Err(anyhow!("Name for window empty"));
    }

    windows
        .into_iter()
        .find(|w| {
            w.title()
                .map(|title| title.split(" ").next().unwrap_or_default().to_lowercase() == name)
                .unwrap_or(false)
        }).ok_or_else(|| anyhow!("Window '{}' not found", name))
}

pub fn get_typer() -> Result<Enigo> {
    Enigo::new(&Settings::default()).context("Failed to init Enigo")
}