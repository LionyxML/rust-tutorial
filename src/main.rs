#![allow(unused)]

use rand::Rng;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};
use std::ops::Add;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

mod restaurant;
use crate::restaurant::order_food;

fn most_basics() {
    println!("Hello, world!");

    println!("Hey! Wazap?");

    let mut name = String::new();
    let sur_name = String::new();
    let greeting = "Nice to meet you!";

    println!("What's your name?");
    io::stdin()
        .read_line(&mut name)
        .expect("Didn't receive input... :(");

    println!("Hello {}! {}", name.trim_end(), greeting);
}

fn variables_basics() {
    const ONE_MIL: u32 = 1_000_000;
    const PI: f32 = 3.141592;

    // you can define variables with the same name but with different data types (shadowing in Rust)
    let age = "47";
    let mut age: u32 = age.trim().parse().expect("Age was'n assigned a number");

    age = age + 1;
    println!("I'm {} and I want ${}", age, ONE_MIL);

    println!("Max u32 : {}", u32::MAX);
    println!("Max u64 : {}", u64::MAX);
    println!("Max usize : {}", usize::MAX);
    println!("Max u128 : {}", u128::MAX);
    println!("Max f32 : {}", f32::MAX);
    println!("Max f64 : {}", f64::MAX);

    let is_true = true;
    let _is_true = true;
    let is_false = false;

    let my_grade = 'A';
}

fn math_basics() {
    let num_1: f32 = 1.111111111111111;
    println!("f32 : {}", num_1 + 0.111111111111111);

    let num_2: f64 = 1.111111111111111;
    println!("f64 : {}", num_2 + 0.111111111111111);

    let mut num_3: u32 = 5;
    let num_4: u32 = 4;
    println!("5 + 4 = {}", num_3 + num_4);
    println!("5 - 4 = {}", num_3 - num_4);
    println!("5 * 4 = {}", num_3 * num_4);
    println!("5 / 4 = {}", num_3 / num_4);
    println!("5 % 4 = {}", num_3 % num_4);

    num_3 += 1;
    println!("{} + 1 = {}", num_3, num_3);
}

fn random_basics() {
    let random_num = rand::thread_rng().gen_range(1..101);
    println!("Random : {}", random_num)
}

fn control_basics() {
    let age = 8;

    if (age >= 1) && (age <= 18) {
        println!("Important Birthday!");
    } else if (age == 21) || (age == 50) {
        println!("Important Birthday!");
    } else if age >= 65 {
        println!("Important Birthday!");
    } else {
        println!("NOT Important Birthday!");
    }

    let mut my_age = 47;
    let can_vote = if my_age >= 18 { true } else { false };
    println!("Can vote : {}", can_vote);

    let age2 = 8;
    match age2 {
        1..=18 => println!("Important Birthday!"),
        21 | 50 => println!("Important Birthday!"),
        65..=i32::MAX => println!("Important Birthday!"),
        _ => println!("NOT Important Birthday!"),
    }

    my_age = 18;
    let voting_age = 18;
    match my_age.cmp(&voting_age) {
        Ordering::Less => println!("Can't Vote"),
        Ordering::Greater => println!("Can Vote"),
        Ordering::Equal => println!("You gained the right to vote"),
    }
}

fn arrays_basics() {
    let arr_1 = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    println!("1st : {}", arr_1[0]);
    println!("Length : {}", arr_1.len());

    let arr_2 = [1, 2, 3, 4, 5, 6, 7, 8, 9];

    let mut loop_idx = 0;
    loop {
        if arr_2[loop_idx] % 2 == 0 {
            loop_idx += 1;
            continue;
        }
        if arr_2[loop_idx] == 9 {
            break;
        }
        println!("Val : {}", arr_2[loop_idx]);
        loop_idx += 1;
    }

    loop_idx = 0;
    while loop_idx < arr_2.len() {
        println!("Arr : {}", arr_2[loop_idx]);
        loop_idx += 1;
    }

    for val in arr_2.iter() {
        println!("Val : {}", val);
    }
}

fn tuples_basics() {
    let my_tuple: (u8, String, f64) = (47, "Rahul".to_string(), 50_0000.00);

    println!("Name : {}", my_tuple.1);

    let (v1, v2, v3) = my_tuple;

    println!("Age : {}", v1);
}

