use ductility::js::*;

#[test]
fn print_from_js_tests_w_cargo() {
    println!("Cthulhu Fhtagn!!!");
    assert_eq!(1 + 1, 2);
}

#[test]
fn lorem() {
    let v1 = lorem();
    println!("\nThe short lorem paragraph is: \n{:?}", v1);

    let v2 = lorem();
    println!("\nThe medium lorem paragraph is: \n{:?}", v2);

    let v3 = lorem();
    println!("\nThe long lorem paragraph is: \n{:?}", v3);

    assert_eq!(v1.len(), 276);
    assert_eq!(v2.len(), 601);
    assert_eq!(v3.len(), 1031);
}
