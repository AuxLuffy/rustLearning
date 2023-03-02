/**
 * 常量命名有以下注意的
 * --- 不可以使用mut，常量永远不可变
 * --- 声明常量使用const关键字，且其类型必须被标注
 * --- 常量可以在任何作用域内声明，包括全局
 * --- 常量只可以绑定到常量表达式，无法绑定到函数的调用结果或在运行时才能算出的值
 */
const A_B: i32 = 10;
// const PID:i32 = rand::thread_rng().gen_rangen(1..100);
const PI: f32 = 3.14 + 2222.0;
fn main() {
    const PID: i32 = 111;
    // another_function(32);
    // test_number_overflow();
    test_clouse_expression();
    test_type();
    tuple_test((32, true, 343333.0));
    test_array();
    println!("return: {}", function_with_return(0, 1));
    testwhile();
}

fn another_function(x: i32) {
    println!("the input is {}", x);
}
/**
 * 测试数字溢出
 */
fn test_number_overflow() {
    // let b: u32 = 25 * 200;
    // let a: u8 = (b / 100).try_into().unwrap();
    // let a = 'b'; //rust的隐藏特性
    // println!("b={}, a(b/10)={}", b, a);
}

fn test_clouse_expression() {
    let x = {
        let y = 6; //是语句，语句的返回值是空的tuple: '()'
        y;
    };
    println!("x={:?}", x);
    let x = 0b101; //默认let变量定义是不可变量，如果想让变量变为可变的需要加mut关键字
    let mut y = 333;
    y = x;
    println!("x={}", x);
}

fn test_type() {
    let t = (true, 123, false, 'a', "abc");
    println!("tuple元组t={:?}", t); //元组输出可以使用{:?}作为占位符
}
/**
 * 测试tuple元组当作函数参数
 */
fn tuple_test(x: (i32, bool, f64)) {
    let (age, is_male, money) = x; //元组解构，可以把元组的内容一个一个赋值给变量
    println!("Age is {}, isMale: {}, money is {}", age, is_male, money);
}

fn test_array() {
    let a = [2, 3, 4, 5];
    let a: [i32; 5] = [2, 3, 4, 5, 6]; //指定类型用分号和容量隔开
                                       //还可以用下面的方法初始化一个固定长度且内部元素相同的数组
    let a = [3; 5]; //会初始化一个长度为5且每个元素都是3的数组，相当于：let a = [3,3,3,3,3];
    let b = a[1];
    // let d = a[6];//编译通过但运行时会报错
    for c in a {
        print!("{} ", c);
    }
}

/**
 * 测试有返回值的方法
 */
fn function_with_return(x: i32, y: i32) -> i32 {
    if x > y || x < 3 {
        return x;
    } else if y > 2 {
        return y + 2;
    } else {
        println!("{}", y)
    }
    y
}

fn testwhile() {
    let mut a = 5;
    while a != 0 {
        println!("{}", a);
        a -= 1;
    }

    for a in ('a'..'z').rev(){
        print!("{}", a)
    }
}
