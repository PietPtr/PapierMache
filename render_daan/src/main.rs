use papier::{
    convenience::call_static,
    papervm::{instructions::circle, CharCell, PaperVM, CHARS_PER_FLOAT},
    programs::{gcd, gcd_with_mod},
};

pub mod render;

fn main() {
    let mut vm = PaperVM::<CharCell>::new(call_static(
        gcd_with_mod(),
        vec![1123., 127.],
        CHARS_PER_FLOAT,
    ));

    while !vm.step().is_finished() {
        println!("{}", vm.print());
    }

    println!("{}", vm.print());
    dbg!(vm.result::<f64>().unwrap());

    println!("Finished running");

    render::render_papers(vm);
}
