use std::vec;

mod simulation;

enum TimeType {
    ADVECTION,
    DISPERSION,
}

struct SimTime {
    tick_type: TimeType,
    dt_A: f64,
    dt_D: f64,
    steps_between: i64,
}

struct SimSpace {
    dx: f64,
    x_max: f64,
}

struct FluidSmear<'a> {
    simulation: &'a Sim,
    empty_before: usize,
    ticks_elapse: i64,
    smear_data: vec::Vec<f64>,
}

impl FluidSmear {
    fn new(simulation_: &Sim, initial_c: f64, initial_span: f64) -> Self {
        let mut vec = Vec::new();
        let initial_slices: i64 = (initial_span / simulation_.sim_space.dx) as i64;
        assert!(initial_slices >= 0);
        for _ in 0..initial_slices {
            vec.push(initial_c)
        }
        Self {
            simulation: simulation_,
            empty_before: 0,
            ticks_elapse: 0,
            smear_data: vec,
        }
    }

    // do a time_tick
    fn next_smear<'a>(self) -> &FluidSmear<'a> {
        let next_empty_before =
            self.empty_before + self.smear_data.iter().take_while(|x| **x <= 0.0).count();

        &Self {
            simulation: self.simulation,
            empty_before: next_empty_before,
            ticks_elapse: self.ticks_elapse + 1,
            smear_data: smear,
        }
    }
}

struct Sim {
    v: f64,
    d_l: f64,
    sim_space: SimSpace,
    // internal
    sim_time: SimTime,
}

fn main() {
    println!("Hello, world!");
}
