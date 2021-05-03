use dialoguer::{theme::ColorfulTheme, Input, Password, Select};

#[derive(Debug)]
pub struct Account {
    pub user: String,
    pub password: String,
}

pub enum LoginStatus {
    Logged,
    NotLogged,
}

pub fn login() -> Result<LoginStatus, reqwest::Error> {
    let account = get_account();
    reqwest::blocking::Client::new()
        .post("http://10.3.8.211/login")
        .form(&[("user", account.user), ("pass", account.password)])
        .send()?;

    Ok(LoginStatus::Logged)
}

pub fn logout() -> Result<LoginStatus, reqwest::Error> {
    reqwest::blocking::get("http://10.3.8.211/logout")?;
    Ok(LoginStatus::Logged)
}

pub fn select_command() {
    let selections = &["login", "logout"];

    let theme = ColorfulTheme::default();
    let selection = Select::with_theme(&theme)
        .with_prompt("Bupt Campus Network")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    match selection {
        0 => login().unwrap(),
        1 => logout().unwrap(),
        _ => LoginStatus::NotLogged,
    };

    println!("{} is done", selections[selection]);
}

pub fn get_account() -> Account {
    let theme = ColorfulTheme::default();
    let user: String = Input::with_theme(&theme)
        .with_prompt("Student ID")
        .validate_with(|input: &String| {
            if input.len() == 10 {
                Ok(())
            } else {
                Err("ID length not 10")
            }
        })
        .interact_text()
        .unwrap();

    let password: String = Password::with_theme(&ColorfulTheme::default())
        .with_prompt("Password")
        .interact()
        .unwrap();

    Account { user, password }
}
