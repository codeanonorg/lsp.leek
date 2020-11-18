use pest::Parser;

#[macro_use]
extern crate pest_derive;

#[derive(Parser)]
#[grammar = "leekscript.pest"]
pub struct LeekParser;

#[test]
fn test_main() {
    let successful_parse = LeekParser::parse(
        Rule::File,
        "var a = 1;
        function test(c) {
            print(1);
        }",
    );
    match successful_parse {
        Ok(ast) => println!("{}", ast),
        Err(err) => println!("{}", err),
    }
}
