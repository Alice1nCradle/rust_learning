pub(crate) fn about() {
    //show the order of the part
    index();
    //show the introduction of the part
    introduction();
    //and then, show up the content
    place_and_value_info();
    about_place_expression();
    about_value_expression();
    about_let();
    ownership_and_borrow();
}

fn index()
{
    println!("2. Variable and Binding");
}

fn introduction()
{
    println!("In Rust, we always use 'let' to create a variable. ");
    println!("And we also call this way 'Binding'.");
    println!("Because it show the connection between Identifier and Value.");
    println!("\n")
}

fn place_and_value_info()
{
    println!("First, let's introduce the expression");
    println!("In Rust, Expression is always divided into two parts: \n \
    Place Expression and Value Expression, also called LValue and Rvalue in \
    another languages.\n");
}

fn about_place_expression()
{
    println!("Place Expression includes local variable, *expr, expr[expr], expr.field and so on.");
    println!("It is used to write, to edit the memory of some data units.");
    println!("It is always become the constant data.")
}

fn about_value_expression()
{
    println!("Value Expression is the rest of all Expression.");
    println!("It just quotes the data, can only be used to read.");
    println!("It is always become the temporary data.")
}

fn about_let()
{
    println!("Second, let's introduce the let ");
    println!("When we only use 'let', the variable will be immutable.");
    println!("If we want a mutable variable, we should use 'let mut'");
    println!("Take, a simple function, for example.");
    fn mutable_or_not()
    {
        let a = 1;
        println!("In function mutable_or_not(), we 'let' a immutable variable 'a'");
        println!("Its value is {}, and after that we can't let it be any other value.", a);
        // a = 2; // No, don't do that in such a stupid way.
        let mut b = 2;
        println!("And then we 'let' a mutable variable 'b'");
        println!("Its value is {}, but this time we can turn it to any other value.", b);
        b = 3;
        println!("In this program, we see that this time the value of b is turned to {} \n", b);
    }
    mutable_or_not();
}

fn ownership_and_borrow()
{
    println!("Third, 'To do, to be mutable.' let the Rust become a language which is good at memory security.");
    println!("But, why does Rust also have such a good speed?");
    println!("Because it does not have GC, there's no need to listen in step by step.");
    println!("To prove its security, Rust define the lifetime and ownership.");
    fn ownership()
    {
        println!("Take, a function, for example.\n");
        let place1 = "Hello";
        // let place2 = "Hello".to_string();
        // let other = place1;
        println!("let 'other' get the ownership of 'place1', no problem.");
        println!("{:?}", place1);
        println!("However, when we try to let 'other' continues to get the ownership of 'place2', \
        something must go wrong.");
        println!("We let more than one variable move for one variable, it's unsafe for Rust.");
        println!("It may cause the 'dirty data'! \n");
        /*
        let other = place2;
        println!("{:?}", place2);  Error:other value used here after move.
         */
    }
    ownership();
    println!("So, Rust use the 'borrow' to solve such a embarrassed problem.");
    println!("Take, another function, for example.");
    lifetime_or_not();
    fn lifetime_or_not()
    {
        let a = [1, 2, 3];
        let b = &a;
        println!("We let variable 'b' borrows the ownership of variable 'a'");
        println!("And then, we note the location of the 'b' first, it's:");
        println!("The location of 'b' is {:p}", b);
        println!("Let's get the value of 'a', it's");
        println!("The value of 'a' is {:?}", a);
        println!("See? the 'a' is not shadowed.");
        let mut c = vec![1, 2, 3];
        let d = &mut c;
        println!("Just like the mutable borrow, we need a mutable binding first.");
        d.push(4);
        println!("And this time we can get the value of the 'd':");
        println!("{:?}", d);
        let e = &42;
        assert_eq!(*e, 42);
        println!("We need '*' to get the borrowed value.");
        println!("As for value expression, when it is borrowed, Rust will create a temporary value for it.")
    }

}