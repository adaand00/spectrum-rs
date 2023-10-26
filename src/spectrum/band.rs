use std::f32::consts::PI;

pub struct Band {
    num: [f32; 3],
    denom: [f32; 3],

    x: [f32; 2],
    y: [f32; 2],

    val: f32,
    decay: f32,

}

impl Band {
    pub fn new(freq: f32, q: f32) -> Band {

        let w0 = freq*PI/48_000.0;

        let b  = q;

        let num = [1.0-b, 0.0, b-1.0];
        let denom = [0.0, -2.0 * w0.cos()*b, 2.0*b-1.0];

        Band { 
            num, 
            denom, 
            x: [0.0, 0.0],
            y: [0.0, 0.0],
            val: 0.0, 
            decay: 0.0002 }
    }

    pub fn process(&mut self, sample: &f32) -> f32{
        // Run through IIR filter
        let x2 = self.x[1];
        let y2 = self.y[1];

        self.x[1] = self.x[0];
        self.x[0] = *sample;

        self.y[1] = self.y[0];

        self.y[0] = 
            self.num[0] * self.x[0] +
            self.num[1] * self.x[1] + 
            self.num[2] * x2 -
            self.denom[1] * self.y[1] -
            self.denom[2] * y2;
        

        // Low pass-filter squared output
        let val2 = self.y[0]*self.y[0];

        self.val = (val2 - self.val)*self.decay + self.val;
        self.val
    }

    pub fn get_value(&self) -> f32{
        self.val
    }

}