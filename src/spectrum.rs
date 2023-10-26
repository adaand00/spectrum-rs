use std::fmt::Display;

use self::band::Band;

mod band;

pub struct Spectrum {
    bands: Vec<Band>
}

impl Spectrum {
    pub fn new(num_bands: u32) -> Spectrum{
        if num_bands == 0 {
            panic!("Number of bands should be more than 0")
        };

        let freqs: Vec<f32> = (0..num_bands).map(|x| {
            let minf = (20.0f32).ln();
            let maxf = (20_000.0f32).ln();
            
            let y = (x as f32)/(num_bands as f32) * (maxf- minf) + minf;

            y.exp()
        }).collect();

        let mut bands = Vec::<Band>::new();

        for i in 0..num_bands {
            let band = Band::new(freqs[i as usize], 0.999);
            bands.push(band);
        }

        Spectrum{bands}
    }

    pub fn process(&mut self, sample: f32){

        for band in &mut self.bands{
            band.process(&sample);
        }

    }

    pub fn get_bands(&self) -> Vec<f32> {
        let mut v = Vec::<f32>::new();

        for band in &self.bands {
            v.push(band.get_value())
        };

        v
    }
}

impl Display for Spectrum{ 
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        let vals = self.get_bands();
        let mut s  = String::new();

        let y: Vec<f32> = (0..10).map(|x|  (- x as f32).exp()).collect();

        for row in &y{
            for v in &vals{
                if v > row {
                    s.push_str("#  ");
                }else{
                    s.push_str("   ");
                }
            }
            s.push('\n');
        }
        write!(f, "{}", s)
    }

}