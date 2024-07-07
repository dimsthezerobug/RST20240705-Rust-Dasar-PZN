use std::{collections::btree_map::Values, fmt::format, mem, ops::RangeInclusive};
use first::say_hellobro;
use second::say_hellobro as say_hellobro_second;


mod third_module;
use third_module::say_hellomas as say_hellomas_third;
#[test]
fn test_crate() {
    say_hellomas_third();
}


mod first_module;
mod second_module;
use first_module::say_hellomas;
use second_module::say_hellomas as say_hellomas_second;

// mengambil semua member
// use first_module::*;
// use first_module::{say_hellomas, say_hellodek};

#[test]
fn module_terpisah() {
    say_hellomas();
    say_hellomas_second();
}

#[test]
fn super_keyword() {
    first_module::sub_first::sub_sub_first::say_hello_sub();
}

mod first {
    pub fn say_hellobro() {
        println!("Hello World!");
    }
}

mod second {
    pub fn say_hellobro() {
        println!("Hello World!");
    }
}

#[test]
fn test_use() {
    // tanpa use
    // first::say_hellobro();
    // second::say_hellobro();

    // dengan use
    say_hellobro();
    say_hellobro_second();

}

mod module {
    // defaultnya private
    // pub artinya public
    pub struct User {
        pub name: String,
        pub email: String,
        pub age: u8,
    }

    impl User {
        pub fn say_hello(&self, name: &str) {
            println!("Hello {}, my name is {}", name, self.name);
        }
    }
}

#[test]
fn test_module() {
    let user = module::User {
        name: String::from("Dimas Saputro"),
        email: String::from("emailku"),
        age: 20,
    };

    user.say_hello("Bambang");
} 

fn main() {
    println!("Hello, world!");
}

#[test]
fn hello_test() {
    println!("Hello Test!");
}

#[test]
fn test_variable() {
    let name1 = "Dimas";
    let name2 = "Saputro";
    println!("Hello {} {}", name1, name2);
}

#[test]
fn test_mutable() {
    let mut name = "Dimas";
    println!("Hello {}", name);
    
    name = "Eko";
    println!("Hello {}", name);
}

#[test]
fn static_typing() {
    let name = "Dimas Saputro";
    println!("Hello {}", name);

    // name = 10;
    println!("Hello {}", name);
}

#[test]
fn shadowing() {
    let name = "Dimas Saputro";
    println!("Hello {}", name);

    let name = 10;
    println!("Hello {}", name);
}

#[test]
fn comment() {
    // komentar satu baris
    /*
    komentar 
    lebih dari satu baris
    */
    println!("comment"); // ini juga komentar
}

#[test]
fn explicit() {
    let age: i32= 20;
    println!("{}", age);
}

#[test]
fn number() {
    let a = 2.5;
    let b = 10;

    println!("{} {}", a, b);
}

#[test]
fn number_conversion() {
    let a: i8 = 10;
    println!("{}", a);

    let b: i16 = a as i16;
    println!("{}", b);

    // integer overflow
    let d: i64 = 10000000000000000;
    let e: i8 = d as i8;
    println!("{}", e); // output: 0
}

#[test]
fn numeric_operator() {
    let a = 10;
    let b = 10;
    let c = a * b;
    println!("{}", c);
    let d = a + b;
    println!("{}", d);
    let e = a - b;
    println!("{}", e);
    let f = a + b;
    println!("{}", f);
}

#[test]
fn augmented_assignment() {
    let mut a = 10;
    println!("{}", a);
    a += 10;
    println!("{}", a);
    a -= 10;
    println!("{}", a);
    a *= 10;
    println!("{}", a);
    a /= 10;
    println!("{}", a);
    a %= 10;
    println!("{}", a);
}

#[test]
fn boolean() {
    let a = true;
    let b: bool = false;
    println!("{} {}", a, b);
}

#[test]
fn char_type() {
    let a = 'c';

    println!("{}", a);
}

#[test]
fn tuple() {
    /*
    bisa berisi lebih dari 1 tipe data
    length is final, tidak bisa berkurang atau bertambah
    */

    let data: (i32, f64, bool) = (10, 10.5, true);
    println!("{:?}", data); // :? untuk debug information

    println!("{}", data.0);

    let b = data.1;
    println!("{}", b);
}

