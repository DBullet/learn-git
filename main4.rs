fn f_scope1(_p: String) {}
fn f_scope2(_p: i32) {}
fn f_scope3(_p: (i32, f32, char)) {}

fn print_string(s: &String) {
	println!("{}", s);
}

fn change_string(s: &mut String) {
// fn change_string(s: &String) { // error: cannot borrow as muttable
	s.push_str("rust");
}

fn main() {
	//move
	let v1 = String::from("hello");
	let v2 = v1;
	// println!("{}", v1); // error: value used here after move

	//clone
	let v3 = v2.clone();
	println!("{} {}", v2, v3); // ok


    let s = String::from("hello world");
    f_scope1(s);
    // println!("{}", s); // error: value used here after move

    let i = 5;
    f_scope2(i);
    println!("{}", i); // ok

    let tuple = (3, 3.14, 'p'); 
    f_scope3(tuple);
    println!("{}", tuple.0); // ok

    let s1 = String::from("hi ");
    print_string(&s1);
    println!("{}", s1);

    // change_string(s1); // error: expected reference
    // change_string(&s1); // error: types differ in mutability

    let mut s2 = String::from("hi ");
    change_string(&mut s2); // ok
    // change_string(&s2); //error: types differ in mutability
    print_string(&s2); // ok
    change_string(&mut s2);

    // cannot borrow `s` as mutable more than once at a time
 	// let r1 = &mut s1;
	// let r2 = &mut s1;

	// ok
	{
    	let r1 = &mut s;
	} // r1 goes out of scope here, so we can make a new reference with no problems.
	let r2 = &mut s;

	let r3 = &s2;
	let r4 = &mut s2;
}
