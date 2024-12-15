#![allow(dead_code)]
#![allow(unused)]

//! General:
//! To ignore warning at file scope use !, e.g: #![allow(unused)]
//! A function that doesn't return -> ! (exit, panic!, endless loop)
//! We can have ONE mutable reference to the same data at a time.
//! We can't have dangling references since references must ALWAYS be valid.
//! The notation for printing memory address is: {:p}
//! Use &str (string slice) for viewing. String if you want to mutate it.
//! String literal: fixed size, type: &static str, read-only.

fn main() {
    compound_types();
}

fn small_projects_with_elegant_code() {
    println!(r"https://practice.course.rs/elegant-code-base.html");
}

fn variables() {
    //! To ignore unused variables warning we can do: #[allow(unused_variables)]
    {
        let x: i32 = 5; // without initialization- error on assert
        let _y: i32;     // without _- warning of unused
        assert_eq!(x, 5);
    }
    {
        let mut x: i32 = 1;
        x += 2;
        assert_eq!(x, 3);
    }
    {
        let x: i32 = 5;
        {
            let x = 12;
            assert_eq!(x, 12);
        }
        assert_eq!(x, 5);
    }
    {
        let (mut x, y) = (1, 2);
        x += 2;
        assert_eq!(x, 3);
        assert_eq!(y, 2);
    }
    {
        let (x, y);
        (x, ..) = (3, 4);
        [.., y] = [1, 2];
        assert_eq!([x, y], [3, 2]);
    }
}

fn basic_types() {
    fn numbers() {
        {
            let v: u16 = 38_u8 as u16;
            assert_eq!(v, 38_u16);  // will work with 38 as well
        }
        {
            fn type_of<T>(_: &T) -> String {
                format!("{}", std::any::type_name::<T>())
            }
            let x: u32 = 5;
            assert_eq!("u32".to_string(), type_of(&x));
        }
        {
            assert_eq!(i8::MAX, 127);
            assert_eq!(u8::MAX, 255);
        }
        {
            let v1 = 251_u16 + 8;
            // Check for overflow
            let v2 = i16::checked_add(251, 8).unwrap();
            assert_eq!(v1 as i16, v2);
        }
        {
            let v = 1_024 + 0xff + 0o77 + 0b1111_1111; // 1024 + 255 + 63 + 255
            assert_eq!(v, 1597);
        }
        {
            // assert_eq!(0.1 + 0.2, 0.3); Fails since 0.1 + 0.2 = 0.3000...002
            assert_eq!(0.1_f32 + 0.2_f32, 0.3_f32);
            // assert_eq!(0.1 as f32 + 0.2 as f32, 0.3 as f32); // Optional
        }
        {
            let mut sum: i32 = 0;
            for i in -3..2 { // the endpoint of the range is excluded
                sum += i;
            }
            assert_eq!(sum, -5);
        }
        {
            for c in 'a'..='z' {    // include the endpoint
                print!("{} ", c as u8);
            }
        }
        {
            use std::ops::{Range, RangeInclusive};
            assert_eq!(1..5, Range{ start: 1, end: 5 });
            assert_eq!(1..=5, RangeInclusive::new(1, 5));
        }
        {
            println!("0011 AND 0101 is {:04b}", 0b0011_u32 & 0b0101);
        }
    }

    fn char_bool_and_unit() {
        use std::mem::size_of_val;
        {
            // Char in Rust is 4 bytes for every Unicode scalar value
            let c1: char = 'a';
            assert_eq!(size_of_val(&c1), 4);
            let c2: char = '❤';
            assert_eq!(size_of_val(&c2), 4);
        }
        {
            let f: bool = false;
            let t: bool = true;
            assert_eq!(size_of_val(&t), 1);
            if !f && t {
                println!("Success {}", 18);
            }
        }
        {
            fn explicitly_ret_unit() -> () {
                println!("Success {}", 19);
            }
            let unit: () = ();
            assert_eq!(size_of_val(&unit), 0);
            assert_eq!(unit, explicitly_ret_unit());
        }
    }
    fn statements_and_expressions() {
        {
            let x: u32 = 5_u32;
            let y = {
                let x_squared = x * x;
                let x_cubed = x_squared * x;
                // This expression will be assigned to 'y'
                x_squared + x_cubed + x
            };
            // If we put a semicolon after the expression, the assigned value will be '()'
            assert_eq!(y, (5*5)+(5*5*5)+5);
        }
        {
            let v = {
                let mut x: i32 = 1;
                x += 2;
                x   // without it the return value will be ()
            };
            assert_eq!(v, 3);
        }
    }
    fn functions() {
        {
            assert_eq!(sum(2, 2), 4);
            fn sum(a: i32, b: i32) -> i32 {a + b};
        }
        {
            fn get_option(tp: u8) -> Option<i32> {
                match tp {
                    1 => {
                        todo!();
                    }
                    _ => {
                        Some(23)
                    }
                }
            }
            println!("Success {}", get_option(2).unwrap());
        }
        {
            fn never_return() -> ! {
                // while(true) {}
                panic!("Success");
            }
        }
    }
}

