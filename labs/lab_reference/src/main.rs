fn main() {
    let a = 98;
    let ref_a = &a; // Return value of a too. Not the address of a. (But it can use as a reference).
    let deref_a = *ref_a;
    let poi_a = std::ptr::addr_of!(a);

    let b = String::from("Waho");
    let ref_b = &b;
    // let deref_b = *ref_b; // Cannot move ref_b from where it's pointed, because type of b is String (One of the type that doesn't have the "Copy Trait").
    let poi_b = std::ptr::addr_of!(b);

    println!("A: {a}\nReference to A: {:p}\nPointer to Reference A: {deref_a}", ref_a);
    println!("B: {b}\nReference to B: {:p}\nPointer to Reference B: ", ref_b);
    println!("Pointer A to address: {:p}", poi_a);
    println!("Pointer B to address: {:?}", poi_b);

}
