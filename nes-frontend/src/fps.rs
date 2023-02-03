use instant::Instant;

pub struct FpsCounter {
    time: Instant,
    avg: MovingAvg,
}

impl FpsCounter {
    pub fn new(window: usize) -> Self {
        FpsCounter {
            time: Instant::now(),
            avg: MovingAvg::new(window),
        }
    }

    pub fn update(&mut self) {
        self.avg.add(1.0 / self.time.elapsed().as_secs_f64());
        self.time = Instant::now();
    }

    pub fn avg(&self) -> f64 {
        self.avg.avg()
    }
}

struct MovingAvg {
    window: usize,
    data: std::collections::VecDeque<f64>,
}

impl MovingAvg {
    fn new(window: usize) -> Self {
        MovingAvg {
            window,
            data: Default::default(),
        }
    }

    fn add(&mut self, value: f64) {
        if self.data.len() == self.window {
            self.data.pop_front();
        }

        self.data.push_back(value);
    }

    fn avg(&self) -> f64 {
        self.data.iter().sum::<f64>() / self.data.len() as f64
    }
}