fn ownership_and_borrowing() {
    fn ownership() {
        {
            let x = String::from("hello");
            let y = x.clone();
            // if we do not use clone, y takes ownership
            assert_eq!(x, y);
        }
    }
    fn reference_and_borrowing() {
        {
            let s = String::from("hello");
            let bytes = s.as_bytes();  // as_bytes takes reference.

            // Will not work if we do: let bytes = s.into_bytes(); (takes ownership)
            assert_eq!(String::from("hello"), s);
            assert_eq!(&[104, 101, 108, 108, 111][..], &bytes[..]);
        }
        {
            // Since we use &str the size is known at compile time, and it is fixed.
            let x: (i32, i32, (), &str) = (1, 2, (), "hello");
            let y: (i32, i32, (), &str) = x;
            assert_eq!(x, y);
        }
        {
            // Mutability can be changed when ownership is transferred.
            let s: String = String::from("hello ");
            let mut s1: String = s;
            s1.push_str("world");
            assert_eq!(s1, "hello world".to_string());
        }
        {
            // Box let you allocate directly on the heap
            let x: Box<i32> = Box::new(5);
            let mut y:Box<i32> = Box::new(1);
            *y = 5;
            assert_eq!(*x, *y);
        }
        {
            struct Person {
                name: String,
                age: Box<u8>,
            }
            let person: Person = Person {
                name: String::from("Alice"),
                age: Box::new(20),
            };
            // 'name' is moved out of person, but 'age' is referenced
            let Person { name, ref age } = person;
            assert_eq!(*age, person.age);   // can access person.age (person is the owner)
            assert_eq!(name, String::from("Alice"));    // cannot access person.name anymore
        }
        {
            fn change(some_string: &mut String) {
                some_string.push_str(", world");
            }
            let mut s = String::from("hello");
            change(&mut s);
            assert_eq!(s, String::from("hello, world"));

        }
        {
            let x: i32 = 5;
            let p: &i32 = &x;
            assert_eq!(5, *p);
        }
        {
            let mut s: String = String::from("hello");
            let p: &mut String = &mut s;
            p.push_str(", world");
            assert_eq!(s, String::from("hello, world"));

        }
        {
            fn get_addr(r: &char){
                format!("{:p}", r);
            }
            let c: char = '❤';
            let r1: &char = &c;
            let ref r2 = c;
            assert_eq!(*r1, *r2);
            assert_eq!(get_addr(r1), get_addr(r2));
        }
    }
}

