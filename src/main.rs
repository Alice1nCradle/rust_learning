mod ctfe;
mod statement_and_expression;
mod hello;
mod variable_and_binding;

fn main() {
    //0. say hello to the viewer!
    hello::main();
    //1. about the statement and expression
    statement_and_expression::about();
    //2. about the variable and binding
    variable_and_binding::about();
    //about the CTFE
    ctfe::about();
}
