use std::{ collections::HashMap, time::{ Duration, Instant } };
mod color_palette;
mod color_art;

type Lap = Instant;
type LapVec = Vec<Instant>;
type LapMap = HashMap<String, LapVec>;
struct Benchmark {
    started: bool,
    start_time: Instant,
    end_time: Instant,
    laps: LapMap,
    max_laps: u8,
}

impl Benchmark {
    fn new(max_laps: u8) -> Self {
        Self {
            started: false,
            start_time: Instant::now(),
            end_time: Instant::now(),
            laps: HashMap::new(),
            max_laps,
        }
    }

    fn start(&mut self) {
        self.started = true;
        self.start_time = Instant::now();
    }

    fn stop(&mut self) {
        self.end_time = Instant::now();
        self.started = false;
    }

    pub fn reset(&mut self) {
        self.started = false;
        self.start_time = Instant::now();
        self.end_time = Instant::now();
        self.laps = HashMap::new();
    }

    fn lap(&mut self, label: &str) {
        if self.max_laps == 0 || self.laps.len() < (self.max_laps as usize) {
            let laps = self.laps.entry(label.to_string()).or_insert(vec![]);
            laps.push(Instant::now());
        }
    }

    fn get_duration(&self) -> Duration {
        self.end_time.duration_since(self.start_time)
    }

    pub fn get_lap_duration_from_previous(&self, lap: u8, label: &str) -> Duration {
        if lap == 0 {
            return self.laps.get(label).unwrap()[0].duration_since(self.start_time);
        }
        self.laps
            .get(label)
            .unwrap()
            [lap as usize].duration_since(self.laps.get(label).unwrap()[(lap as usize) - 1])
    }

    pub fn get_lap_duration_average(&self, label: &str) -> Duration {
        let mut total = Duration::new(0, 0);
        for i in 0..self.laps.get(label).unwrap().len() {
            total += self.get_lap_duration_from_previous(i as u8, label);
        }
        total / (self.laps.get(label).unwrap().len() as u32)
    }

    pub fn lap_to_string(&self, (label, lap): (&str, &LapVec), display_all: bool) -> String {
        let mut result = "".to_string();

        for (index, lap_time) in lap.iter().enumerate() {
            let previous = {
                if index == 0 { self.start_time } else { lap[index - 1] }
            };

            result.push_str(
                &format!("    Lap {}_{}: {:?}\n", label, index, lap_time.duration_since(previous))
            );
        }
        result
    }

    pub fn to_string(&self, skip_laps: bool) -> String {
        let mut result = format!("Benchmark duration: {:?}\n", self.get_duration());
        result.push_str(format!("Laps({}):\n", self.laps.len()).as_str());
        for (label, laps) in &self.laps {
            if !skip_laps {
                result.push_str(format!("  {}\n", label).as_str());
                result.push_str(self.lap_to_string((&label, laps), true).as_str());
            }
            result.push_str(
                &format!(
                    "Average {} lap duration: {:?}\n",
                    label,
                    self.get_lap_duration_average(label)
                )
            );
        }
        result
    }
}

fn main() {
    let mut benchmark = Benchmark::new(0);
    benchmark.start();
    for y in 0..5 {
        benchmark.lap("palette_frame");

        for i in 0..(800 * 600) / 2 {
            let detection = color_palette::ColorDetection::from_hsv([0.1, 0.0, 0.0], None);
            let pixel = [0, 0, 0, 0];
            detection.color_match(&pixel);

            benchmark.lap("palette_pixel");
        }
    }
    

    if false {
        benchmark.stop();
        for y in 0..5 {
            benchmark.lap("art_frame");
    
            for i in 0..(800 * 600) / 2 {
                let detection = color_art::ColorDetection::from_hsv(vec![[0.0, 0.0, 0.0]], None);
                let pixel = [0, 0, 0, 0];
                detection.color_match(&pixel);
    
                benchmark.lap("art_pixel");
            }
        }
    
    }
   
    benchmark.stop();
    println!("{}", benchmark.to_string(true));
    benchmark.reset();
    // Bench;ark
}
