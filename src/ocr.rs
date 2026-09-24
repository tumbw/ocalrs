use anyhow::{Context, Result, anyhow};
use image::{ImageBuffer, Rgba};
use ocrs::{ImageSource, OcrEngine};


pub fn get_word (engine: &OcrEngine, grayscale: ImageBuffer<Rgba<u8>, Vec<u8>>) -> Result<String> {

    let img_source: ImageSource<'_> = ImageSource::from_bytes(grayscale.as_raw(), grayscale.dimensions()).context("Failed to create ImageSource")?;

    let ocr_input: ocrs::OcrInput = engine.prepare_input(img_source).context("Failed to Prepare Input")?;

    let word_rects = engine.detect_words(&ocr_input).context("Failed to Detection Words")?;

    let line_rects = engine.find_text_lines(&ocr_input, &word_rects);

    let line_texts: Vec<Option<ocrs::TextLine>> = engine.recognize_text(&ocr_input, &line_rects).context("Failed to Recognition Text")?;

    let text  = line_texts
        .into_iter()
        .flatten()
        .filter(|l| l.to_string().len() > 1)
        .map(|l| l.to_string())
        .collect::<Vec<_>>()
        .join("");

    if text.is_empty() {
        return Err(anyhow!("Could not recognize any words"));
    }

    Ok(text)

}