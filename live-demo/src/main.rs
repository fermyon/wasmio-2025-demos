use bindings::wasmio::demo::sorting::sort;

mod bindings;

fn main() {
    let input = vec![3, 5, 6, 233, 3434, 1, 22323];
    let sorted = sort(&input);
    println!("Hello WasmIO. This is the sorted: {sorted:?}");
}
