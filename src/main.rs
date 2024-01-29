slint::include_modules!();
use lazy_static::lazy_static;
use std::sync::RwLock;

lazy_static! {
    static ref R: RwLock<f64> = RwLock::new(0.0);
}

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let ui_handle = ui.as_weak();

    ui.on_change_to_twd(move |number, currency| {
        if let Ok(num) = number.trim().parse::<f64>() {
            let _ = get_rate(&currency);
            let read_lock = R.read().unwrap();
            let output = num * *read_lock;
            let result = format!("大約是台幣: {:.2}\n", output);
            ui_handle.unwrap().set_results(result.into());
        }
    });
    ui.run()
}

#[tokio::main]
async fn get_rate(currency: &str) -> Result<(), reqwest::Error> {
    let url = "https://rate.bot.com.tw/xrt/flcsv/0/day";
    let response = reqwest::get(url).await?;
    let content = response.text().await?;
    let lines: Vec<&str> = content.lines().collect();

    let mapped_values: Vec<(String, String)> = lines
        .iter()
        .filter_map(|line| {
            let items: Vec<&str> = line.split(',').collect();
            if let (Some(first), Some(thirteenth)) = (items.get(0), items.get(12)) {
                Some((first.to_string(), thirteenth.to_string()))
            } else {
                None
            }
        })
        .collect();

    // mapped_values.iter().for_each(|(key, value)| {
    //     println!("Key: {}, Value: {}", key, value);
    // });

    if let Some(usd_value) = mapped_values.iter().find(|(key, _)| key == currency) {
        if let Ok(parsed_value) = usd_value.1.parse::<f64>() {
            let mut write_lock = R.write().unwrap();
            *write_lock = parsed_value;
        } else {
            println!("Failed to parse USD value as f64");
        }
    } else {
        println!("USD not found");
    }

    Ok(())
}
