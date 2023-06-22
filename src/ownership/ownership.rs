pub fn fail_move_ownership() {
    let i_am_on_stack: &str = "7427466391.com";
    let me_too = i_am_on_stack;

    println!("i_am_on_stack is {}", i_am_on_stack);
    println!("me_too is {}", me_too);

    let i_am_on_heap = String::from("Ferris");
    print_function(i_am_on_heap.clone());

    let me_too = i_am_on_heap.clone();
    print_function(me_too);
}

fn print_function(name: String) {
    println!("{}", name);
}
