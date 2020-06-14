use std::fmt;

pub struct Foo<'a, 'b> {
    pub ival: i32,
    pub iref: &'a mut i32,
    pub sval: String,
    pub sref: &'b mut String,
}

impl<'a, 'b> Foo<'a, 'b> {
    pub fn new(ival: i32, iref: &'a mut i32, sval: String, sref: &'b mut String) -> Foo<'a, 'b> {
        Foo {
            ival,
            iref,
            sval,
            sref,
        }
    }
}

impl<'a, 'b> fmt::Display for Foo<'a, 'b> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Foo:[ival(i32): {}, iref(&mut i32): {}, sval(String): {}, sref(&mut String): {}]",
            self.ival, self.iref, self.sval, self.sref
        )
    }
}

#[cfg(test)]
mod test {
    use super::Foo;

    #[test]
    fn match_val_by_val() {
        println!("--------match_val_by_val--------");
        let mut i: i32 = 1;
        let mut s: String = "a".to_string();

        let f: Foo = Foo::new(1, &mut i, "a".to_string(), &mut s);
        println!("{}", f);

        match f {
            Foo {
                ival,
                iref,
                sval,
                sref,
            } => {
                //ival(type i32)         : copied from f.ival;
                //iref(type &mut i32)    : moved from f.iref;
                //sval(type String)      : moved from f.sval;
                //sref(type &mut String) : moved from f.sref;

                *iref = *iref * 2;
                *sref = "b".to_string();
                println!("matched: [{} {} {} {}]", ival, *iref, sval, *sref);
            }
        }
        //println!("{}", f); //this would not compile, because f was moved;
    }

    #[test]
    fn match_ref_by_ref() {
        println!("--------match_ref_by_ref--------");
        let mut i: i32 = 8;
        let mut s: String = "hello".to_string();

        let f: Foo = Foo::new(8, &mut i, "hello".to_string(), &mut s);
        let r: &Foo = &f;
        println!("{}", r);

        /*
        match r {
            &Foo {ival, iref, sval, sref} => {
                //ival: copied from r.ival;
                //iref: would be moved from r.iref; but r is a reference, move is not allowed;
                //sval: would be moved from r.sval; not allowed;
                //sref: would be moved from r.sref; not allowed;

                //this would be fine if all members of Foo are copiable. It can be thought this way:
                //rust tries to create a brand new Foo object from *r, this would fail if any memeber
                //of *r is not copiable;

                println!("matched: [{} {} {} {}]", ival, *iref, sval, *sref);
            }
        }
        */

        match r {
            &Foo {
                ival,
                ref iref,
                ref sval,
                ref sref,
            } => {
                //ival(type i32)          : copied from r.ival;
                //iref(type &&mut i32)    : borrowed from r.iref;
                //sval(type &String)      : borrowed from r.sval;
                //sref(type &&mut String) : borrowed from r.sref;

                println!("matched: [{} {} {} {}]", ival, **iref, *sval, **sref);
            }
        }
        println!("{}", f);
        println!("{}", r);
    }

    #[test]
    fn match_ref_by_val() {
        println!("--------match_ref_by_val--------");
        let mut i: i32 = 8;
        let mut s: String = "hello".to_string();

        let f: Foo = Foo::new(8, &mut i, "hello".to_string(), &mut s);
        let r: &Foo = &f;
        println!("{}", r);

        match r {
            Foo {
                ival,
                iref,
                sval,
                sref,
            } => {
                //ival(type &i32)           : borrowed from r.ival;
                //iref(type &&mut i32)      : borrowed from r.iref;
                //sval(type &String)        : borrowed from r.sval;
                //sref(type &&mut String)   : borrowed from r.sref;

                println!("matched: [{} {} {} {}]", *ival, **iref, *sval, **sref);
            }
        }
        println!("{}", f); //f was not moved;
    }

    /*
    #[test]
    fn match_val_by_ref() {
        println!("--------match_val_by_ref--------");
        let mut i: i32 = 8;
        let mut s: String = "hello".to_string();

        let f: Foo = Foo::new(8, &mut i, "hello".to_string(), &mut s);
        println!("{}", f);

        match f {
            //error here: expected struct `Foo`, found reference
            &Foo {
                ival,
                iref,
                sval,
                sref,
            } => {} //not allowed
        }
    }
    */
}
