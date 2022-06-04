//! main function goes under here
// use std::fmt;

// struct Structure(i32);

// impl fmt::Display for Structure {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{}", self.0)
//     }
// }

// #[derive(Debug)]
// struct MinMax(i64, i64);

// impl fmt::Display for MinMax {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{}, {}", self.0, self.1)
//     }
// }

// #[derive(Debug)]
// struct Point2D {
//     x: f64,
//     y: f64,
// }

// impl fmt::Display for Point2D {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "x: {}, y: {}", self.x, self.y)
//     }
// }

// #[derive(Debug)]
// struct Complex {
//     real: f64,
//     imag: f64,
// }

// impl fmt::Display for Complex {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "real: {}, image: {}", self.real, self.imag)
//     }
// }

// struct List(Vec<i32>);

// impl fmt::Display for List {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         let vec = &self.0;
//         write!(f, "[")?;
//         for (count, v) in vec.iter().enumerate() {
//             if count != 0 {
//                 write!(f, ",  ")?;
//             }
//             write!(f, "{}", v)?;
//         }
//         write!(f, "]")
//     }
// }

use std::fmt::{Display, Formatter, Result};

struct City {
    name: &'static str,
    lat: f32,
    lon: f32,
}

impl Display for City {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

        write!(
            f,
            "{}: {:.3}deg {} {:.3}deg {}",
            self.name,
            self.lat.abs(),
            lat_c,
            self.lon.abs(),
            lon_c
        )
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "Red: {} Green: {} Blue: {}",
            self.red, self.green, self.blue
        )
    }
}

// Main function
fn main() {
    for city in [
        City {
            name: "Dublin",
            lat: 53.347778,
            lon: -6.259722,
        },
        City {
            name: "Oslo",
            lat: 59.95,
            lon: 10.75,
        },
        City {
            name: "Vancouver",
            lat: 49.25,
            lon: -123.1,
        },
    ]
    .iter()
    {
        println!("{}", *city);
    }
    for color in [
        Color {
            red: 128,
            green: 255,
            blue: 90,
        },
        Color {
            red: 0,
            green: 3,
            blue: 254,
        },
        Color {
            red: 0,
            green: 0,
            blue: 0,
        },
    ]
    .iter()
    {
        // println!("{:?}", color);
        // println!("{:#?}", color);
        println!("{}", color);
    }

    // let v = List(vec![1,2,3]);
    // println!("{}", v);

    // let minmax = MinMax(0, 14);
    // println!("Compare structures:");
    // println!("Display: {}", minmax);
    // println!("Debug:  {:?}", minmax);
    // println!("Clean Debug: {:#?}", minmax);
    // let big_range = MinMax(-300, 300);
    // let small_range = MinMax(-3, 3);
    // println!("The big range is {big} and the small range is {small}", big=big_range, small=small_range);

    // let point = Point2D {x: 3.3, y:7.2};
    // println!("Compare points:");
    // println!("Display: {}", point);
    // println!("Debug: {:?}", point);
    // println!("Clean Debug: {:#?}", point);
    // let complex = Complex {real: 27.98, imag: 2.5};
    // println!("Comparing complexes");
    // println!("Display: {}", complex);
    // println!("Debug: {:?}", complex);
    // println!("Clean Debug: {:#?}", complex);

    //   println!("{} days ",  30);

    //   println!("{0}, this is {1}.  {1}, this is {0}","Alice","Bob");

    //   println!("{subject} {verb} {object}",  object="The lazy dog", subject="the quick brown fox",verb="jumps over");

    //   println!("Base 10 repr:                  {}", 69420);
    //   println!("Base 2 (binary) repr:          {:b}", 69420);
    //   println!("Base 8 (octal) repr:           {:o}", 69420);
    //   println!("Base 16 (hexadecimal) repr:    {:x}", 69420);
    //   println!("Base 16 (hexadecimal) repr:    {:X}", 69420);

    //   println!("{number:>5}",  number=5);

    //   println!("{number:0>5}",number=8);

    //   println!("{number:0>width$}", number=6, width=10);

    //   println!("My name is {0},  {1}","John", "Bash.");

    // #[allow(dead_code)]
    // struct Structure(i32);
    // println!("This struct `{}` won't print...", Structure(3));

    //   let number: f64 = 1.0;
    //   let width: usize=6;

    //   println!("{number:0>width$}");

    //   let pi: f64 = 3.141592;
    //   println!("Pi is roughly {pi:0>.size$}", size=3);

    // This struct cannot be printed either with fmt::Display or with fmt::Debug
    // struct UnPrintable(i32);

    // The derive attribute
    // #[deriv(Debug)]
    // struct DebugPrintable(i32);

    // println!("Print Debug Value {}", DebugPrintable(5));

    //   #[derive(Debug)]
    //   struct Structure(i32);

    //   #[derive(Debug)]
    //   struct Deep(Structure);

    //   println!("{:?} months in a year.", 12);
    //   println!("{1:?} {0:?} is the {actor:?} name.", "Slater", "Christian", actor="actor's name");

    //   println!("Now {:?} will print!",Structure(3));

    //   //  Printing a structure that has a debug trait
    //   println!("Now {:?} will print!", Deep(Structure(7)));

    //   #[allow(dead_code)]
    //   #[derive(Debug)]
    //   struct Person<'a> {
    //       name: &'a str,
    //       age: u8
    //   }

    //   let name = "John";
    //   let age = 25;

    //   let user = Person { name, age };

    //   println!("{:#?}", user);
}
