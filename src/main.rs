mod consts;
mod init;
mod ocr;
mod cv;
mod typer;
mod capture;
mod config;

use std::{fs::File, io::{self, Write}, time::{Duration, Instant}};
use anyhow::{Context, Result};
use log::{error, info, warn};
use ocrs::OcrEngine;
use xcap::Window;
use enigo::Enigo;

use crate::{capture::WindowImage, config::CONFIG};


fn main() {
    let run_result = ( || -> Result<()> {
        let engine: OcrEngine = init::get_engine().context("Failed to init OCR Engine")?;
        let mut typer:  Enigo = init::get_typer().context("Failed to init Typing modyle")?;
        let window: Window    = init::get_window().context("Could not find the Window {}")?;

        loop {
            let user_command = read_user_input();

            match user_command.as_str() {
                "s" => run_game_session(&window, &engine, &mut typer)?,
                //change, never
                "s -l" => {
                    setup_logger(true);
                    run_game_session(&window, &engine, &mut typer)?
                },
                "q" | "e" => break,
                _ => println!("Invalid"),
            }

            println!("Type new command")
        }
        Ok(())
    })();

    if let Err(e) = run_result {
        error!("Bad luck :\n{:#}", e);
        wait_befor_close();
    } else {
        info!("Thank God");
    }
}


fn read_user_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let command = input.trim();
    command.to_string()
}


fn setup_logger(enabled: bool) {
    if !enabled {
        return;
    }

    let start = Instant::now();

    let log_file = File::options()
        .create(true)
        .write(true)
        .truncate(true)
        .open("ocalrs.log")
        .expect("Failed to create ocalrs.log");

    env_logger::Builder::new()
        .target(env_logger::Target::Pipe(Box::new(log_file)))
        .filter_level(log::LevelFilter::Info)
        .format(move |buf, record| {
            let elapsed = start.elapsed().as_secs_f32();
            writeln!(buf, "[{:>6.2}s] [{}] {}", elapsed, record.level(), record.args())
        }).init();
}


fn wait_befor_close() {
    eprintln!("Body, i'm dead, if -l flag was used see logs");
    println!("Press Enter");
    let _ = io::stderr().flush();
    let _ = io::stdin().read_line(&mut String::new());
}


fn is_game_begin(image: &mut WindowImage) -> Result<bool> {
    let timer = Instant::now();

    while timer.elapsed() < Duration::from_secs(CONFIG.work_timer) {
        let img = &image.image;

        let start = cv::is_start(img);

        if start {
            return Ok(true);
        } else {
            std::thread::sleep(Duration::from_millis(100));
            image.next_frame(0)?;
        }
    }
    Ok(false)
}


fn run_game_session(window: &Window, engine: &OcrEngine, typer: &mut Enigo) -> Result<()> {
    let delay = CONFIG.delay_before_new_capture;
    let mut image = capture::WindowImage::new(window)?;

    match is_game_begin(&mut image) {
        Ok(true) => {
            info!("Game begins");
            std::thread::sleep(Duration::from_millis(CONFIG.delay_before_word_appears));
        },
        Ok(false) => {
            info!("Could not detect game beginning, back to menu");
            return Ok(());
        },
        Err(e) => return Err(e),
    }

    let timer = Instant::now();

    while timer.elapsed() < Duration::from_secs(CONFIG.work_timer) {
        // atomic attempt to recognize a word in an image
        image.next_frame(delay)?;

        let (start_x, start_y) = match cv::get_beginning_of_word(&image.image) {
            Some(xy) => xy,
            None => {
                warn!("Could not find beginning of word");
                continue;
            },
        };

        let (x, y, width, height) = cv::get_word_bounds(start_x, start_y, &image.image);
        info!("find word with area {}", width * height);

        let bin = match cv::make_bin(x, y, width, height, &image.image) {
            Some(g) => g,
            None => {
                warn!("Could not create bin with bounds: x = {}, y = {}, width = {}, height = {}", x, y, width, height);
                continue;
            },
        };

        let word = match ocr::get_word(engine, bin) {
            Ok(w) => w,
            Err(e) => {
                error!("OCR failed while trying recognize word\n{:#}", e);
                continue;
            },
        };

        info!("Recognized word: {}", word);

        if let Err(e) = typer::type_text(typer, &word) {
            error!("Typing failed when try type: {}, error: {:#}", word, e)
        };

        image.next_frame(CONFIG.delay_defore_new_word_appears)?;
    }

    Ok(())
}