mod conversion;
mod custom_types;
mod primitives;
mod r_types;
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

    println!("-- TYPES --");
    get_function_name_and_call(r_types::casting);
    get_function_name_and_call(r_types::literals);
    get_function_name_and_call(r_types::inference);
    get_function_name_and_call(r_types::aliasing);

    println!("-- CONVERSION --");
    get_function_name_and_call(conversion::from_and_into);
    get_function_name_and_call(conversion::try_from_and_try_into);
    get_function_name_and_call(conversion::to_string_and_parse);
}