#[test]
fn destructuring_tuple() {
    let data: (i32, f64, bool) = (10, 10.5, true);
    println!("{:?}", data);

    let (a, _, c) = data;
    println!("{} {}", a, c);
}

#[test]
fn mut_tuple() {
    let mut data: (i32, f64, bool) = (10, 10.5, true);
    println!("{:?}", data);

    data.0 = 9;
    data.1 = 9.5;
    data.2 = false;
    println!("{:?}", data);
}

fn unit() {
    // tuple kosong
    println!("Hello");
}

#[test]
fn test_unit() {
    let result = unit();
    println!("{:?}", result);
}

#[test]
fn array() {
    /* 
    sama seperti tuple tapi tipe datanya sejenis 
    */

    let array: [i32; 5] = [1, 2, 3, 4, 5]; // secara explisit
    // let array = [1, 2, 3, 4, 5]; secara implisit
    println!("{:?}", array);

    println!("{}", array[0]);

    let length = array.len();
    println!("{}", length);
}

#[test]
fn mut_array() {
    let mut array = [1, 2, 3, 4, 5];
    println!("{:?}", array);

    array[0] = 10;
    array[1] = 20;

    println!("{:?}", array);
}

#[test]
fn two_dimensional_array() {
    let array2d = [[1, 2, 3], [4, 5, 6]];
    println!("{:?}", array2d);

    println!("{}", array2d[1][0]);
}

#[test]
fn constant() {
    const MINIMUM: &str = "Hello World!";
    println!("{}", MINIMUM);
}

#[test]
fn variable_scope() {
    let eko = 1;

    {
        // inner scope
        println!("inner eko: {}", eko);
        let kurniawan = 2;
        println!("inner kurniawan: {}", kurniawan);
    }

    // println!("inner kurniawan: {}", kurniawan); //error
}

#[test]
fn stack_heap() {
    function_a();
    function_b();
}

fn function_a() {
    let a = 10; // disimpan di stack
    let b = String::from("Dimas"); // disimpan diheap karena bisa membesar dan mengecil
    println!("{} {}", a, b);
}

fn function_b() {
    let a = 10;
    let b = String::from("Saputro");
    println!("{} {}", a, b);
}

#[test]
fn string_type() {
    let a = "Dimas"; // string slice, disimpan di stack, fix sized
    let b = String::from("Dimas"); // String, disimpan di heap, data bisa mengembang

    let size_of_string_slice = mem::size_of_val(&a);
    let size_of_string = mem::size_of_val(&b);

    /*
    alokasi memori lebih banyak karena menyimpan pointer ke data, panjang, dan kapasitas
    in this case: 24 bytes
    */
    println!("Ukuran memori dari String: {} bytes", size_of_string);

    /* 
    alokasi hanya menyimpan referensi ke data dan panjangnya, jadi lebih kecil
    in this case: 16 bytes
    */
    println!("Ukuran memori dari &str: {} bytes", size_of_string_slice);


    println!("Alamat memori dari String: {:p}", &b); // alamat variabel
    println!("Alamat memory dari data String: {:p}", b.as_ptr()); // alamat pointer
    println!("Alamat memori dari &str: {:p}", &a); // alamat variabel
    println!("Alamat memori dari data &str: {:p}", a.as_ptr()); // alamat pointer
}

#[test]
fn string_slice() {
    let name = "  Dimas Saputro  ";
    let trim = name.trim();

    println!("{} {}", name, trim);
}

#[test]
fn string() {
    let mut name: String = String::from("Dimas Saputro");
    name.push_str(" Khannedy");

    println!("{}", name);

    let budi = name.replace("Dimas", "Budi");
    println!("{}", budi);
}

#[test]
fn ownership_rules() {
    // a tidak bisa diakses disini, belum dideklarasikan
    let a = 10; // a bisa diakses mulai dari sini

    { // b tidak isa diakses disini, belum dideklarasikan
        let b = 20; // b mulai bisa diakses dari sini
        println!("{}", b);
    } // scope b selesai, b dihapus, b tidak bisa diakses

    println!("{}", a);
} // scope a selesai, a dihapus, a tidak bisa diakses  


