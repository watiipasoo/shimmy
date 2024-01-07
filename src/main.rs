use std::fmt::format;
use webbrowser;

slint::include_modules!();

const TAXPER: f64 = 0.30;
const OWNERPER: f64 = 0.55;
const PROFITPER: f64 = 0.05;
const OPEXPER: f64 = 0.10;

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let ui_handle = ui.as_weak();
    
    ui.on_open_sveltekit(move || {
        let ui = ui_handle.unwrap();
        // let num: f64 = string.trim().parse().unwrap();
        // let tax: f64 = num * TAXPER;
        // let onwer: f64 = num * OWNERPER;
        // let profit: f64 = num * PROFITPER;
        // let opex: f64 = num * OPEXPER;
        // let result: String = format!("Taxes: {:2}\nOwner: {:2} \nProfit {:2} \nOpex {:2}", tax, onwer, profit, opex);

        // ui.set_result(result.into());

        if webbrowser::open("https://kit.svelte.dev/").is_ok() {
            print!("Open");
        }
    });
    
    ui.on_open_github(move||{
        if webbrowser::open("http://github.com").is_ok() {
            print!("Open");
        }
    });

    ui.on_open_youtube(move||{
        if webbrowser::open("https://www.youtube.com/").is_ok(){
            print!("Open")
        }
    });

    ui.on_open_supabase(move||{
        if webbrowser::open("https://supabase.com/").is_ok(){
            print!("Open")
        }
    });

    ui.run()
}
