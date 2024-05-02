#[allow(dead_code)]
pub fn bench_closure<F>(mut closure: F) -> std::time::Duration where F: FnMut() {
    let start = std::time::Instant::now();
    closure();
    start.elapsed()
}

pub struct Benchmark {
    iterations: u32,
    name: String,
    results: Vec<std::time::Duration>,
    average: std::time::Duration,
    print_result: bool,
}

impl Benchmark {
    pub fn new(iterations: u32, name: &str, print: bool) -> Self {
        Self {
            iterations,
            name: name.to_string(),
            results: Vec::new(),
            average: std::time::Duration::new(0, 0),
            print_result: print,
        }
    }

    pub fn run<F>(&mut self, mut closure: F) where F: FnMut(u32) {
        println!("Running benchmark: {}(x{})", self.name, self.iterations);
        for i in 0..self.iterations {
            let start = std::time::Instant::now();
            closure(i);

            self.results.push(start.elapsed());
        }
        self.average =
            self.results.iter().fold(std::time::Duration::new(0, 0), |acc, &x| acc + x) /
            self.iterations;
        if self.print_result {
            self.print();
        }
        println!("Benchmark done: {}", self.name);
    }

    pub fn print(&self) {
        println!("  Slowest: {:?}", self.results.iter().max().unwrap());
        println!("  Fastest: {:?}", self.results.iter().min().unwrap());
        println!("  Iterations: {}", self.iterations);
        println!("  Average: {:?}", self.average);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bench_closure() {
        let duration = bench_closure(|| {
            std::thread::sleep(std::time::Duration::from_secs(1));
        });
        assert_eq!(duration.as_secs(), 1);
    }

    #[test]
    fn test_benchmark() {
        let mut benchmark = Benchmark::new(10, "test", true);
        benchmark.run(|_| {
            std::thread::sleep(std::time::Duration::from_secs(1));
        });
    }
    #[test]
    fn test_benchmark_borrow() {
        let mut benchmark = Benchmark::new(10, "test", true);
        let mut hello = vec!["world", "just", "testing"];
        let mut world = vec!["rust", "is", "cool"];
        println!("Hello: {:?}", hello);
        println!("World: {:?}", world);
        let mut closure = |_| {
            std::thread::sleep(std::time::Duration::from_secs(1));
            // transfer value from world to hello we add one element and remove it
            let elem = world.pop();
            if let Some(elem) = elem {
                hello.push(elem);
            }
        };
        benchmark.run(&mut closure);
        println!("Hello: {:?}", hello);
        println!("World: {:?}", world);
    }
}