fn strings_basics() {
    let mut str1 = String::new();
    str1.push('A');
    str1.push_str(" word");

    for word in str1.split_whitespace() {
        println!("{}", word);
    }

    let str2 = str1.replace("A", "Another");
    println!("{}", str2);

    let str3 = String::from("x r t b h k k a m c");
    let mut v1: Vec<char> = str3.chars().collect();
    v1.sort();
    v1.dedup();
    for char in v1 {
        println!("{}", char);
    }

    let str4 = "Random string";
    let mut str5 = str4.to_string();
    println!("{}", str5);

    let byte_arr1 = str5.as_bytes();
    let str6 = &str5[0..6];
    println!("String length : {}", str6.len());

    str5.clear();

    let str6 = String::from("Just some");
    let str7 = String::from(" words...");
    let str8 = str6 + &str7; // after this str6 does not exists anymore, str7 does

    for char in str8.bytes() {
        println!("{}", char)
    }
}

fn casting_basics() {
    let int_u8: u8 = 5;
    let int2_u8: u8 = 4;

    let int3_u32: u32 = (int_u8 as u32) + (int2_u8 as u32);
}

fn enums_basics() {
    enum Day {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday,
    }

    impl Day {
        fn is_weekend(&self) -> bool {
            match self {
                Day::Saturday | Day::Sunday => true,
                _ => false,
            }
        }
    }

    let today: Day = Day::Monday;

    match today {
        Day::Monday => println!("Everyone hates Monday"),
        Day::Tuesday => println!("Donuts day!"),
        Day::Wednesday => println!("Hump day!"),
        Day::Thursday => println!("Pay day!"),
        Day::Friday => println!("Almost weekend"),
        Day::Saturday => println!("Weekend"),
        Day::Sunday => println!("Weekend"),
    }

    println!("Is today the weekend? {}", today.is_weekend());
}

fn vectors_basics() {
    let vec1: Vec<i32> = Vec::new();
    let mut vec2 = vec![1, 2, 3, 4];

    vec2.push(5);
    println!("1st : {}", vec2[0]);

    let second: &i32 = &vec2[1];
    match vec2.get(1) {
        Some(second) => println!("2nd : {}", second),
        None => println!("No second value"),
    }

    for i in &mut vec2 {
        *i *= 2;
    }
    for i in &vec2 {
        println!("{}", i);
    }

    println!("Vec Length {}", vec2.len());
    println!("Pop : {:?}", vec2.pop())
}

fn functions_basics() {
    fn say_hello() {
        println!("Hello");
    }

    fn get_sum(x: i32, y: i32) {
        println!("{} + {} = {}", x, y, x + y)
    }

    fn get_sum2(x: i32, y: i32) -> i32 {
        x + y
    }

    fn get_sum3(x: i32, y: i32) -> i32 {
        return x + y;
    }

    say_hello();
    get_sum(3, 4);
    println!("{} + {} = {}", 3, 4, get_sum2(3, 4));
    println!("{} + {} = {}", 3, 4, get_sum3(3, 4));

    fn get_2(x: i32) -> (i32, i32) {
        return (x + 1, x + 2);
    }

    let (val_1, val_2) = get_2(10);
    println!("Nums : {} {}", val_1, val_2);

    fn sum_list(list: &[i32]) -> (i32) {
        let mut sum = 0;
        for &val in list.iter() {
            sum += &val;
        }
        sum
    }

    let num_list = vec![1, 2, 3, 4, 5];
    println!("Sum of list = {}", sum_list(&num_list));
}

fn generics_basics() {
    // fn get_sum_gen1<T>(x: T, y: T) -> T {
    //     // return x + y;   // no can't do it, since + cannot be used in generics, solution, traits
    // }

    fn get_sum_gen2<T: Add<Output = T>>(x: T, y: T) -> T {
        return x + y;
    }

    println!("5 + 4 = {}", get_sum_gen2(5, 4));
    println!("5.2 + 4.6 = {}", get_sum_gen2(5.2, 4.6));
}

fn ownership_basics() {
    // Heap: When putting data on the heap you request a certain amount of space. The OS
    // finds space available and returns an address for that space called a pointer.

    // Rules:
    // 1. Each value has a variable that's called its owner
    // 2. There's only one owner at a time
    // 3. When the owner goes out of scope the value disappears

    let str1 = String::from("World");

    //    let str2 = str1;
    //    println!("Hello {}", str1); // error: borrow of moved value str1

    let str3 = str1.clone();
    println!("Hello {}", str1);

    // This wont apply with datatypes such as: int, bool, char, float, tuples
    // It will aply with strings, vectors, arrays, etc.

    fn print_str(x: String) {
        println!("A string {}", x);
    }

    fn print_return_str(x: String) -> String {
        println!("A string {}", x);
        x
    }

    fn change_string(name: &mut String) {
        name.push_str(" is happy");
        println!("Message : {}", name);
    }

    // print_str(str1);

    let str4 = print_return_str(str1); // uncomment above and it does not work anymore

    println!("str4 : {}", { str4 });
    // println!("str4 : {}", { str4 }); // if you try again, no can't do, it has already been used

    let mut str5 = String::from("Rahul");
    change_string(&mut str5);
    println!("{}", &str5);
    println!("{}", &str5);
    println!("{}", &str5);
}