#[test]
fn data_copy() {
    // cuma terjadi untuk data yang disimpan di stack (fix size)
    let a = 10;
    let b = a; // copy data dari a ke b
    // jadi di stack itu ada a = 10, dan b = 10
    // bukan a = b = 10

    println!("{} {}", a, b);
}

#[test]
fn ownership_movement() {
    let name1 = String::from("Dimas Saputro");

    // ownership dari name1 dipindahkan ke name2
    let name2 = name1;
    // name1 tidak bisa diakses mulai dari sini

    // println!("{}", name1); // error
    println!("{}", name2);
}

#[test]
fn clone() {
    // membuat data tiruan yang sama dengan data aslinya
    // kalau data yang di stack itu di copy, yang di heap itu di clone

    let name1 = String::from("Dimas Saputro");

    let name2 = name1.clone();

    println!("{} {}", name1, name2);
}

#[test]
fn if_expression() {
    // let value = 6;
    // let result = if value >= 8 {
    //     "Good";
    // } else if value >= 6 {
    //     "Not Bad";
    // } else {
    //     "Bad";
    // };

    // println!("{}", result);
}

#[test]
fn loop_expression() {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter > 10 {
            break counter * 2;
        } else if counter % 2 == 0 {
            continue;
        }

        println!("Counter: {}", counter);
    };

    println!("{}", result);
}

#[test]
fn loop_label() {
    let mut number = 1;
    'outer: loop {
        let mut i = 1;
        loop { // loop inner
            if number > 10 {
                break 'outer; // akan menghentikan loop outer
            }

            println!("{} x {} = {}", number, i, number * i);
            i += 1;
            if i > 10 {
                break; // hanya akan mengentkan loop inner
            }
        }

        number += 1;
    }
}

#[test]
fn while_loop() {
    let mut counter = 0;
    while counter <= 10 {
        if counter % 2 == 0 {
            println!("Counter: {}", counter);
        }

        counter += 1;
    }
}

#[test]
fn for_loop() {
    // iterasi array
    let array: [&str; 5] = ["A", "B", "C", "D", "E"];
    
    for value in array {
        println!("Value {}", value);
    }
}

#[test]
fn range_exclusive() {
    // range dimulai dari start, dan berakhir sebelum end
    let range = 0..5;
    println!("Start: {}", range.start);
    println!("End: {}", range.end);

    for i in range {
        println!("Value {}", i);

    }
}


#[test]
fn range_inclusive() {
    // range dimulai dari start, dan berakhir di end
    let range = 0..=5;
    println!("Start: {}", range.start());
    println!("End: {}", range.end());

    for i in range {
        println!("Value {}", i);
    }
}

fn say_hello() {
    println!("Hello");
}

fn say_goodbye(first_name: &str, last_name: &str) {
    println!("Goodbye {} {}", first_name, last_name);
}

#[test]
fn test_say_hello() {
    say_hello();
    say_goodbye("Dimas", "Saputro");
}


fn factorial_loop(n: i32) -> i32 {
    if n < 1 {
        return 0;
    }

    let mut result = 1;
    for i in 1..=n {
        result *= i;
    };

    // return result; // juga bisa begini
    result
}

#[test]
fn test_factorial_loop() {
    let result = factorial_loop(5);
    println!("Result: {}", result);

    let result = factorial_loop(-1);
    println!("Result: {}", result);
}


fn factorial_recursive(n: u32) -> u32 {
    if n == 1 {
        return 1;
    }

    n * factorial_recursive(n-1)
}

#[test]
fn test_recursive() {
    let result = factorial_recursive(5);

    println!("Result: {}", result);
}


fn print_number(number: i32)  {
    println!("number: {}", number);
}

fn hi(name: String) {
    println!("Hi, {}", name);
}

fn full_name(first_name: String, last_name: String) -> String {
    format!("{} {}", first_name, last_name)
}

#[test]
fn ownership_function() {
    let number = 10;
    let name = String::from("Dimas Saputro");
    println!("number: {}", number);
    println!("name: {}", name);

    print_number(number);
    hi(name);

    println!("number: {}", number);
    // println!("name: {}", name); // ownershipnya dipindahkan ke parameter hi

    let first_name = String::from("Dimas");
    let last_name = String::from("Saputro");
    println!("{} {}", first_name, last_name);

    let my_name = full_name(first_name, last_name);
    // println!("{} {}", first_name, last_name); // ownershipnya sudah berpindah
    println!("{}", my_name);
}

