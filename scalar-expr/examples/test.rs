use scalar_expr::expression;

fn main() {
    let wow = 3;
    println!(
        "{:#?}",
        expression!(field:"x" == wow && field:"y" == 2 || 2 == 2)
    );
}
