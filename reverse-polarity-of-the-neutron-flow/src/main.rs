fn display_neutron_flow(polarity: isize) {
    println!(
        "Neutron Flow is {}",
        if polarity < 0 { "reversed" } else { "normal" }
    );
}

fn main() {
    let polarity = 1;
    {
        let polarity = polarity - 2;
        display_neutron_flow(polarity);
    }
    display_neutron_flow(polarity);
}
/*
Compiling reverse-polarity-of-the-neutron-flow v0.1.0 (/Users/squeeko/RustBrainTeasers/reverse-polarity-of-the-neutron-flow)
    Finished dev [unoptimized + debuginfo] target(s) in 1.07s
     Running `target/debug/reverse-polarity-of-the-neutron-flow`
Neutron Flow is reversed
Neutron Flow is normal

The code creates a variable named polarity. It then creates another variable
with the same name and a different value. Reusing variable names is known
as shadowing and is a controversial topic in Rust development houses. Rust
explicitly permits the creation of shadowed variables; however, unlike many
other languages, you won’t see a compiler warning when using this feature.

Shadow variables don’t replace the name binding until they complete, so
you can access the previous version of a variable in the assignment
statement.

Creating a shadow variable doesn’t affect the original variable. You
haven’t magically changed it to be mutable; the new variable has its own
space in memory. All that changes is that the name binding now points
to the new value.

Once you’ve shadowed a variable, you can’t access the original variable
until the new binding leaves scope.

Shadowed variables don’t have to retain the same type or mutability
requirements as the previous variable. In fact, they don’t have to be
related at all because it’s an entirely new variable.

The initial—but unavailable—variable is still occupying memory. If
you’re concerned about RAM usage, shadowing won’t help. The Rust
compiler will often remove the unused variable for you, but it’s better to
be certain that it has been removed.

Scope and Shadowing (Rust by Example)
https://doc.rust-lang.org/rust-by-example/variable_bindings/scope.html
*/
