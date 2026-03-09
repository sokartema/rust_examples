// ============================================================
// Functions in Rust
// Based on: https://doc.rust-lang.org/rust-by-example/fn.html
// ============================================================
//
// Functions are declared with `fn`. Rust is expression-based,
// so the final expression in a block is implicitly returned
// (no semicolon). Explicit `return` statements also work.

// Helper struct used by the `methods` example.
// Defined at module level so it's accessible to all functions here.
struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    // Associated function (no `self`) — called as `Rectangle::new(...)`
    fn new(width: f64, height: f64) -> Self {
        Rectangle { width, height }
    }

    // Methods take `&self` (immutable borrow of the receiver)
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }

    fn is_square(&self) -> bool {
        self.width == self.height
    }
}

// ── 1. Basic Functions ───────────────────────────────────────

pub fn basic_functions() {
    // Functions must declare their parameter types and return type.
    // The last *expression* (no semicolon) is the return value.
    fn add(a: i32, b: i32) -> i32 {
        a + b // expression — implicitly returned
    }

    // A statement ends with `;` and produces the unit value `()`.
    // An expression produces a value and has no trailing `;`.
    fn describe(n: i32) -> &'static str {
        if n > 0 {
            "positive" // expression branch
        } else if n < 0 {
            "negative"
        } else {
            "zero"
        }
    }

    // Recursive function — Rust has no tail-call optimisation guarantee,
    // but recursion works fine for moderate depths.
    fn factorial(n: u64) -> u64 {
        if n == 0 {
            1
        } else {
            n * factorial(n - 1)
        }
    }

    println!("add(3, 4) = {}", add(3, 4));
    println!("describe(7) = {}", describe(7));
    println!("describe(-3) = {}", describe(-3));
    println!("describe(0) = {}", describe(0));

    for n in 0..=7 {
        println!("{}! = {}", n, factorial(n));
    }
}

// ── 2. Methods ───────────────────────────────────────────────

pub fn methods() {
    // `Rectangle::new` is an *associated function* — like a static method.
    // It doesn't have a `self` parameter, so it's called with `::` syntax.
    let rect = Rectangle::new(5.0, 3.0);
    let square = Rectangle::new(4.0, 4.0);

    println!("rect   area      = {}", rect.area());
    println!("rect   perimeter = {}", rect.perimeter());
    println!("rect   is_square = {}", rect.is_square());
    println!("square is_square = {}", square.is_square());

    // Methods are called with `.` syntax on an instance.
    // Rust automatically borrows/dereferences as needed (auto-ref/deref).
    let r2 = Rectangle::new(10.0, 2.0);
    println!("r2 area = {:.2}", r2.area());
}

// ── 3. Closures ──────────────────────────────────────────────

pub fn closures() {
    // Closures are anonymous functions written as `|params| expression`.
    // Unlike regular functions, they can *capture* variables from their
    // enclosing scope.

    let double = |x: i32| x * 2;
    println!("double(5) = {}", double(5));

    // Type annotations on closure parameters are often optional — Rust infers
    // them from context.
    let add_one = |x| x + 1i32;
    println!("add_one(9) = {}", add_one(9));

    // Multi-statement closures use a block `{ ... }`.
    let verbose_square = |x: i32| {
        let result = x * x;
        println!("  squaring {} -> {}", x, result);
        result
    };
    verbose_square(7);

    // Capturing by reference (default when possible)
    let greeting = String::from("Hello");
    let greet = |name: &str| println!("{}, {}!", greeting, name);
    greet("Alice");
    greet("Bob");
    // `greeting` is still usable here because the closure only borrowed it.
    println!("greeting is still: {}", greeting);

    // Capturing by value with `move`
    // Forces the closure to take ownership of captured variables.
    let prefix = String::from(">> ");
    let print_with_prefix = move |msg: &str| println!("{}{}", prefix, msg);
    print_with_prefix("moved closure");
    // `prefix` is no longer accessible here (it was moved into the closure).
}

// ── 4. Higher-Order Functions ────────────────────────────────

pub fn higher_order_functions() {
    // A higher-order function (HOF) either accepts a function/closure as an
    // argument, or returns one.

    // --- Passing a closure as an argument ---
    fn apply<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
        f(x)
    }

    println!("apply(|x| x*x, 5) = {}", apply(|x| x * x, 5));
    println!("apply(|x| x+10, 3) = {}", apply(|x| x + 10, 3));

    // --- Returning a closure ---
    // `impl Fn(...)` lets us return a closure without boxing it.
    fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
        move |x| x + n
    }

    let add5 = make_adder(5);
    let add100 = make_adder(100);
    println!("add5(3) = {}", add5(3));
    println!("add100(3) = {}", add100(3));

    // --- Iterator adapters ---
    // `.map()` transforms each element, `.filter()` keeps matching elements,
    // `.fold()` reduces a sequence to a single value.

    let numbers: Vec<i32> = (1..=10).collect();

    // Sum of squares of even numbers
    let result: i32 = numbers
        .iter()
        .filter(|&&x| x % 2 == 0)       // keep evens: 2,4,6,8,10
        .map(|&x| x * x)                  // square them: 4,16,36,64,100
        .fold(0, |acc, x| acc + x);       // sum: 220

    println!("sum of squares of evens 1..=10 = {}", result);

    // Equivalent using `.sum()` instead of fold
    let result2: i32 = (1..=10)
        .filter(|x| x % 2 == 0)
        .map(|x| x * x)
        .sum();

    println!("same result via .sum() = {}", result2);

    // Collect filtered/mapped values into a new Vec
    let even_squares: Vec<i32> = (1..=6)
        .filter(|x| x % 2 == 0)
        .map(|x| x * x)
        .collect();
    println!("even squares 1..=6 = {:?}", even_squares);
}

// ── 5. Diverging Functions ───────────────────────────────────

pub fn diverging_functions() {
    // A *diverging function* never returns to its caller — it has the return
    // type `!` (called the "never" type). Because `!` can coerce to any type,
    // diverging functions can appear in any expression context.

    // `panic!` is the canonical diverging function.
    // It unwinds the stack and terminates the current thread.
    fn always_panics() -> ! {
        panic!("this function never returns normally")
    }

    // `unimplemented!()` and `todo!()` are diverging macros used as
    // placeholders during development.
    fn not_yet_done() -> ! {
        todo!("implement me later")
    }

    // Because `!` coerces to any type, diverging calls can appear in match
    // arms where only one branch needs to compute a value.
    fn safe_divide(a: i32, b: i32) -> i32 {
        match b {
            0 => panic!("division by zero!"), // diverges — type `!` → `i32`
            _ => a / b,
        }
    }

    println!("safe_divide(10, 2) = {}", safe_divide(10, 2));
    println!("safe_divide(7, 3) = {}", safe_divide(7, 3));

    // Demonstrate that `todo!()` diverges (we call it in a branch never taken)
    fn maybe_todo(use_todo: bool) -> i32 {
        if use_todo {
            todo!("branch not implemented")
        } else {
            42
        }
    }

    println!("maybe_todo(false) = {}", maybe_todo(false));

    // Show that always_panics and not_yet_done exist but don't call them,
    // since calling them would terminate the demo.
    println!("always_panics and not_yet_done are defined (but not called here)");

    // Suppress unused-variable warnings for the function items shown above.
    let _ = always_panics as fn() -> !;
    let _ = not_yet_done as fn() -> !;
}
