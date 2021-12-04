const MAX_POINTS: u32 = 100_000;

fn main() {
    immutable_variable();
    mutable_variable();
    constant_variable();
    shadowing1();
    shadowing2();
}

fn immutable_variable() {
    let x = 5; // default 불변성 변수, 변수 선언 시 let 키워드를 사용한다
    println!("The value of x is: {}", x);
    // x = 6;    -- 컴파일 에러 발생, 러스트는 기본적으로 immutable 변수를 선언하기 때문
    println!("The value of x is: {}", x);
}

fn mutable_variable() {
    let mut x = 5; // mut 키워드를 접두어로 붙인다
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}

fn constant_variable() {
    println!("the value of MAX_POINTS: {}", MAX_POINTS);
}

fn shadowing1() {
    let x = 5;

    let x = x + 1; // 처음 선언된 x 변수를 let x 구문으로 shadowing

    let x = x * 2; // 세번째 let x로 두번째 x를 shadowing

    println!("The value of x is: {}", x);
}

fn shadowing2() {
    let spaces = "   ";
    let spaces = spaces.len();

    println!("spaces length : {}", spaces);
}
