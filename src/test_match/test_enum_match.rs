use std::fmt;

pub enum MyOption<T> {
    MyNone,
    MySome(T),
}

impl<T> fmt::Display for MyOption<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "MyOption")
    }
}

#[cfg(test)]
mod test {
    use super::MyOption::MyNone;
    use super::MyOption::MySome;
    use crate::test_match::test_struct_match::Foo;

    #[test]
    fn match_val_by_val() {
        let mut i: i32 = 1;
        let mut s: String = "a".to_string();

        let o1 = MySome(Foo::new(i, &mut i, "a".to_string(), &mut s));
        match o1 {
            MyNone => {
                println!("matched MyNone");
            }
            MySome(f) => {
                println!("matched MySome. f={}", f);
            }
        }
        //println!("{}", o1);  //o1 was moved;
    }

    #[test]
    fn match_ref_by_ref() {
        let mut i: i32 = 1;
        let mut s: String = "a".to_string();

        let o1 = MySome(Foo::new(i, &mut i, "a".to_string(), &mut s));
        let o_ref = &o1;

        match o_ref {
            &MyNone => {
                println!("matched MyNone");
            }

            /*
            this would be fine if all members of MySome are copiable. It can be thought this way:
            rust tries to create a brand new MySome from *o_ref, this would fail if any memeber
            of *o_ref is not copiable;
            &MySome(ref f) => {
                println!("matched MySome. f={}", f);
            }
            */
            &MySome(ref f) => {
                println!("matched MySome. f={}", f);
            }
        }
    }

    #[test]
    fn match_ref_by_val() {
        let mut i: i32 = 1;
        let mut s: String = "a".to_string();

        let o1 = MySome(Foo::new(i, &mut i, "a".to_string(), &mut s));
        let o_ref = &o1;

        match o_ref {
            MyNone => {
                println!("matched MyNone");
            }
            MySome(f) => {
                println!("matched MySome. f={}", f);
            }
        }
    }

    /*
    #[test]
    fn match_val_by_ref() {
        let mut i: i32 = 1;
        let mut s: String = "a".to_string();

        let o1 = MySome(Foo::new(i, &mut i, "a".to_string(), &mut s));

        match o1 {
            &MyNone => {
                println!("matched MyNone");
            }
            &MySome(f) => {
                println!("matched MySome. f={}", f);
            }
        }
    }
     */
}
