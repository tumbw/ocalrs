use anyhow::Result;
use image::{ImageBuffer, Rgba};
use xcap::Window;
use std::time::{Duration};


pub struct WindowImage<'a> {
    pub image: ImageBuffer<Rgba<u8>, Vec<u8>>,
    pub window: &'a Window,
}

impl<'a> WindowImage<'a> {
    pub fn new(window: &'a Window) -> Result<Self> {
        let img = window.capture_image()?;
        Ok(Self { image: img, window })
    }

    pub fn next_frame(&mut self, delay: u64) -> Result<()> {
        std::thread::sleep(Duration::from_millis(delay));
        let img = self.window.capture_image()?;
        self.image = img;
        Ok(())
    }
}