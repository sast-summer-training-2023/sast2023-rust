//! 本作业形式借鉴于cs61a，在此鸣谢
//! 
//! 请在对应的region进行代码修改，使得可以正常进行输出


fn task1() {
    // region 1 start
    int x = 4;

    // region 1 end

    // 输出: x = 4
    println!("x = {}", x);
}

fn task2() {
    // region 2 start
    const x:usize = 4;
    const x:usize = 5;
    // region 2 end

    // 输出: x = 5
    println!("x = {}", x);
}

fn calc(x: i32) -> i32 {
    // region 3 start
    if x > 4 {
        x
    }
    else {
        x / 2.0
    }
    // region 3 end
}

fn task3() {
    
    let x = 2;
    calc(x);

    // 输出: x = 1
    println!("x = {}", x);
}

fn task4() {
    let date = 724;
    
    // region 4 start
    let z = if date % 2 == 0 {
        4 + 2
    }
    else {
        2.0
    };
    // region 4 end

    // 输出: z = 4
    println!("z = {}", z);
}

fn task5() {
    // region 5 start

    // 输出: 0 1 2 3 4 5 6 7 8 9 10
    for i in 0..10 {
        print!("{} ", i);
    }
    // region 5 end
}

fn task6() {
    // region 6 start
    let x = 10;

    x = 1919810;
    // region 6 end

    // 输出: x = 1919810
    print!("x = {}", x);
}

fn task7() {
    // region 7 start
    let x = 10;

    let y = &mut x;

    *y = 10;
    // region 7 end

    // 输出: x = 10
    println!("x = {}", x);

}

fn task8() {
    // region 8 start
    let mut x = 8;

    let y = &mut x;
    let z = &x;

    *y = 10;
    // region 8 end

    // 输出: x = 10
    println!("x = {}", x);
}

fn output9(x: String) {
}

fn task9() {
    let x = String::from("hello world");
    // region 9 start
    output9(x);
    // region 9 end

    // 输出: x = hello world
    println!("x = {}", x);
}

fn task10() {
    let x = String::from("hello world");

    // region 10 start
    let y = x;
    // region 10 end

    // 输出: 
    // x = hello world
    // y = hello world
    println!("x = {}", x);
    println!("y = {}", y);
}

fn task11() {
    let mut x = String::from("hello world");
    let y = &mut x;
    
    // region 11 start
    y = String::from("hello rust");
    // region 11 end

    // 输出: x = hello rust
    println!("x = {}", x);
}

fn task12() {
    let mut x = String::from("hello world");
    // region 12 start
    let y = &x;
    *y = String::from("hello rust");
    // region 12 end

    // 输出: x = hello rust
    println!("x = {}", x);
}

fn task13() {
    let mut x = 10;
    // region 13 start
    // 请只进行语句交换，而不要修改语句内容
    let z = &mut x;
    let y = &x;
    *z = 20;
    // region 13 end

    // 输出: x = 20
    println!("x = {}", y);
}

fn task14() {
    let x = 10;
    // region 14 start
    match x {
        10 => println!("x = 10"),
        20 => println!("x = 20"),
    };
    // region 14 end

    // 输出: x = 10
}

fn task15() {
    let x = String::from("hello world");
    let y = String::from("hello rust");
    let z = String::from("hello world");
    // region 15 start
    match x {
        y => println!("x = hello world"),
        z => println!("x = hello rust"),
        _ => println!("x = others"),
    }
    // region 15 end

    // 输出: (注意有两个)
    // x = hello world
    // x = hello world
    print!("x = {}", x);
}

fn main() {
    task1();
    task2();
    task3();
    task4();
    task5();
    task6();
    task7();
    task8();
    task9();
    task10();
    task11();
    task12();
    task13();
    task14();
    task15();
}
