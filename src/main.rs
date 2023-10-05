use gdk::Key;
use std::collections::HashMap;
use std::fs;

use dirs;
use gtk::{prelude::*, EventControllerKey};
use serde::{Deserialize, Serialize};

trait WindowDimensions {
    fn get_window_height(&self) -> i32;
    fn get_window_width(&self) -> i32;
}

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    max_rows: i32,
    max_columns: i32,
    margin: i32,
    spacing: i32,
    button_width: i32,
    button_height: i32,
    options: HashMap<String, Vec<HashMap<String, String>>>,
}

impl WindowDimensions for Config {
    fn get_window_width(&self) -> i32 {
        (self.button_width * self.max_columns)
            + (self.margin * 2)
            + (self.max_columns - 1) * self.spacing
    }

    fn get_window_height(&self) -> i32 {
        (self.button_height * self.max_rows)
            + (self.margin * 2)
            + (self.max_rows - 1) * self.spacing
    }
}

fn err_to_string<T: ToString>(err: T) -> String {
    err.to_string()
}

fn on_activate(application: &gtk::Application) -> Result<(), String> {
    let default_config: Config = Config {
        max_rows: 3,
        max_columns: 4,
        margin: 16,
        spacing: 16,
        button_width: 150,
        button_height: 150,
        options: HashMap::new(),
    };

    let mut config = default_config;

    let system_config_dir = dirs::config_dir().expect("Config directory not found");

    let app_config_dir = system_config_dir.join("chorder");
    let app_config_dir_exists = app_config_dir.try_exists().map_err(err_to_string)?;

    if !app_config_dir_exists {
        fs::create_dir(&app_config_dir).map_err(|err| err.to_string())?;
    }

    let app_config_file_path = app_config_dir.join("config.json");
    let app_config_file_exists = app_config_file_path.try_exists().map_err(err_to_string)?;

    if app_config_file_exists {
        let config_file_contents =
            fs::read_to_string(&app_config_file_path).map_err(err_to_string)?;
        config = serde_json::from_str::<Config>(&config_file_contents)
            .map_err(|err| format!("JSON error: {}", err.to_string()))?;
    }

    let stringified_config = serde_json::to_string_pretty(&config).map_err(err_to_string)?;

    fs::write(&app_config_file_path, stringified_config).map_err(err_to_string)?;

    let window = gtk::ApplicationWindow::new(application);

    window.set_default_width(config.get_window_width());
    window.set_default_height(config.get_window_height());
    window.set_resizable(false);

    let grid = gtk::Grid::builder()
        .column_spacing(config.spacing)
        .row_spacing(config.spacing)
        .margin_top(config.margin)
        .margin_bottom(config.margin)
        .margin_start(config.margin)
        .margin_end(config.margin)
        .build();

    let mut buttons = vec![];

    for y in 0..config.max_rows {
        for x in 0..config.max_columns {
            let button = gtk::Button::builder()
                .label("Hello there")
                .width_request(config.button_width)
                .height_request(config.button_height)
                .visible(false)
                .build();

            button.connect_clicked(|button| {
                button.set_label("What's up?");
            });

            grid.attach(&button, x, y, 1, 1);

            buttons.push(button);
        }
    }

    let state = "main";

    let options_option = config.options.get(state);

    if let Some(options) = options_option {
        for (index, option) in options.iter().enumerate() {
            println!("{} -> {:?}", index, option);

            let shortcut = option.get("shortcut").ok_or("No shortcut provided".to_string())?;

            if let Some(button) = buttons.get(index) {
                button.set_label(shortcut);
                button.set_visible(true);
            }
        }
    }

    // clone!(@weak window => move |_| window.close())

    let key_controller = EventControllerKey::new();

    key_controller.connect_key_pressed(|_controller, key, _code, _modifier| {
        println!("{}", key.name().unwrap_or("".into()));
        match key {
            Key::a => println!("You pressed 'a'!"),
            _ => (),
        }

        return gtk::glib::Propagation::Stop;
    });
    grid.add_controller(key_controller);

    window.set_child(Some(&grid));
    window.present();

    return Ok(());
}

fn on_activate_with_error_handling(application: &gtk::Application) {
    let activate_result = on_activate(&application);

    if let Err(error) = activate_result {
        println!("{}", error);
        application.quit();
    }
}

fn main() {
    let app = gtk::Application::builder()
        .application_id("dev.kodespresso.chorder")
        .build();
    app.connect_activate(on_activate_with_error_handling);
    app.run();
}
