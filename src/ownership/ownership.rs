pub fn fail_move_ownership() {
    let i_am_on_stack: i64 = 7427466391;
    let me_too = i_am_on_stack;

    println!("i_am_on_stack is {}", i_am_on_stack);
    println!("me_too is {}", me_too);

    let i_am_on_heap = vec![500, 60000];
    let (size, vector) = return_params_length(i_am_on_heap);

    println!("{size}");
    println!("{:?}", vector);
}

fn return_params_length(params: Vec<i32>) -> (usize, Vec<i32>) {
    let length = params.len();

    (length, params)
}