fn hashmaps_basics() {
    let mut heroes = HashMap::new();
    heroes.insert("Superman", "Clark Kent");
    heroes.insert("Batman", "Bruce Wayne");
    heroes.insert("The Flash", "Barry Allen");

    for (k, v) in heroes.iter() {
        println!("{} = {}", k, v);
    }

    println!("Lenght : {}", heroes.len());

    if heroes.contains_key(&"Batman") {
        let the_batman = heroes.get(&"Batman");

        match the_batman {
            Some(x) => println!("Batman is a hero"),
            None => println!("Batman is not a hero"),
        }
    }
}

fn struct_basics() {
    struct Costumer {
        name: String,
        address: String,
        balance: f32,
    }

    let mut bob = Costumer {
        name: String::from("Bob Lazar"),
        address: String::from("51 area"),
        balance: 555.50,
    };

    bob.address = String::from("California 55");

    struct Rectangle<T, U> {
        length: T,
        height: U,
    }

    let rec = Rectangle {
        length: 4,
        height: 10.5,
    };
}

fn trait_basics() {
    const PI: f32 = 3.141592;

    // Traits can be used by any structs that implement the correct trait
    trait Shape {
        fn new(length: f32, width: f32) -> Self;
        fn area(&self) -> f32;
    }

    struct Rectangle {
        length: f32,
        width: f32,
    }

    struct Circle {
        length: f32,
        width: f32,
    }

    impl Shape for Rectangle {
        fn new(length: f32, width: f32) -> Rectangle {
            return Rectangle { length, width };
        }
        fn area(&self) -> f32 {
            return self.length * self.width;
        }
    }

    impl Shape for Circle {
        fn new(length: f32, width: f32) -> Circle {
            return Circle { length, width };
        }
        fn area(&self) -> f32 {
            return (self.length / 2.0).powf(2.0) * PI;
        }
    }

    let rec: Rectangle = Shape::new(10.0, 10.0);
    let cir: Circle = Shape::new(10.0, 10.0);

    println!("Rec Area: {}", rec.area());
    println!("Cir Area: {}", cir.area());
}

fn modules_basics() {
    // Crates: Modules that produce a library or executable
    // Modules: Organize and handle privacy
    // Packages: Build, test and share crates
    // Paths: A way of naming an item such as a struct, function

    // take a look at ./restaurant/mod.rs

    // take a look at the top of this file:
    // mod restaurant;
    // use crate::restaurant::order_food;

    order_food();
}

fn errors_basics_and_files() {
    // forced error:
    // panic!("Terrible Error!");

    let lil_arr = [1, 2];

    // forced error:
    // println!("{}", lil_arr[10])

    // File returns Result
    // Result has 2 varients Ok and Error
    // enum Result<T, E> {
    // Ok(T),
    // Err(E), }
    // Where T represents the data typeof the value returns
    // E the type of error

    let path = "lines.txt";
    let output = File::create(path);

    let mut output = match output {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem creating file. Aborted. {:?}", error)
        }
    };

    write!(output, "Just some\nRandom words").expect("Failed to write to the file");

    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);
    for line in buffered.lines() {
        println!("{}", line.unwrap())
    }

    let output2 = File::create("random.txt");
    let output2 = match output2 {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("random.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Can't create file: {:?}", e),
            },
            _other_error => panic!("Problem opening file : {:?}", error),
        },
    };
}

fn iterators_basics() {
    let mut arr_it = [1, 2, 3, 4];

    for val in arr_it.iter() {
        // an iterator will cycle the values trough borrowing
        // downside: u can't modify the original
        println!("{}", val);
    }

    arr_it.into_iter(); // but you'll consume it at the end

    let mut iter1 = arr_it.iter();

    println!("1st : {:?}", iter1.next())
}

