//TODO (maybe): make the algorithm more elegant
//TODO: make a version to multiple derivatives

pub enum Method {
    Forward,
    Backward,
    Central,
}

fn func_forward(f: fn(f64) -> f64, x: f64, delta_x: f64) -> f64 {
    (f(x + delta_x) - f(x)) / delta_x
}

fn func_backward(f: fn(f64) -> f64, x: f64, delta_x: f64) -> f64 {
    (f(x) - f(x - delta_x)) / delta_x
}

fn func_central(f: fn(f64) -> f64, x: f64, delta_x: f64) -> f64 {
    (f(x + delta_x) - f(x - delta_x)) / (2.0 * delta_x)
}

pub fn deriv(f: fn(f64) -> f64, x: f64, d: &Method) -> (f64, i32) {
    let mut k = 0;
    let mut delta_x = 1.0;
    let episilon = 0.00000001;
    let func: fn(fn(f64) -> f64, f64, f64) -> f64;
    match d {
        Method::Forward => {
            func = func_forward;
            println!("Metódo Forward");
        },
        Method::Backward => {
            func = func_backward;
            println!("Metódo Backward");
        },
        Method::Central => {
            func = func_central;
            println!("Metódo Central");
        }
    }
    let mut deriv_back = func(f, x, delta_x);
    let mut deriv_now = deriv_back + 1.0;
    while (deriv_now - deriv_back).abs() >= episilon {
        k += 1;
        delta_x /= 10.0;
        deriv_back = deriv_now;
        deriv_now = func(f, x, delta_x);
    }
    (deriv_now, k)
}
