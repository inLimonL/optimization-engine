
use super::super::*;
use super::*;
use crate::constraints;

const N_DIM: usize = 2;

fn my_cost(u: &[f64], cost: &mut f64) -> i32 {
    *cost = 0.5 * (u[0].powi(2) + 2. * u[1].powi(2) + 2.0 * u[0] * u[1]) + u[0] - u[1] + 3.0;
    0
}

fn my_gradient(u: &[f64], grad: &mut [f64]) -> i32 {
    grad[0] = u[0] + u[1] + 1.0;
    grad[1] = u[0] + 2. * u[1] - 1.0;
    0
}

#[test]
fn fbs_step_no_constraints() {
    let no_constraints = constraints::NoConstraints::new();
    let problem = Problem::new(no_constraints, my_gradient, my_cost);
    let gamma = 0.1;
    let tolerance = 1e-6;
    let mut fbs_cache = FBSCache::new(N_DIM, gamma, tolerance);
    {
        let mut fbs_engine = FBSEngine::new(problem, &mut fbs_cache);
        let mut u = [1.0, 3.0];
        assert_eq!(true, fbs_engine.step(&mut u));
        assert_eq!([0.5, 2.4], u);
    }
    assert_eq!([1., 3.], *fbs_cache.work_u_previous);
}

#[test]
fn fbs_step_ball_constraints() {
    let no_constraints = constraints::Ball2::new_at_origin_with_radius(0.1);
    let problem = Problem::new(no_constraints, my_gradient, my_cost);
    let gamma = 0.1;
    let tolerance = 1e-6;
    let mut fbs_cache = FBSCache::new(N_DIM, gamma, tolerance);
    let mut fbs_engine = FBSEngine::new(problem, &mut fbs_cache);

    let mut u = [1.0, 3.0];

    assert_eq!(true, fbs_engine.step(&mut u));
    assert!((u[0] - 0.020395425411200).abs() < 1e-14);
    assert!((u[1] - 0.097898041973761).abs() < 1e-14);
}

#[test]
fn solve_fbs() {
    let radius = 0.2;
    let box_constraints = constraints::Ball2::new_at_origin_with_radius(radius);
    let problem = Problem::new(box_constraints, my_gradient, my_cost);
    let gamma = 0.1;
    let tolerance = 1e-6;
    let mut fbs_cache = FBSCache::new(N_DIM, gamma, tolerance);
    let mut fbs_engine = FBSEngine::new(problem, &mut fbs_cache);
    let mut u = [0.0; N_DIM];
    let mut optimizer = FBSOptimizer::new(&mut fbs_engine);
    let status = optimizer.solve(&mut u);
    assert!(status.has_converged());
    assert!(status.get_norm_fpr() < tolerance);
    assert!((-0.14896 - u[0]).abs() < 1e-4);
    assert!((0.13346 - u[1]).abs() < 1e-4);
}

#[test]
fn solve_fbs_many_times() {
    // Algorithm configuration
    let gamma = 0.1;
    let tolerance = 1e-6;

    // The cache is constructed ONCE. This step allocates memory.
    let mut fbs_cache = FBSCache::new(N_DIM, gamma, tolerance);

    let mut u = [0.0; 2];

    for _i in 1..10 {
        // Every time NMPC is executed, the constraints may change
        let box_constraints = constraints::Ball2::new_at_origin_with_radius(0.2);
        // The problem is surely update at every execution of NMPC
        let problem = Problem::new(box_constraints, my_gradient, my_cost);
        // Construct a new Engine; this does not allocate any memory
        let mut fbs_engine = FBSEngine::new(problem, &mut fbs_cache);
        // Here comes the new initial condition
        u[0] = 2.0 * _i as f64;
        u[1] = -_i as f64;
        // Create a new optimizer...
        let mut optimizer = FBSOptimizer::new(&mut fbs_engine);
        let status = optimizer.solve(&mut u);
        assert!(status.get_norm_fpr() < tolerance);
    }
}
