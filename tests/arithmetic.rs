use cucumber::{given, then, when, World};
use arithmetic::{add, subtract, multiply, divide};

#[derive(Debug, Default, World)]
pub struct ArithWorld {
    a: i32,
    b: i32,
    result: Option<i32>,
    error: Option<String>,
}

#[given(regex = r"I have two numbers (\d+) and (\d+)")]
async fn given_numbers(world: &mut ArithWorld, a: i32, b: i32) {
    world.a = a;
    world.b = b;
}

#[when("I add them")]
async fn when_add(world: &mut ArithWorld) {
    world.result = Some(add(world.a, world.b));
}

#[when("I subtract them")]
async fn when_subtract(world: &mut ArithWorld) {
    world.result = Some(subtract(world.a, world.b));
}

#[when("I multiply them")]
async fn when_multiply(world: &mut ArithWorld) {
    world.result = Some(multiply(world.a, world.b));
}

#[when("I divide them")]
async fn when_divide(world: &mut ArithWorld) {
    match divide(world.a, world.b) {
        Ok(res) => world.result = Some(res),
        Err(e) => world.error = Some(e),
    }
}

#[then(regex = r"I should get (\d+)")]
async fn then_result(world: &mut ArithWorld, expected: i32) {
    assert_eq!(world.result.unwrap(), expected);
}

#[then(regex = r"I should get an error \"([^\"]*)\"")]
async fn then_error(world: &mut ArithWorld, expected: String) {
    assert_eq!(world.error.as_ref().unwrap(), &expected);
}

#[tokio::main]
async fn main() {
    ArithWorld::run("features/arithmetic.feature").await;
}
