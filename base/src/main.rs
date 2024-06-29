use papier::{
    convenience::call_static,
    papervm::{CharCell, PaperVM, CHARS_PER_FLOAT},
    programs::*,
};

fn main() {
    let mut vm = PaperVM::<CharCell>::new(call_static(
        gcd_with_mod(),
        vec![98765432., 1234567.],
        CHARS_PER_FLOAT,
    ));
    vm.run();
    println!("{}", vm.print());
    dbg!(vm.result::<f64>().unwrap());

    // let a = 127.;
    // let b = 1322.;

    // dbg!(gcd_main(127., 1322.));

    // let program = papier::programs::GCD;

    // let mut vm = PaperVM::<CharCell>::new(program);
    // vm.run();

    // let result: f64 = vm.result().unwrap();
    // println!("GCD: {}", result);
}