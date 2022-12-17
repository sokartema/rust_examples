mod custom_types;
mod primitives;

fn get_function_name_and_call<F>(fun: F)
where
    F: Fn(),
{
    println!("\nCALLING ----> {} \n", std::any::type_name::<F>());
    fun();
}

fn main() {
    println!("-- PRIMITIVES --");
    get_function_name_and_call(primitives::literals_operators);
    get_function_name_and_call(primitives::tuples);
    get_function_name_and_call(primitives::arrays_and_slices);

    println!("-- CUSTOM_TYPES --");
    get_function_name_and_call(custom_types::structs);
}
