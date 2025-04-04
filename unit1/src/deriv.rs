//TODO: reduce code duplication
//TODO (maybe): make the algorithm more elegant

fn func_forward(f: fn(f64) -> f64, x: f64, delta_x: f64) -> f64 {
    (f(x + delta_x) - f(x)) / delta_x
}

fn func_backward(f: fn(f64) -> f64, x: f64, delta_x: f64) -> f64 {
    (f(x) - f(x - delta_x)) / delta_x
}

fn func_central(f: fn(f64) -> f64, x: f64, delta_x: f64) -> f64 {
    (f(x + delta_x) - f(x - delta_x)) / (2.0 * delta_x)
}

pub fn forward(f: fn(f64) -> f64, x: f64) -> (f64, i32) {
    let mut k = 0;
    let mut delta_x = 1.0;
    let episilon = 0.00000001;
    let mut deriv_back = func_forward(f, x, delta_x);
    let mut deriv_now = deriv_back + 1.0;
    while (deriv_now - deriv_back).abs() >= episilon {
        k += 1;
        delta_x /= 10.0;
        deriv_back = deriv_now;
        deriv_now = func_forward(f, x, delta_x);
    }
    (deriv_now, k)
}

pub fn backward(f: fn(f64) -> f64, x: f64) -> (f64, i32) {
    let mut k = 0;
    let mut delta_x = 1.0;
    let episilon = 0.001;
    let mut deriv_back = func_backward(f, x, delta_x);
    let mut deriv_now = deriv_back + 1.0;
    while (deriv_now - deriv_back).abs() >= episilon {
        k += 1;
        delta_x /= 10.0;
        deriv_back = deriv_now;
        deriv_now = func_backward(f, x, delta_x);
    }
    (deriv_now, k)
}

pub fn central(f: fn(f64) -> f64, x: f64) -> (f64, i32) {
    let mut k = 0;
    let mut delta_x = 1.0;
    let episilon = 0.001;
    let mut deriv_back = func_central(f, x, delta_x);
    let mut deriv_now = deriv_back + 1.0;
    while (deriv_now - deriv_back).abs() >= episilon {
        k += 1;
        delta_x /= 10.0;
        deriv_back = deriv_now;
        deriv_now = func_central(f, x, delta_x);
    }
    (deriv_now, k)
}
