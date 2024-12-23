use proc_lib_test3::before_caller;
use proc_lib_test3::callee_create;
use proc_lib_test3::callee_log;
use proc_lib_test3::callee_log1;
use proc_lib_test3::excute_block;
use proc_lib_test3::AnswerFn;
use proc_lib_test3::ObjectShowTrait;
use proc_lib_test3::StructHelloFn;

#[callee_log]
fn test_func2(i: i32) {
    println!("test_func2");
}

// #[callee_log1]
// fn test_func3() {
//     println!("called test_func3");
// }

#[callee_log1(Id = 3)]
fn test_func4() {
    println!("called test_func4");
}

// #[callee_log1(Id = 3, Name = 44)]
// fn test_func5() {
//     println!("called test_func5");
// }

#[callee_create(Debug)]
fn test_func6(a: i32, b: &str, c: String) -> Option<i32> {
    println!("processing test_func6");
    Some(a)
}

#[derive(AnswerFn, Debug, StructHelloFn)]
struct A;

#[derive(ObjectShowTrait)]
struct B {
    pub _a: i32,
    pub _b: i16,
    pub _str: String,
}

#[before_caller]
fn test_func1d(_p1: i32, _p2: &str, _p31: String) -> Option<i32> {
    println!("A:{} B:{} C:{}", _p1, &_p2, &_p31);
    Some(_p1)
}

/// .
fn main() {
    excute_block!({
        println!("进入过程宏");
        let sum = 2 + 3;
        println!("退出过程宏{}", sum);
    });

    let a = 13;

    execute();

    println!("Answer:{}", answer());

    A::struct_hello();

    let mut _add = B {
        _a: 13,
        _b: 26,
        _str: String::from("dwdw"),
    };
    _add.print_fields();

    let c = test_func1d(13, "dd", String::from("dwdwdw"));

    match c {
        Some(_v) => println!("return:{}", _v),
        None => println!("return non!"),
    }
    test_func2(22);

    test_func4();

    let c = Debug_test_func6(11, "dw", String::from("ddd"));
}
