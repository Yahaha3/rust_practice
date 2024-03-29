use core::fmt;
use std::fmt::Display;

#[derive(Debug)]
struct Structure(i32);

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

#[derive(Debug)]
struct MinMax(i64, i64);

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.imag > 0.0 {
            write!(f, "{}+{}i", self.real, self.imag)
        } else {
            write!(f, "{}{}i", self.real, self.imag)
        }
    }
}
#[derive(Debug)]
struct List(Vec<i32>);
impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let vec = &self.0;
        write!(f, "[")?;

        for (count, v) in vec.iter().enumerate() {
            if count != 0 {write!(f, ", ")?;}
            write!(f, "{}: {}", count, v)?;
        }

        write!(f, "]")
    }
}

struct City {
    name: &'static str,
    lat: f32,
    lon: f32,
}

impl std::fmt::Display for City {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let lat_c = if self.lat > 0.0 {'N'} else {'S'};
        let lon_c = if self.lon > 0.0 {'E'} else {'W'};

        write!(f, "{}: {:.3}°{} {:.3}°{}",
               self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        
        write!(f, "RGB({}, {}, {}) {}", self.red, self.green, self.blue, format!("0x{:X}{:X}{:X}", self.red, self.green, self.blue))
    }
}

#[cfg(test)]
mod test{

    use crate::example_debug::{City, Color, Complex, Deep, List, MinMax, Person, Point2D, Structure};

    
    #[test]
    fn test_debug() {
        println!("#############################【Debug】#############################");
        println!("{:?}, months in years.", 12);
        println!("{1:?} {0:?} is the {actor:?} name.",
                "Slater",
                "Christian",
                actor="actor`s");
        println!("Now {:?} will print", Structure(3));
        println!("Now {:?} will print", Deep(Structure(3)));

        let name = "Alan";
        let age = 27;
        let alan = Person {name, age};
        println!("{:#?}", alan);
        println!("#############################【 Over 】#############################");
    }

    #[test]
    fn test_display() {
        println!("#############################【Display】#############################");
        let brange = MinMax(-300, 300);
        let srange = MinMax(-3, 3);
        println!("The big range is {big}, and the small is {small}", 
                big = brange, small = srange);
        
        let point = Point2D {x: 3.3, y: 7.7};
        println!("Compare points:");
        println!("Display: {}", point);
        println!("Debug: {:?}", point);

        let complex = Complex{real: 3.3, imag: -7.2};
        println!("Display: {}", complex);
        println!("Debug: {:?}", complex);
        println!("#############################【 Over 】#############################");
    }

    #[test]
    fn test_list() {
        println!("#############################【 List 】#############################");

        let v = List(vec![1,2,3]);
        println!("{}", v);

        println!("#############################【 Over 】#############################");
    }

    #[test]
    fn test_format() {
        println!("#############################【 Format 】#############################");

        for city in [
            City { name: "Dublin", lat: 53.347778, lon: -6.259722 },
            City { name: "Oslo", lat: 59.95, lon: 10.75 },
            City { name: "Vancouver", lat: 49.25, lon: -123.1 },
        ].iter() {
            println!("{}", *city);
        }

        for color in [
            Color { red: 128, green: 255, blue: 90 },
            Color { red: 0, green: 3, blue: 254 },
            Color { red: 0, green: 0, blue: 0 },
        ].iter() {
            // 在添加了针对 fmt::Display 的实现后，请改用 {} 检验效果。
            println!("{}", *color)
        }
        println!("#############################【 Over 】#############################");
    }
}