fn full_name_2(first_name: String, last_name: String) -> (String, String, String) {
    let full_name = format!("{} {}", first_name, last_name);

    (first_name, last_name, full_name)
}

#[test]
fn mengembalikan_ownership() {
    let first_name = String::from("Dimas");
    let last_name = String::from("Saputro");
    println!("{} {}", first_name, last_name);

    let (first_name, last_name, full_name) = full_name_2(first_name, last_name);
    println!("{}", full_name);
    println!("{}", first_name);
    println!("{}", last_name);
}

fn full_name3(first_name: &String, last_name: &String) -> String {
    format!("{} {}", first_name, last_name)
}

#[test]
fn reference() {
    // proses pembuatan reference dinamakan borrowing
    // menggunakan value, tanpa transfer ownership

    let first_name = String::from("Dimas");
    let last_name = String::from("Saputro");

    let name = full_name3(&first_name, &last_name);
    println!("{}", name);
    println!("{}", first_name);
    println!("{}", last_name);
}


fn change_value(value: &mut String) {
    value.push_str("Hello");
}

#[test]
fn mutable_reference() {
    // defaultnya reference itu imutable

    let mut value = String::from("Dimas");

    let value_borrow = &mut value;    
    // let value_borrow2 = &mut value; // pada satu waktu cuma boleh ada satu reference (jika ada mutable reference)  

    change_value(value_borrow);
    println!("{}", value);
}

// // dangling pointer
// // pointer yang mengarah ke value yang tidak ada di memori
// fn dangling(value: String) -> &String {
//     &value 
// }

// #[test]
// fn dangling_test() {
//     let x = String::from("Dimas");
//     let x_pointer = dangling(x); // ownership x berpindah ke parameter fungsi dangling
//     // setelah dangling selesai, parameter dihapus karena variable scope
//     // x_pointer menunjuk ke value yang tidak ada di alamat memori 

// }

#[test]
fn slice() {
    // slice = reference ke sebagian elemen dari data collection (ex: array)

    let array: [i32; 5] = [0, 1, 2, 3, 4];
    let range_exclusive = 1..3; // 1, 2
    let range_inclusive = 1..=3; // 1, 2, 3
    let range_from = 1..;
    let range_full = ..;
    let range_to_exclusive = ..3; // 0, 1, 2
    let range_to_inclusive = ..=3; // 0, 1, 2, 3

    println!("{:?}", &array[range_exclusive]);
    println!("{:?}", &array[range_inclusive]);
    println!("{:?}", &array[range_from]);
    println!("{:?}", &array[range_full]);
    println!("{:?}", &array[range_to_exclusive]);
    println!("{:?}", &array[range_to_inclusive]);
}


#[test]
fn string_slice2() {
    let name = String::from("Dimas Saputro");
    let first_name = &name[..5];
    println!("{}", first_name);

    let last_name = &name[6..];
    println!("{}", last_name);

}

struct Person {
    first_name: String,
    middle_name: String,
    last_name: String,
    age: u8,
}

fn print_person(person: &Person) {
    println!("{} {}", person.first_name, person.last_name);
}

#[test]
fn test_struct() {
    let person: Person = Person {
        last_name: String::from("Saputro"),
        first_name: String::from("Dimas"),
        middle_name: String::from(" "),
        age: 20,
    };

    print_person(&person);

    println!("{} {}", person.first_name, person.last_name);
}


#[test]
fn init_shorthand() {
    let first_name = String::from("Dimas");
    let last_name = String::from("Saputro");

    let person = Person {
        first_name,
        last_name,
        middle_name: String::from(""),
        age: 20,
    };

    println!("{} {}", person.first_name, person.last_name);
}

#[test]
fn struct_update() {
    let mut person = Person {
        first_name: String::from("Dimas"),
        last_name: String::from("Saputro"),
        middle_name: String::from(" "),
        age: 20,
    };
    print_person(&person);

    let person2 = Person {..person};
    print_person(&person2);
    // print_person(&person); // ownershipnya pindah ke person2

    let person3 = Person {
        first_name: person2.first_name.clone(),
        middle_name: person2.middle_name.clone(),
        last_name: person2.last_name.clone(),
        ..person2
    };
    print_person(&person3);
    print_person(&person2);
}

