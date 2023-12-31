use ductility::js::*;

#[test]
fn print_from_js_tests_w_cargo() {
    println!("Cthulhu Fhtagn!!!");
    assert_eq!(1 + 1, 2);
}

// #[test]
// fn lorem() {
//     let v1 = lorem();
//     println!("\nThe short lorem paragraph is: \n{:?}", v1);

//     let v2 = lorem();
//     println!("\nThe medium lorem paragraph is: \n{:?}", v2);

//     let v3 = lorem();
//     println!("\nThe long lorem paragraph is: \n{:?}", v3);

//     assert_eq!(v1.len(), 276);
//     assert_eq!(v2.len(), 601);
//     assert_eq!(v3.len(), 1031);
// }

#[test]
fn email_jenner() {
    let desired_len: u32 = 10;
    let s1 = words::email_jenner(desired_len);

    println!(
        "\n====================> EMAIL JENNER <===================== \n{:?}\n",
        s1
    );
}

#[test]
fn username_jenner() {
    let desired_number_of_names: u32 = 3;
    let s1 = words::username_jenner(desired_number_of_names);

    println!(
        "\n====================> UNAME JENNER <===================== \n{:?}\n",
        s1
    );
}

#[test]
fn password_jenner() {
    let desired_len: u32 = 15;
    let s1 = words::password_jenner(desired_len);

    println!(
        "\n====================> PASSWORD JENNER <===================== \n{:?}\n",
        s1
    );
}

#[test]
fn name_picker() {
    let s1 = names::name_picker();

    println!(
        "\n====================> NAME PICKER <===================== \n{:?}\n",
        s1
    );
}
