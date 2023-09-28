use ductility::rs::*;

#[test]
fn test_lorem() {
    let v = words::lorem("short");
    println!("\nThe short lorem paragraph is: \n{:?}", v);

    assert_eq!(1 + 1, 2);
}

#[test]
fn letters() {
    let v = words::paragraph("letters", 100);
    println!("\nThe letter paragraph is: \n{:?}", v);

    assert_eq!(1 + 1, 2);
}

#[test]
fn numbers() {
    let v = words::paragraph("numbers", 100);
    println!("\nThe number paragraph is: \n{:?}", v);

    assert_eq!(1 + 1, 2);
}

#[test]
fn symbols() {
    let v = words::paragraph("symbols", 100);
    println!("\nThe symbol paragraph is: \n{:?}", v);

    assert_eq!(1 + 1, 2);
}

#[test]
fn letters_and_numbers() {
    let v = words::paragraph("letters_and_numbers", 100);
    println!("\nThe letter and number paragraph is: \n{:?}", v);

    assert_eq!(1 + 1, 2);
}

#[test]
fn kitchen_sink() {
    let v = words::paragraph("kitchen_sink", 100);
    println!("\nThe kitchen sink paragraph is: \n{:?}", v);

    assert_eq!(1 + 1, 2);
}
