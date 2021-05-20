use rand::Rng;

fn main() {
    let mut stay = 0;
    let mut switch = 0;
    let iterations = 50_000;
    for _ in 0..iterations {
        let (a, b) = simulation();
        stay += a as i32;
        switch += b as i32;
    }
    println!("Staying: {:.1}%", (stay as f64) / (iterations as f64) * 100.0);
    println!("Switching: {:.1}%", (switch as f64) / (iterations as f64) * 100.0);
}

fn simulation() -> (bool, bool) {
    let doors = get_doors();
    let pick_index = rand::thread_rng().gen_range(0..doors.len());
    if doors[pick_index] {
        return (true, false);
    }
    (false, true)
}

fn get_doors() -> [bool; 3] {
    let mut doors = [false, false, false];
    let index = rand::thread_rng().gen_range(0..doors.len());
    doors[index] = true;
    doors
}
