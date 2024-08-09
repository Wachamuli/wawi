mod binding;
mod styling;
mod widget;

use iced::{
    widget::{button, column, container, row, slider, svg, text},
    Alignment, Command, Element, Length,
};
use iced_layershell::{
    reexport::{Anchor, Layer},
    Application as _,
};

struct ControlCenter {
    is_battery_available: bool,
    on_battery: bool,
    percentage: f64,
    time_to_empty: i64,

    master_volume: u32,

    active_power_profile: binding::hadess::PowerProfile,
}

#[derive(Debug, Clone)]
enum Message {
    UPowerDevice(binding::upower::BatteryInfo),
    Hadess(binding::hadess::PowerProfileInfo),
    // SelectPowerProfile(binding::hadess::PowerProfile),
    SetMasterVolume(u32),
    ToggleProfiles,
}

impl iced_layershell::Application for ControlCenter {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = styling::theme::Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self {
                is_battery_available: true,
                on_battery: true,
                percentage: 100.0,
                time_to_empty: 0,

                master_volume: 100,

                active_power_profile: binding::hadess::PowerProfile::Balanced,
            },
            Command::none(),
        )
    }

    fn namespace(&self) -> String {
        String::from("morpheus")
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        iced::Subscription::batch([
            binding::upower::subscription(0).map(Message::UPowerDevice),
            binding::hadess::subscription(0).map(Message::Hadess),
        ])
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::UPowerDevice(event) => match event {
                binding::upower::BatteryInfo::NotAvailable => {
                    self.is_battery_available = false;
                }
                binding::upower::BatteryInfo::Available {
                    on_battery,
                    percent,
                    time_to_empty,
                } => {
                    self.on_battery = on_battery;
                    self.percentage = percent;
                    self.time_to_empty = time_to_empty;
                }
            },
            Message::Hadess(event) => match event {
                binding::hadess::PowerProfileInfo::Active(profile) => {
                    self.active_power_profile = profile;
                }
            },
            // Message::SelectPowerProfile(_profile) => {
            //     todo!("Maybe use std::process::Command to set the power profile?")
            // }
            Message::ToggleProfiles => todo!(),
            Message::SetMasterVolume(volume) => {
                self.master_volume = volume;
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<Message, Self::Theme> {
        let battery_icon = format!(
            "{}/assets/icons/battery{}-{}.svg",
            env!("CARGO_MANIFEST_DIR"),
            if !self.on_battery { "-charging" } else { "" },
            (self.percentage as i32 + 5) / 10 * 10
        );
        let wifi_icon = format!("{}/assets/icons/wifi-full.svg", env!("CARGO_MANIFEST_DIR"));
        let blue_icon = format!("{}/assets/icons/bluetooth.svg", env!("CARGO_MANIFEST_DIR"));
        let plane_icon = format!("{}/assets/icons/airplane.svg", env!("CARGO_MANIFEST_DIR"));
        let power_icon = format!("{}/assets/icons/power-mode.svg", env!("CARGO_MANIFEST_DIR"));
        let fan_icon = format!("{}/assets/icons/fan.svg", env!("CARGO_MANIFEST_DIR"));
        let volume_icon = format!("{}/assets/icons/volume-up.svg", env!("CARGO_MANIFEST_DIR"));
        let bright_icon = format!("{}/assets/icons/brightness.svg", env!("CARGO_MANIFEST_DIR"));

        // TODO: Network Manager, add power profile dropdown menu, change background to transparent, add degraded performance to power profile
        let icon = |icon_path| svg(svg::Handle::from_path(icon_path)).width(25).height(25);

        let circular_button = |icon_path| {
            button(icon(icon_path))
                .style(styling::style::Button::Circular)
                .padding(17)
        };

        let rectangular_button = |title, subtitle, icon_path, message| {
            button(
                row![
                    icon(icon_path),
                    column![
                        text(title).font(styling::font::SF_PRO_BOLD).size(16),
                        text(subtitle)
                    ]
                ]
                .spacing(20)
                .align_items(Alignment::Center),
            )
            .on_press(message)
            .width(250)
            .padding([10, 20, 10, 20])
        };

        let battery: Element<Message, Self::Theme> = if !self.is_battery_available {
            text("No Battery").into()
        } else {
            row![
                icon(battery_icon),
                column![
                    text(format!("{}%", self.percentage)).font(styling::font::SF_PRO_BOLD),
                    text(styling::format::seconds_to_hour_minute(self.time_to_empty))
                ],
            ]
            .spacing(10)
            .align_items(Alignment::Center)
            .into()
        };

        container(
            column![
                row![
                    container(battery).width(Length::Fill),
                    container(
                        row![
                            circular_button(wifi_icon),
                            circular_button(blue_icon),
                            circular_button(plane_icon),
                        ]
                        .spacing(10)
                    )
                ]
                .align_items(Alignment::Center),
                container(
                    column![
                        row![
                            icon(volume_icon),
                            slider(0..=100, self.master_volume, Message::SetMasterVolume)
                        ]
                        .align_items(Alignment::Center)
                        .spacing(10),
                        row![
                            icon(bright_icon),
                            slider(0..=100, self.master_volume, Message::SetMasterVolume),
                        ]
                        .align_items(Alignment::Center)
                        .spacing(10)
                    ]
                    .spacing(10)
                ),
                container(
                    column![
                        rectangular_button(
                            "Power Mode",
                            &self.active_power_profile.clone().into(),
                            power_icon,
                            Message::ToggleProfiles
                        ),
                        rectangular_button(
                            "Fan Profile",
                            &"Silent".to_string(),
                            fan_icon,
                            Message::ToggleProfiles
                        ),
                    ]
                    .spacing(10)
                )
            ]
            .spacing(20),
        )
        .style(styling::style::Container::HeavyRounded)
        .padding(32)
        .width(iced::Length::Fill)
        .height(iced::Length::Fill)
        .into()
    }
}

fn main() -> Result<(), iced_layershell::Error> {
    ControlCenter::run(iced_layershell::settings::Settings {
        id: Some(String::from("control_center")),
        antialiasing: true,
        default_font: styling::font::SF_PRO,
        layer_settings: iced_layershell::settings::LayerShellSettings {
            layer: Layer::Top,
            anchor: Anchor::Right | Anchor::Top,
            margins: (40 + 15, 10, 0, 0),
            size: Some((475, 375)),
            ..Default::default()
        },
        ..Default::default()
    })
}
