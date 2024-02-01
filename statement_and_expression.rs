pub(crate) fn about() {
    //show the order of the part
    index();
    //show the introduction of the part
    introduction();
    //and then, show up the content
    about_statement();
    about_expression();
}

fn index ()
{
    println!("1. Statement and Expression");
}
fn introduction()
{
    println!("In Rust, the grammar can always be divided into two parts: \n \
    Statement and Expression");
    println!("\nOf course, these parts can also be divided into many parts.\n");
}

fn about_statement()
{
    println!("First, let's start with Statement.\n");
    println!("Statement includes 'Declaration Statement' and 'Expression Statement'");
    statement_example();
}

fn statement_example()
{
    println!("For example:");
    println!("Declaration Statement is used to declare Item such as value, constant value, \
    struct, function and so on.");
    println!("What's more, it also includes extern, use and mod to import some crates and packages.");
    println!("When you look at the source code, especially there, you will understand.")
}

fn about_expression()
{
    println!("Second, let's start with Expression.");
    println!("The expression is used to calculation");
    fn sum()
    {
        println!("Assume that there is a function named sum(), and it can plus two numbers.");
        println!("Let them be a and b");
        let a = 114514;
        let b = 1919810;
        println!("And the final answer is {}\n", a + b);
    }
    println!("Take a function for example:");
    sum();
}