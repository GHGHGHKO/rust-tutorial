pub fn fail_move_ownership() {
    let i_am_on_stack: i64 = 7427466391;
    let me_too = i_am_on_stack;

    println!("i_am_on_stack is {}", i_am_on_stack);
    println!("me_too is {}", me_too);

    let i_am_on_heap = vec![500, 60000];
    print_function(&i_am_on_heap);

    let me_too = i_am_on_heap;
    print_function(&me_too);
}

fn print_function(params: &Vec<i32>) {
    println!("{:?}", &params);
}
