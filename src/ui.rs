use dialoguer::{theme::ColorfulTheme, Input, Select};

pub fn request_raw_content() -> Result<String, &'static str> {
    match Input::with_theme(&ColorfulTheme::default()).with_prompt("Please input some content with valid markups.")
    .default("トヨタ自動車はあすからロシアにある<<工場・こうじょう>>の<<稼働・かどう・operation of a machine, running>>を<<停止・ていし>>すると<<発表・はっぴょう>>しました。".into())
    .interact_text() {
        Ok(input) => Ok(input),
        Err(_) => Err("Failed to get the input"),
    }
}