struct GeoPoint(f64, f64);

#[test]
fn tuple_struct() {
    let geo_point = GeoPoint(-6.9999, 106.9999);
    println!("{} {}", geo_point.0, geo_point.1);
}

struct Nothing; // sama aja seperti unit ()

#[test]
fn struct_tanpa_field() {
    let _nothing1 = Nothing;
    let _nothing2 = Nothing {};
}

impl Person {
    // function yang di dalam impl ini disebut associated functions
    // associated function yang punya parameter self disebut method

    fn say_hello(&self, name: &str) { // reference self agar ownership tidak berpindah
        println!("Hello, {} my name is {}", name, self.first_name);
    }
}

impl GeoPoint {
    fn say_hello() {
        println!("Hello World");
    }
}

impl GeoPoint {
    fn say_hello_2() {
        println!("Hello World 2");
    }
}

#[test]
fn method() {
    let person = Person {
        first_name: String::from("Dimas"),
        middle_name: String::from(" "),
        last_name: String::from("Saputro"),
        age: 20,
    };

    person.say_hello("Eko");
    GeoPoint::say_hello();
    GeoPoint::say_hello_2();
}


enum Level {
    Regular,
    Premium,
    Platinum,
}

enum Payment {
    // card number
    CreditCard(String),

    // bank name, account number
    BankTransfer(String, String),

    // ewaller name, ewallet number
    EWallet(String, String),
}

impl Payment {
    fn pay(&self, amount: u32) {
        println!("Paying amount {}", amount);
    }
}

#[test]
fn test_enum() {
    let _level = Level::Platinum;
    let _payment = Payment::BankTransfer(String::from("BCA"), String::from("123456789"));

    _payment.pay(12000);
    
    // secara default, data enum tidak bisa diakses
}


impl Payment {
    fn checkout(&self, amount: u32) {
        match self {
            Payment::CreditCard(number) => {
                println!("Paying with credit card {} amount {}", number, amount);
            }
            Payment::BankTransfer(bank_name, number ) => {
                println!("Paying with bank transfer {} {} amount {}", bank_name, number, amount);
            }
            Payment::EWallet(name, number ) => {
                println!("Pating with EWallet {} {} amount {}", name, number, amount);
            }
        }
    }
}

#[test]
fn pattern_matching() {
    let level = Level::Platinum;
    match level {
        Level::Regular => {
            println!("Regular");
        }
        Level::Premium => {
            println!("Premium");
        }
        Level::Platinum => {
            println!("Platinum");
        }
    }

    let payment = Payment::BankTransfer(String::from("BCA"), String::from("123456789"));

    payment.checkout(120000);
}

#[test]
fn test_match_value() {
    let name = "Dimas Saputro";

    match name {
        "Dimas" => {
            println!("Hello Dimas");
        }
        "Saputro" | "Dimas Saputro"=> {
            println!("Hello Saputro");
        }
        other => {
            println!("Kenalan dulu {}", other);
        }
    }
}


#[test]
fn test_match_range() {
    let nilai = 50;
    match nilai {
        81..=100 => {
            println!("Good Job")
        }
        51..=80 => {
            println!("Not Bad")
        }
        0..=50 => {
            println!("Bad")
        }
        other => {
            println!("Salah input nilai")
        }
    }
}


#[test]
fn test_match_struct() {
    let point = GeoPoint(1.0, 2.0);
    match point {
        GeoPoint(long, 0.0) => {
            println!("long: {}", long);
        }
        GeoPoint(0.0, lat) => {
            println!("lat: {}", lat)
        }
        GeoPoint(lat, long) => {
            println!("lat: {}, long: {}", lat, long)
        }
    }

    let person = Person {
        first_name: String::from("Dimas"),
        middle_name: String::from(" "),
        last_name: String::from("Saputro"),
        age: 20,
    };

    match person {
        Person {first_name, last_name, .. } => {
            println!("First name: {}, last name: {}", first_name, last_name);
        }
    }
}