fn compound_types() {
    fn string() {
        {
            let s: String = String::from("hello world");
            let hello = &s[0..5];
            let world = &s[6..11];
            assert_eq!(hello, "hello");
            assert_eq!(world, "world");
            // We can't do: let s: str = "hello" (we can't use str like that)
            // We can do: let s: Box<str> = "hello".into(); & can be used to convert Box<str> to &str.
        }
        {
            let mut s: String = String::from("hello");
            s.push(',');
            s.push_str(" world");
            s += "!";
            assert_eq!(s, String::from("hello, world!"));

        }
        {
            let mut s: String = String::from("hello world");
            s = s.replace("hello", "goodbye");
            assert_eq!(s, String::from("goodbye world"));
            println!("Success {}", 36);
        }
        {
            let s1: String = String::from("hello ");
            let s2: String = String::from("world");
            let s3: String = s1 + &s2;  // Can also do: let s3: String = s1 + s2.as_str();
            // s3 took ownership over s1.
            assert_eq!(s3, String::from("hello world"));

        }
        {
            let byte_escape = "Ru\x73\x74";
            assert_eq!(byte_escape, "Rust");

            let unicode_codepoint = "\u{211D}";
            assert_eq!(unicode_codepoint, "ℝ");

            let character_name = "\"";
            assert_eq!(character_name, "\"");

            let long_string = "One \
                            Two";
            assert_eq!(long_string, "One Two");

            // a raw string a string which has now escapes
            let raw_str = r"http:\";
            assert_eq!(raw_str, "http:\\");

            let quotes = r#" _ "Quote" _ "#;
            assert_eq!(quotes,  " _ \"Quote\" _ ");

            let delimiter = r###" _ A string with "##" _ "###;
            assert_eq!(delimiter, " _ A string with \"##\" _ ");

        }
        {
            for c in "🐶🐶🐶🐶".chars() {
                assert_eq!(c, '🐶');
            }
        }
    }
    fn array() {
        {
            //! Array: fixed sized [T; Length] (size must be known at compile time).
            //! All elements in an array must be of the same type.
            //! Out of bound indexing causes panic

            let arr: [i32; 5] = [1, 2, 3, 4, 5];
            assert_eq!(arr.len(), 5);

            let char_arr: [_; 3] = ['a', 'b', 'c'];
            assert_eq!(std::mem::size_of_val(&char_arr), 12);

            // All elements can be initialized to the same value at once.
            let list: [i32; 100] = [1; 100];
            assert_eq!(list[37], 1);
            assert_eq!(list.len(), 100);

            let names: [String; 2] = [String::from("Asaf"), String::from("Nicole")];

            let name0 = names.get(0);
            if name0.is_some() {
                assert_eq!(name0.unwrap(), "Asaf");
            }

            // Cause panic: let name2 = &names[2];  (Out of bound)
            let name0 = names.get(2);
            if name0.is_none() {
                println!("Out of bound");
            }
            // let name0 = names[0]; cannot move (have to use &names[0]
        }
    }
    fn slice() {
        //! a reference to contiguous of elements in a collection
        //! borrow part of a collection.
        //! can be created from arrays, vectors, Strings and other collections implementing Deref trait.
        //! Slice is composed of pointer and length: both of type u_size. In our case 64bit machine- 8 bytes.

        let mut a = [1, 2, 3, 4, 5];
        let mut sl = &mut a[1..3];
        assert_eq!(sl, &[2, 3]);

        sl[0] = 20;
        assert_eq!(20, a[1]);

        let arr: [char; 3] = ['中', '国', '人'];
        let sl: &[char] = &arr[..];
        assert_eq!(std::mem::size_of_val(&sl), 16);

        let s: String = String::from("中国人");
        let sl: &str = &s[..3]; // 中 takes 3 bytes
        assert_eq!(sl, "中");

        // a ref to a string can implicitly convert into a str slice.
        let mut s = String::from("hello");
        let slice0 = &s[..1];   // implicitly convert into a str slice
        println!("s[0] = {}", slice0);
        s.clear();
        // If we were to put the s.clear() above the println!() we will get an error since s is ""
    }
    fn tuple() {}
    fn struct_type() {}
    fn enum_type() {}

    tuple();
}

fn flow_control() {}

fn pattern_match() {
    fn match_matches_iflet() {}
    fn patterns() {}
}

fn method_and_associated_function() {}

fn generics_and_traits() {
    fn generics() {}
    fn const_generics() {}
    fn traits() {}
    fn trait_object() {}
    fn advanced_traits() {}
}

fn collection_types() {
    fn string() {}
    fn vector() {}
    fn hashmap() {}
}

fn type_conversion() {
    fn as_conversion() {}
    fn from_into() {}
    fn others() {}
}

fn result_and_panic() {
    fn panic_macro() {}
    fn result_and_question_mark() {}
}

fn crate_and_module() {
    fn package_and_crate() {}
    fn module() {}
    fn advanced_use_and_pub() {}
}

fn comments_and_docs() {}

fn formatted_output() {
    fn println_and_format() {}
    fn debug_and_display() {}
    fn formatting() {}
}

fn lifetime() {
    fn basic() {}
    fn static_and_t_static() {}
    fn advanced() {}
}

fn functional_programming() {
    fn closure() {}
    fn iterator() {}
}

fn newtype_and_dst() {}

fn smart_pointers() {
    fn box_pointer() {}
    fn deref() {}
    fn drop() {}
    fn rc_and_arc() {}
    fn cell_and_refcell() {}
}

fn weak_and_circle_reference() {}

fn self_referential() {}

fn threads() {
    fn basic_using() {}
    fn message_passing() {}
    fn sync() {}
    fn atomic() {}
    fn send_and_sync() {}
}

fn global_variables() {}

fn errors() {}

fn unsafe_doing() {
    fn inline_assembly() {}
}

fn macro_todo() {}

fn tests() {
    fn write_tests() {}
    fn benchmark() {}
    fn unit_and_integration() {}
    fn assertions() {}
}

fn async_await() {
    fn async_and_await() {}
    fn future() {}
    fn pin_and_unpin() {}
    fn stream() {}
}

fn standard_library() {
    fn string_library() {}
}

fn fighting_with_compiler() {
    fn borrowing() {}
}
