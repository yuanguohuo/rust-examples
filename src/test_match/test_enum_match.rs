use std::fmt;

pub enum MyOption<T> {
    MyNone,
    MySome(T),
}

impl<T: fmt::Display> fmt::Display for MyOption<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "fmt {}",
            match self {
                MyOption::MyNone => "MyNone".to_string(),
                MyOption::MySome(v) => format!("MySome({})", v),
            }
        )
    }
}

impl<T: fmt::Display> MyOption<T> {
    pub fn dump(&self) {
        match self {
            MyOption::MyNone => println!("dump MyNone"),
            MyOption::MySome(v) => println!("dump MySome({})", v),
        }
    }
}

#[cfg(test)]
mod test {
    use super::MyOption::MyNone;
    use super::MyOption::MySome;
    use crate::test_match::test_struct_match::Foo;

    #[test]
    fn match_val_by_val() {
        let i: i32 = 3;
        let ri = &i;
        let o1 = MySome(ri);
        match o1 {
            MyNone => println!("match MyNone"),
            MySome(v) => println!("match MySome({})", v),
        }

        println!("{}", o1); //o1 is not moved;
        o1.dump(); //o1 is not moved;
        println!("{}", ri); //ri is not moved;

        let mut j: i32 = 4;
        let rj = &mut j;
        let o2 = MySome(rj);
        match o2 {
            MyNone => println!("match MyNone"),
            MySome(v) => println!("match MySome({})", v),
        }
        //println!("{}", o2); //o2 is moved;
        //o2.dump();
        //println!("{}", rj); //rj is moved;

        let mut i: i32 = 1;
        let mut s: String = "a".to_string();
        let foo = Foo::new(i, &mut i, "a".to_string(), &mut s);
        println!("{}", foo);
        let o3 = MySome(foo);
        match o3 {
            MyNone => println!("match MyNone"),
            MySome(v) => println!("match MySome({})", v),
        }
        //println!("{}", o3);  //o3 is moved;
        //println!("{}", foo); //foo is moved;
    }

    #[test]
    fn match_ref_by_ref() {
        let i: i32 = 333;
        let o1 = MySome(i);
        let o1_ref = &o1;
        match o1_ref {
            &MyNone => println!("match MyNone"),
            &MySome(v) => println!("match MySome({})", v),
        }

        println!("{}", o1); //o1 is not moved;
        o1.dump(); //o1 is not moved;

        let mut i: i32 = 1;
        let mut s: String = "a".to_string();
        let o2 = MySome(Foo::new(i, &mut i, "a".to_string(), &mut s));
        let o2_ref = &o2;

        match o2_ref {
            &MyNone => println!("match MyNone"),

            //this would be fine if all members of MySome are Copy. It can be thought this way:
            //rust would create a brand new MySome from o_ref, as a result, all members would be
            //moved (unless it is Copy) from o_ref; but on the other hand o_ref is a reference
            //which cannot be moved. So, this will not compile if any member of o_ref is not Copy;
            //See o1 above, because i32 is Copy, so it is OK;
            //&MySome(v) => println!("match MySome({})", v),
            &MySome(ref v) => println!("match MySome({})", v),
        }
        println!("{}", o2);
        println!("{}", o2_ref);
    }

    #[test]
    fn match_ref_by_val() {
        let mut i: i32 = 1;
        let mut s: String = "a".to_string();

        let o1 = MySome(Foo::new(i, &mut i, "a".to_string(), &mut s));
        let o1_ref = &o1;

        match o1_ref {
            MyNone => println!("match MyNone"),
            MySome(v) => println!("match MySome({})", v),
        }

        println!("{}", o1);
        println!("{}", o1_ref);
    }

    /*
    #[test]
    fn match_val_by_ref() {
        let mut i: i32 = 1;
        let mut s: String = "a".to_string();

        let o1 = MySome(Foo::new(i, &mut i, "a".to_string(), &mut s));

        match o1 {
            &MyNone => println!("match MyNone"),
            &MySome(v) => println!("match MySome({})", v),
        }
    }
    */
}
