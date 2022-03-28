static _Y: u64 = 10;

#[allow(unused_variables, unused_assignments, dead_code)]
fn main() {
    //* print
    // println!("hello {0} {0} {1}", 1, 2);
    // println!("hello {num2} {num1}", num1 = 1, num2 = 2);
    // println!("Binary: {:b}, Hex: {:x}, Octal:{:o}", 10, 10, 10); // placeholder traits
    // println!("{:.4}", 1f32 / 3f32);
    // println!("{:?}", "hi");
    // println!("{:?}", [1, 2, 3]);

    //* variables
    // let mut x = 1;
    // x += _Y;
    // x += _Y;
    // println!("{}", x);
    // let (a, b): (u64, f64) = (1, 1.0);

    //* types
    // println!("{}", std::u128::MIN);

    // let b = 1 == 2;
    // let i = true as i32;
    // println!("{} {}", b, i);

    // let ch = 'a';
    // let face = '\u{1F600}';
    // println!("{} {}", ch, face);

    // type MyType = (i32, i64, f32);
    // let x: MyType = (1, 2, 3f32);
    // println!("{:?}", x);

    //* strings
    // let s = "hello world";
    // let x = 1;
    // let slice = &s[x..3];
    // let r = s.replace("world", "again");
    // let has_h = s.contains('h');
    // println!("{:?}", (s, slice, r, has_h));

    //* tuples
    // let person: (&str, &str, i8) = ("Joe", "Mama", 69);
    // println!("{} {} is {}", person.0, person.1, person.2);

    //* arrays
    // let mut numbers = [1i8, 2, 3, 4, 5];
    // numbers[1] = 20;
    // let slice = &numbers[2..4];
    // println!("{:?}", (numbers, slice, std::mem::size_of_val(&numbers)));

    //* vectors
    // let mut v = vec![1; 3];
    // v.push(4);
    // v.push(5);
    // v.push(6);
    // v.pop();
    // println!("{:?}", v);

    //* if
    // if 1 == 1 {
    //     println!("hi");
    // } else if 1 == 2 {
    //     println!("hii");
    // } else {
    //     println!("hiii");
    // }

    // let num = if 1 == 1 { 1 } else { 10 };
    // println!("{}", num);

    //* loops
    // let mut count = 0;
    // loop {
    //     count += 1;
    //     println!("{}", count);

    //     if count > 1000 {
    //         break;
    //     }
    // }

    // let mut count = 0;
    // while count <= 100 {
    //     if count % 15 == 0 {
    //         println!("fizzbuzz")
    //     }
    //     count += 1;
    // }
    // for count in 0..100 {
    //     if count % 15 == 0 {
    //         println!("fizzbuzz")
    //     }
    // }

    //* functions
    // fn add(num1: i32, num2: i32) -> i32 {
    //     num1 + num2
    // }
    // println!("{}", add(10, 1));

    // let x = 9;
    // let add_x = |n1: i32| n1 + x;
    // println!("{}", add_x(5));

    // let mut y = 0;
    // let mut mutate_y = |n: i32| y += n;
    // mutate_y(4);
    // println!("{}", y);

    //* references
    // fn add(s1: &String, s2: &String) -> String {
    //     s1.clone() + s2
    // }
    // let x = String::from("hello");
    // let y = " world".to_string();
    // let result = add(&x, &y);
    // println!("{} + {} = {}", x, y, result);

    // let mut x = 0;
    // let y = &mut x;
    // *y += 3;
    // x += 1;
    // println!("{}", x);

    //* struct
    // struct Color {
    //     red: u8,
    //     green: u8,
    //     blue: u8,
    // }
    // let mut c = Color {
    //     red: 0,
    //     green: 100,
    //     blue: 0,
    // };
    // c.blue = 25;
    // println!("{} {} {}", c.red, c.green, c.blue);

    // struct ColorTuple(u8, u8, u8);
    // let mut cc = ColorTuple(0, 0, 255);
    // cc.0 = 10;
    // println!("{} {} {}", cc.0, cc.1, cc.2);

    //* enums
    // enum Direction {
    //     Up(u32),
    //     Down(u32),
    //     Left,
    //     Right,
    // }
    // fn m(d: Direction) {
    //     match d {
    //         Direction::Up(x) => println!("up: {}", x),
    //         Direction::Down(x) => println!("down: {}", x),
    //         Direction::Left => println!("left"),
    //         Direction::Right => println!("right"),
    //     };
    // }

    // let d = Direction::Up(123);
    // m(d);

    //* results
    // let mut input = String::new();
    // let r = std::io::stdin().read_line(&mut input);
    // // .expect("couldnt read input");

    // match r {
    //     Err(_) => {
    //         println!("couldnt read input");
    //         std::process::exit(1);
    //     }
    //     _ => {}
    // }

    // let res = input.trim().parse::<i32>();
    // // let num = res.unwrap();
    // // println!("{}", num + 1);

    // match res {
    //     Ok(num) => println!("{}", num + 1),
    //     Err(_) => println!("cope harder"),
    // }

    // option
    // let o: Option<i32> = Some(123);
    // // match o {
    // //     Some(n) => println!("{}", n),
    // //     None => println!("no value"),
    // // }
    // if let Some(n) = o {
    //     println!("{}", n);
    // } else {
    //     println!("no value");
    // }

    // cli arguments
    // let args: Vec<_> = std::env::args().skip(1).collect();
    // println!("{:?}", args);

    // oop (nincs amugy xd)
    #[derive(Debug)]
    struct Person {
        first_name: String,
        last_name: String,
        age: u8,
    }
    impl Person {
        fn new(first: &str, last: &str, age: u8) -> Self {
            Self {
                first_name: first.to_string(),
                last_name: last.to_string(),
                age,
            }
        }

        fn full_name(&self) -> String {
            // self.first_name.clone() + " " + &self.last_name
            format!("{} {}", self.first_name, self.last_name)
        }

        fn birthday(&mut self) {
            self.age += 1;
        }
    }

    trait CanTalk {
        fn talk(&self) -> String;
    }
    impl CanTalk for Person {
        fn talk(&self) -> String {
            format!("Hi, my name is {}", self.full_name())
        }
    }
    struct Dog {}
    impl CanTalk for Dog {
        fn talk(&self) -> String {
            "woof!".into()
        }
    }
    // fn print_talk(x: &(impl CanTalk)) {
    //     println!("{}", x.talk());
    // }
    // fn print_talk<T: CanTalk>(x: &T) {
    //     println!("{}", x.talk());
    // }
    fn print_talk<T>(x: &T)
    where
        T: CanTalk,
    {
        println!("{}", x.talk());
    }

    fn get_talker() -> Box<dyn CanTalk> {
        Box::new(Dog {})
    }

    let mut p = Person::new("Joe", "Nuts", 69);
    println!("{}", p.full_name());
    p.birthday();
    println!("{}", p.age);
    // println!("{}", p.talk());
    print_talk(&p);

    let dog = Dog {};
    // println!("{}", dog.talk());
    print_talk(&dog);

    let t = get_talker();
    t.talk();

    println!("{:?}", &p);
}
