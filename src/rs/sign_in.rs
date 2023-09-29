// ==================== Experimentatin RE signing a user in on kintro
// make everything generic obvs

// use thirtyfour::prelude::*;

// #[doc ="Takes the driver & caps from the actual test"]
// pub async fn sign_in(d: &WebDriver, email: &str, password: &str) -> &WebDriver {
//     // let caps = DesiredCapabilities::chrome();
//     // let d = WebDriver::new("http://localhost:9515", caps).await?;

//     d.goto("https://sandbox-web.kintro.co/sign-in").await?;

//     // find the form
//     let form = d
//         .find(By::XPath(
//             "/html/body/div[1]/div[1]/div/div/div[1]/div[5]/div/form",
//         ))
//         .await?;

//     // find and fill email field
//     let email_field = form.find(By::Id("email")).await?;
//     email_field.send_keys(email).await?;

//     // find and fill password field
//     let password_field = form.find(By::Id("password")).await?;
//     password_field.send_keys(password).await?;

//     let submit_btn = d
//         .find(By::XPath(
//             "//*[@id='root']/div[1]/div/div/div[1]/div[5]/div/form/button",
//         ))
//         .await?;
//     submit_btn.click().await?;

//     // d.quit().await?;

//     println!("User with email `{}` is signed in!", email);

//     // let res = match d {
//     //     Ok(_) => None,
//     //     Err(_) => Some(res),
//     // };

//     // Ok(())

//     // somehow pass the webdriver instance back to the actual test,
//     // presumably this will allow current_url to be the same here as there
//     d
// }
