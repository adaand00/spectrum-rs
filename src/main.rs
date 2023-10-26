use std::{thread, time::Duration, sync::{Arc, Mutex}};
use spectrum::Spectrum;
use console::Term;
use cpal::traits::{HostTrait, DeviceTrait, StreamTrait};

mod spectrum;


fn main() {
    let spec = Spectrum::new(10);

    let spect: Arc<Mutex<Spectrum>> = Arc::new(Mutex::new(spec));
    
    let term = Term::stdout();

    let spect1 = spect.clone();

    let h1 = thread::spawn(move || {       
        loop {
            let lock = spect1.lock().unwrap();
            let s = lock.to_string();
            drop(lock);

            term.write_line(&s).unwrap();

            thread::sleep(Duration::from_millis(50));
            term.clear_last_lines(11).unwrap();
        }
    });

    let host = cpal::default_host();

    let device = host.default_input_device().unwrap();

    let mut supported_configs_range = device.supported_input_configs()
        .expect("error while querying configs");
    let supported_config = supported_configs_range.next()
        .expect("no supported config?!")
        .with_max_sample_rate();

    let spect2 = spect.clone();

    let stream = device.build_input_stream(
        &supported_config.into(),
        move |data: &[f32], _: &cpal::InputCallbackInfo| {
            let mut spec = spect2.lock().unwrap();
            for d in data {
                spec.process(*d)
            }
        },
        |err| {eprintln!("an error occurred on the output audio stream: {}", err)},
        None
    ).unwrap();

    stream.play().unwrap();
    h1.join().unwrap();
}
