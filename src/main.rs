#![allow(dead_code)]
#![allow(unused_variables)]
extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::{Rng, thread_rng};

fn main() {
    // run_guessing_game();
    // run_variable();
    // run_data_type();
    // run_function();
    run_if();
}

fn run_if() {
    let num = 10;
    if num == 10 {
        println!("if");
    }

    let result = if num == 10 {
        20
    } else {
        300
    };

    println!("{}", result);

    loop {
        println!("loop");
        break;
    }

    let mut count = 0;

    while count < 5 {
        println!("count: {}", count);
        count = count + 1;
    }

    for num in [1,2,3,4].iter() {
        println!("for: {}", num);
    }

    for num in (1..4).by_ref() {
        println!("range: {}", num);
    }
}

fn run_function() {
    test_a(10, 10.1);
}

fn test_a(a: i32, b: f32) {
    println!("{}, {}", a, b);
}

fn run_data_type() {
    // 러스트는 기본적으로 모두 타입을 가져야 한다, parse 함수는 숫자 타입을 다 가질 수 있으므로 타입을 지정해주어야 한다
    let guess: u32 = "42".parse().expect("error");
    println!("guess - {}", guess);

    // 정수, 부동소수, boolean, 문자 타입을 스칼라 타입이라 한다
    let scalar: u32 = 10;
    // isize, usize는 cpu의 아키텍처에 따라 비트가 결정됨 (32bit, 64bit)
    let i_size: isize = -100;
    let u_size: usize = 100;
    println!("{}, {}, {}", scalar, i_size, u_size);

    let t = true;
    let g:bool = false;
    let z = 'z';
    let emoji = '😀';
    let tup: (i32, f64, bool) = (1, 2.2, false);
    let (a, b, c) = tup;
    let i = tup.0;
    let f = tup.1;
    let b = tup.2;
    let nums = [1, 2, 3, 4];
    // let pick_num = nums[5]; // 잘못된 색인 접근은 메모리에 접근하지 않음

}

fn run_variable() {
    // 상수는 타입이 반드시 필요하다
    const MAX_POINT: u32 = 10000;

    // mut는 가변성을 나타냄
    let mut x = 5;
    println!("x - {}", x);
    x = 10;
    println!("x - {}", x);

    let y = 10;
    // shadowing이 가능하다
    let y = y + 20;
    // 이전의 값을 계속해서 참조하여 shadowing 할 수 있다
    let y = y + 20;
    println!("y - {}", y);

    let z = "    ";
    // shadowing은 같은 변수에 다른 유형의 데이터를 넣을 수 있다, 이와 반대로 mut은 형변환을 허용하지 않는다
    let z = z.len();
    println!("z - {}", z);
}

fn run_guessing_game() {
    println!("Guess the number!");

    let mut thread_random = thread_rng();
    let secret_number = thread_random.gen_range(0..100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let number_guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Error: Please input number");
                continue;
            },
        };

        println!("You guessed: {}", number_guess);

        match number_guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            },
        }
    }
}
