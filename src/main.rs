mod custom_types;
mod primitives;
mod variable_bindings;

// Some fancy way to get the name of the function with the module and call it
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
    get_function_name_and_call(custom_types::enums);
    get_function_name_and_call(custom_types::consts);

    println!("-- VARIABLE BINDINGS --");
    get_function_name_and_call(variable_bindings::intro);
    get_function_name_and_call(variable_bindings::mutability);
    get_function_name_and_call(variable_bindings::scope_and_shadowing);
    get_function_name_and_call(variable_bindings::declare_first);
    get_function_name_and_call(variable_bindings::freezing);
}
