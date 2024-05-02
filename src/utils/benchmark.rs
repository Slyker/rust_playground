pub fn bench_closure<F>(name: &str, mut closure: F) -> std::time::Duration
where
    F: FnMut(),
{
    let start = std::time::Instant::now();
    closure();
    start.elapsed()
}

pub struct Benchmark {
    iterations: u32,
    name: String,
    results: Vec<std::time::Duration>,
    average: Option<std::time::Duration>,    
}

impl Benchmark {
    pub fn new(iterations: u32, name: &str) -> Self {
        Self {
            iterations,
            name: name.to_string(),
            results: Vec::new(),
            average: None,
        }
    }

    pub fn run<F>(&mut self, mut closure: F)
    where
        F: FnMut(u32),
    {
        println!("Running benchmark: {}(x{})", self.name, self.iterations);
        for i in 0..self.iterations {
            let start = std::time::Instant::now();
            closure(i);
            
            self.results.push(start.elapsed());
        }
        self.average = Some(
            self.results
                .iter()
                .fold(std::time::Duration::new(0, 0), |acc, &x| acc + x)
                / self.iterations,
        );
        println!("Benchmark done: {}", self.name);
    }



    /// Return results with the percentage difference from the average and fastest time
    pub fn to_diff_percent(&self) -> Vec<(std::time::Duration, f64, f64)> {
        let average = self.average.unwrap();
        let fastest = self.results.iter().min().unwrap();
        self.results
            .iter()
            .map(|x| {
                let diff = x.checked_sub(average).unwrap_or(std::time::Duration::new(0, 0));
                let diff_percent = diff.as_secs_f64() / average.as_secs_f64() * 100.0;
                let diff_percent_fastest = diff.as_secs_f64() / fastest.as_secs_f64() * 100.0;
                (*x, diff_percent, diff_percent_fastest)
            })
            .collect()
    }

    pub fn diff_percent_to_string(&self) -> String {
        let results = self.to_diff_percent();
        results
            .iter()
            .map(|(x, y, z)| format!("{:?} avg:{:.5}% fastest:{:.5}%", x, y,z))
            .collect::<Vec<String>>()
            .join("\n")
    }

    pub fn print(&self) {
        //println!("Benchmark: {}", self.name);
        println!("  Slowest: {:?}", self.results.iter().max().unwrap());
        println!("  Fastest: {:?}", self.results.iter().min().unwrap());
        println!("  Iterations: {}", self.iterations);
        //println!("  Results: {:?}", self.results);
        //println!("  Results:\n {}", self.diff_percent_to_string());
        println!("  Average: {:?}", self.average);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bench_closure() {
        let duration = bench_closure("test", || {
            std::thread::sleep(std::time::Duration::from_secs(1));
        });
        assert_eq!(duration.as_secs(), 1);
    }

    #[test]
    fn test_benchmark() {
        let mut benchmark = Benchmark::new(10, "test");
        benchmark.run(|i| {
            std::thread::sleep(std::time::Duration::from_secs(1));
        });
        benchmark.print();
    }
    #[test]
    fn test_benchmark_borrow() {
        let mut benchmark = Benchmark::new(10, "test");
        let mut hello = vec!["world", "just", "testing"];
        let mut world = vec!["rust", "is", "cool"];
        println!("Hello: {:?}", hello);
        println!("World: {:?}", world);
        let mut closure = |i| {
            std::thread::sleep(std::time::Duration::from_secs(1));
            // transfer value from world to hello we add one element and remove it
            let elem = world.pop();
            if let Some(elem) = elem {
                hello.push(elem);
            }
        };
        benchmark.run(&mut closure);
        benchmark.print();
        println!("Hello: {:?}", hello);
        println!("World: {:?}", world);
    }
}