fn closures_basics() {
    // let var_name = |parameters| -> return_type { BODY }

    let can_vote = |age: i32| age >= 18;
    println!("Can vote: {}", can_vote(28));

    let mut samp1 = 5;
    let print_var = || println!("samp1 = {}", samp1);
    print_var();

    samp1 = 10;
    let mut change_var = || samp1 += 1;
    change_var();
    println!("samp1 = {}", samp1);

    samp1 = 10;
    println!("samp1 = {}", samp1);

    fn use_func<T>(a: i32, b: i32, func: T) -> i32
    where
        T: Fn(i32, i32) -> i32,
    {
        func(a, b)
    }

    let sum = |a, b| a + b;
    let pro = |a, b| a * b;

    println!("5 + 4 = {}", use_func(5, 4, sum));
    println!("5 * 4 = {}", use_func(5, 4, pro));
}

fn smart_pointer_basics() {
    // Stack: Stores values in a last in first out format
    // Data on the stach must have a defined fixed size
}

fn box_basics() {
    let b_int1 = Box::new(10);

    println!("b_int1 = {}", b_int1);

    //       50
    //      /  \
    //     35  40
    //    / \  / \

    struct TreeNode<T> {
        pub left: Option<Box<TreeNode<T>>>, // try only with TreeNode<T> and look at the linter
        pub right: Option<Box<TreeNode<T>>>,
        pub key: T,
    }

    impl<T> TreeNode<T> {
        pub fn new(key: T) -> Self {
            TreeNode {
                left: None,
                right: None,
                key,
            }
        }
        pub fn left(mut self, node: TreeNode<T>) -> Self {
            self.left = Some(Box::new(node));
            self
        }
        pub fn right(mut self, node: TreeNode<T>) -> Self {
            self.right = Some(Box::new(node));
            self
        }
    }

    let node1 = TreeNode::new(1)
        .left(TreeNode::new(2))
        .right(TreeNode::new(3));
}

fn concurrency_basics() {
    // Common Problems with parallel programming involve:
    // 1. Thread are accessing data in the wrong order;
    // 2. Threads are blocked from executing because of confusion
    // over requirements to proceed with execution

    // Basic thread and a main thread
    let mut thread1 = thread::spawn(|| {
        for i in 1..25 {
            println!("Spawned thread : {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..20 {
        println!("Main thread : {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    // in the form above there's no garanties when or even IF they'll be executed
    // that can be secured with join

    thread1.join().unwrap();
}

fn concurrency_example() {
    pub struct Bank {
        balance: f32,
    }

    // This won't work because we cant have a situation when a clojure
    // may outlive the current function. Specially because it is borrowing.
    //

    // fn withdraw(the_bank: &mut Bank, amt: f32) {
    //     the_bank.balance -= amt;
    // }

    // let mut bank = Bank { balance: 1000.0 };

    // withdraw(&mut bank, 5.00);
    // println!("Balance : {}", bank.balance);

    // fn customer(the_bank: &mut Bank) {
    //     withdraw(the_bank, 5.00);
    // }

    // thread::spawn(|| {
    //     customer(&mut bank);
    // })
    // .join()
    // .unwrap()

    // One of the fixes for it is smart pointers

    fn withdraw(the_bank: &Arc<Mutex<Bank>>, amt: f32) {
        let mut bank_ref = the_bank.lock().unwrap();
        if bank_ref.balance < 5.00 {
            println!(
                "Current balance : {} Withdraw a smaller amount",
                bank_ref.balance
            );
        } else {
            bank_ref.balance -= amt;
            println!(
                "Customer widthdrew : {} Current Balance : {}",
                amt, bank_ref.balance
            );
        }
    }

    fn customer(the_bank: Arc<Mutex<Bank>>) {
        withdraw(&the_bank, 5.00);
    }

    let bank: Arc<Mutex<Bank>> = Arc::new(Mutex::new(Bank { balance: 20.00 }));

    let handles = (0..10).map(|_| {
        let bank_ref = bank.clone();
        thread::spawn(|| customer(bank_ref))
    });

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Total {}", bank.lock().unwrap().balance);
}

fn main() {
    // most_basics();
    // variables_basics();
    // math_basics();
    // random_basics();
    // control_basics();
    // arrays_basics()
    // tuples_basics();
    // strings_basics();
    // casting_basics();
    // enums_basics();
    // vectors_basics();
    // functions_basics();
    // generics_basics();
    // ownership_basics();
    // hashmaps_basics();
    // struct_basics();
    // trait_basics();
    // modules_basics();
    // errors_basics_and_files();
    // iterators_basics();
    // closures_basics();
    // smart_pointer_basics();
    // box_basics();
    // concurrency_basics();
    // concurrency_example();
}
