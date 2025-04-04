mod deriv;

fn f(x: f64) -> f64 {
    4.0 * x.powf(3.0) + 2.0 * x.powf(2.0) + x
}

fn main() {
    let fdev_fw = deriv::deriv(f, 1.0, &deriv::Method::Forward);
    let fdev_bw = deriv::deriv(f, 1.0, &deriv::Method::Backward);
    let fdev_ct = deriv::deriv(f, 1.0, &deriv::Method::Central);
    let fun = f(1.0);
    println!("Função: {fun}");
    println!("Derivada (forward): {} com {} iterações", fdev_fw.0, fdev_fw.1);
    println!("Derivada (backward): {} com {} iterações", fdev_bw.0, fdev_bw.1);
    println!("Derivada (central): {} com {} iterações", fdev_ct.0, fdev_ct.1);
}
