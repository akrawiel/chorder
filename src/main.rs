use gdk::ModifierType;
use gdk::{pango::AttrList, Key};
use std::fs;
use std::{collections::HashMap, path::PathBuf};

use dirs;
use gtk::{prelude::*, EventControllerKey};
use serde::{Deserialize, Serialize};

trait WindowDimensions {
    fn get_window_height(&self) -> i32;
    fn get_window_width(&self) -> i32;
}

type OptionsMap = HashMap<String, Vec<HashMap<String, serde_json::Value>>>;

fn default_max_rows() -> i32 {
    3
}
fn default_max_columns() -> i32 {
    4
}
fn default_margin() -> i32 {
    16
}
fn default_spacing() -> i32 {
    16
}
fn default_button_width() -> i32 {
    150
}
fn default_button_height() -> i32 {
    150
}
fn default_shortcut_font() -> String {
    "monospace bold 24".to_owned()
}
fn default_description_font() -> String {
    "monospace 10".to_owned()
}
fn default_options() -> OptionsMap {
    HashMap::new()
}
fn default_shell() -> String {
    "".to_owned()
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Config {
    #[serde(default = "default_max_rows")]
    max_rows: i32,
    #[serde(default = "default_max_columns")]
    max_columns: i32,
    #[serde(default = "default_margin")]
    margin: i32,
    #[serde(default = "default_spacing")]
    spacing: i32,
    #[serde(default = "default_button_width")]
    button_width: i32,
    #[serde(default = "default_button_height")]
    button_height: i32,
    #[serde(default = "default_shell")]
    shell: String,
    #[serde(default = "default_shortcut_font")]
    shortcut_font: String,
    #[serde(default = "default_description_font")]
    description_font: String,
    #[serde(default = "default_options")]
    options: OptionsMap,
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

fn emacsify_modifier(modifier: ModifierType) -> &'static str {
    match modifier {
        ModifierType::ALT_MASK => "a-",
        ModifierType::CONTROL_MASK => "c-",
        ModifierType::SUPER_MASK => "m-",
        ModifierType::SHIFT_MASK => "s-",
        _ => "",
    }
}

fn err_to_string<T: ToString>(err: T) -> String {
    err.to_string()
}

fn on_activate(application: &gtk::Application) -> Result<(), String> {
    let mut config = serde_json::from_str::<Config>("{}").unwrap();

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

    for (key, opts) in &config.options {
        if opts.len() > (config.max_rows as usize) * (config.max_columns as usize) {
            return Err(format!(
                r#"Too many options at key "{}" - should be at most {}, has {}"#,
                key,
                config.max_rows * config.max_columns,
                opts.len()
            ));
        }
    }

    let stringified_config = serde_json::to_string_pretty(&config).map_err(err_to_string)?;

    fs::write(&app_config_file_path, stringified_config).map_err(err_to_string)?;

    let window = gtk::ApplicationWindow::new(application);

    window.set_default_width(config.get_window_width());
    window.set_default_height(config.get_window_height());
    window.set_modal(true);
    window.set_resizable(false);

    let grid = gtk::Grid::builder()
        .column_spacing(config.spacing)
        .row_spacing(config.spacing)
        .margin_top(config.margin)
        .margin_bottom(config.margin)
        .margin_start(config.margin)
        .margin_end(config.margin)
        .build();

    let mut buttons_with_labels = vec![];

    for y in 0..config.max_rows {
        for x in 0..config.max_columns {
            let button = gtk::Button::builder()
                .width_request(config.button_width)
                .height_request(config.button_height)
                .visible(false)
                .focusable(false)
                .build();

            let button_box = gtk::Box::builder()
                .homogeneous(true)
                .orientation(gtk::Orientation::Vertical)
                .build();

            let attr_list = AttrList::new();
            attr_list.insert(gdk::pango::AttrFontDesc::new(
                &gdk::pango::FontDescription::from_string(&config.description_font),
            ));
            attr_list.insert(gdk::pango::AttrInt::new_line_height_absolute(1));

            let dummy_label = gtk::Label::builder()
                .attributes(&attr_list)
                .valign(gtk::Align::Start)
                .label(" ")
                .build();

            let attr_list = AttrList::new();
            attr_list.insert(gdk::pango::AttrFontDesc::new(
                &gdk::pango::FontDescription::from_string(&config.shortcut_font),
            ));
            attr_list.insert(gdk::pango::AttrInt::new_line_height_absolute(1));

            let shortcut_label = gtk::Label::builder()
                .attributes(&attr_list)
                .use_markup(true)
                .valign(gtk::Align::Center)
                .build();

            let attr_list = AttrList::new();
            attr_list.insert(gdk::pango::AttrFontDesc::new(
                &gdk::pango::FontDescription::from_string(&config.description_font),
            ));
            attr_list.insert(gdk::pango::AttrInt::new_line_height_absolute(1));

            let description_label = gtk::Label::builder()
                .attributes(&attr_list)
                .use_markup(true)
                .valign(gtk::Align::End)
                .ellipsize(gdk::pango::EllipsizeMode::End)
                .max_width_chars(1)
                .build();

            button_box.append(&dummy_label);
            button_box.append(&shortcut_label);
            button_box.append(&description_label);

            button.set_child(Some(&button_box));

            grid.attach(&button, x, y, 1, 1);

            buttons_with_labels.push((button, shortcut_label, description_label));
        }
    }

    let action_state = gdk::gio::SimpleAction::new_stateful(
        "state",
        Some(&String::static_variant_type()),
        &"main".to_variant(),
    );

    let options_option = config.options.get("main");

    for index in 0..(config.max_columns * config.max_rows) {
        if let Some((button, _, _)) = buttons_with_labels.get(index as usize) {
            button.set_visible(false);
        }
    }

    if let Some(options) = options_option {
        for (index, option) in options.iter().enumerate() {
            if let Some((button, shortcut_label, description_label)) =
                buttons_with_labels.get(index)
            {
                let raw_shortcut = option.get("shortcut");
                let raw_description = option.get("description");

                if raw_shortcut.is_none() || raw_description.is_none() {
                    continue;
                }

                let shortcut = raw_shortcut
                    .unwrap()
                    .as_str()
                    .unwrap_or_default()
                    .to_owned();
                let description = raw_description
                    .unwrap()
                    .as_str()
                    .unwrap_or_default()
                    .to_owned();

                button.set_visible(true);
                shortcut_label.set_label(&shortcut);
                description_label.set_label(&description);
            }
        }
    }

    let config_clone = config.clone();

    action_state.connect_change_state(move |action, parameter| {
        let parameter = &parameter
            .expect("Could not get parameter")
            .get::<String>()
            .unwrap_or_default();

        action.set_state(&parameter.to_variant());

        let options_option = config_clone.options.get(parameter);

        for (button, _, _) in &buttons_with_labels {
            button.set_visible(false);
        }

        if let Some(options) = options_option {
            for (index, option) in options.iter().enumerate() {
                if let Some((button, shortcut_label, description_label)) =
                    buttons_with_labels.get(index)
                {
                    let raw_shortcut = option.get("shortcut");
                    let raw_description = option.get("description");

                    if raw_shortcut.is_none() || raw_description.is_none() {
                        continue;
                    }

                    let shortcut = raw_shortcut
                        .unwrap()
                        .as_str()
                        .unwrap_or_default()
                        .to_owned();
                    let description = raw_description
                        .unwrap()
                        .as_str()
                        .unwrap_or_default()
                        .to_owned();

                    button.set_visible(true);
                    shortcut_label.set_label(&shortcut);
                    description_label.set_label(&description);
                }
            }
        }
    });
    window.add_action(&action_state);

    let key_controller = EventControllerKey::new();

    key_controller.connect_key_pressed(move |_controller, key, _code, modifier| {
        if key == Key::Escape {
            std::process::exit(0);
        }

        let pressed_key = format!(
            "{}{}",
            [
                emacsify_modifier(modifier & ModifierType::ALT_MASK),
                emacsify_modifier(modifier & ModifierType::CONTROL_MASK),
                emacsify_modifier(modifier & ModifierType::SUPER_MASK),
                emacsify_modifier(modifier & ModifierType::SHIFT_MASK),
            ]
            .join(""),
            key.name().unwrap_or("".into()).to_lowercase()
        );

        let options_option = config
            .options
            .get(&action_state.state().unwrap().get::<String>().unwrap())
            .map(|found_option| {
                found_option.iter().find(|option| {
                    option
                        .get("shortcut")
                        .map_or(false, |shortcut| shortcut.to_owned() == pressed_key)
                })
            });

        if let Some(Some(option)) = options_option {
            let config_clone = config.clone();

            if let Some(raw_switch) = option.get("switch") {
                let switch = raw_switch.as_str().unwrap_or_default().to_owned();
                action_state.change_state(&switch.to_variant());
            }

            if let Some(raw_run) = option.get("run") {
                let run = raw_run.as_str().unwrap_or_default().to_owned();

                let args = option.get("args").map_or(vec![], |value| {
                    value.as_array().map_or(vec![], |array| {
                        array
                            .iter()
                            .map(|value| value.as_str().unwrap_or_default().to_owned())
                            .collect()
                    })
                });

                let _ = std::process::Command::new(run).args(args).spawn();
                std::process::exit(0);
            }

            if let Some(raw_script) = option.get("script") {
                let script = raw_script.as_str().unwrap_or_default().to_owned();
                let path = PathBuf::from(
                    script.replace(
                        "$HOME",
                        dirs::home_dir()
                            .unwrap_or_default()
                            .to_str()
                            .unwrap_or_default(),
                    ),
                );

                let shell = option.get("shell").map_or(config_clone.shell, |value| {
                    value.as_str().unwrap_or_default().to_owned()
                });

                let _ = std::process::Command::new(shell).args([path]).spawn();
                std::process::exit(0);
            }
        }

        return gtk::glib::Propagation::Stop;
    });

    window.set_child(Some(&grid));
    window.present();

    window.add_controller(key_controller);

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