#[test] 
fn ignoring() {
    let point = GeoPoint(1.0, 2.0);
    match point {
        GeoPoint(_, long) => {
            println!("Long {}", long);
        }
    }

    let value = 10;

    match value {
        10..= 100 => {
            println!("value: {}", value);
        }
        _ => {
            println!("value not valid");
        }
    }
}

#[test]
fn test_match_expression() {
    let value = 2;
    let result = match value {
        0 => "nol",
        _ => "invalid"
    };

    println!("Result {}", result);
}
 
type Age = u8;
type IdentityNumber = String;

struct Customer {
    id: IdentityNumber,
    name: String,
    age: Age,
}


#[test]
fn type_alias() {
    let customer = Customer {
        id: String::from("12947287"),
        name: String::from("Dimas Saputro"),
        age: 22,
    };

    println!("{}", customer.name);
}
 

trait CanSayHello {
    fn say_hello(&self) -> String;
    fn say_hello_to(&self, name: &str) -> String;

    // default implementation
    fn hello(&self) -> String {
        String::from("hello")
    }
}

impl CanSayHello for Person {
    fn say_hello(&self) -> String {
        format!("Hello, my name is {}", self.first_name)
    }

    fn say_hello_to(&self, name: &str) -> String {
        format!("Hello {}, my name is {}", name, self.first_name)
    }
}

#[test]
fn test_trait() {
    let person = Person {
        first_name: String::from("Dimas"),
        middle_name: String::from(" "),
        last_name: String::from("Saputro"),
        age: 20,
    };

    let message = person.say_hello_to("Fitria");
    println!("{}", message);

    let result = person.hello();
    println!("{}", result);
}

fn say_hello_trait(person: &impl CanSayHello) {
    println!("{}", person.say_hello_to("Fitria"));
}

#[test]
fn trait_as_parameter() {
    let person = Person {
        first_name: String::from("Dimas"),
        middle_name: String::from(" "),
        last_name: String::from("Saputro"),
        age: 20,
    };

    say_hello_trait(&person);
}

trait CanSayGoodBye {
    fn good_bye(&self) -> String;
    fn good_bye_to(&self, name: &str) -> String;
}

impl CanSayGoodBye for Person {
    fn good_bye(&self) -> String {
        format!("Good bye")
    }

    fn good_bye_to(&self, name: &str) -> String {
        format!("Good bye, {}", name)
    }
}

fn hello_and_goodbye(value: &(impl CanSayHello + CanSayGoodBye)) {
    println!("{}", value.say_hello_to("Fitria"));
    println!("{}", value.good_bye_to("Fitria"));
}

#[test]
fn test_multiple_trait() {
    let person = Person {
        first_name: String::from("Dimas"),
        middle_name: String::from(" "),
        last_name: String::from("Saputro"),
        age: 20,
    };

    hello_and_goodbye(&person);
}

struct SimplePerson {
    name: String,
}



impl CanSayGoodBye for SimplePerson {
    fn good_bye(&self) -> String {
        format!("Good bye, my name is {}", self.name)
    }
    
    fn good_bye_to(&self, name: &str) -> String {
        format!("Good bye {}, my name is {}", name, self.name)
    }
}

fn create_person(name: String) -> impl CanSayGoodBye {
    SimplePerson {name}
}

#[test]
fn test_return_trait() {
    let simple_person = create_person(String::from("Dimas Saputro"));
    println!("Hello {}", simple_person.good_bye());
}

#[test]
fn conflict_method() {
    let person = Person {
        last_name: String::from("Dimas"),
        middle_name: String::from(" "),
        first_name: String::from("Saputro"),
        age: 20,
    };

    Person::say_hello(&person, "Budi");
    CanSayHello::say_hello(&person);
}


trait CanSay: CanSayHello + CanSayGoodBye {
    fn say(&self) {
        println!("{}", self.say_hello());
    }
}

struct SimpleMan {
    name: String,
}

impl CanSayHello for SimpleMan {
    fn say_hello(&self) -> String {
        String::from("Hello")
    }

    fn say_hello_to(&self, name: &str) -> String {
        String::from("Hello to")
    }
}

impl CanSayGoodBye for SimpleMan {
    fn good_bye(&self) -> String {
        String::from("Good bye")
    }

    fn good_bye_to(&self, name: &str) -> String {
        String::from("Good bye to")
    }
}

impl CanSay for SimpleMan {}
