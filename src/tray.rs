use anyhow::{Context, Result};
use image::ImageFormat;
use log::info;
use tao::{
    event::Event,
    event_loop::{ControlFlow, EventLoopBuilder},
};
use tray_icon::{
    Icon, TrayIconBuilder, TrayIconEvent,
    menu::MenuEvent,
    menu::{Menu, MenuItem},
};

#[derive(Debug)]
enum UserEvent {
    TrayIconEvent(TrayIconEvent),
    MenuEvent(MenuEvent),
}

pub fn start_gui_tray() -> Result<()> {
    let event_loop = EventLoopBuilder::<UserEvent>::with_user_event().build();
    let proxy_tray = event_loop.create_proxy();
    let proxy_menu = event_loop.create_proxy();

    TrayIconEvent::set_event_handler(Some(move |event| {
        proxy_tray.send_event(UserEvent::TrayIconEvent(event)).ok();
    }));
    MenuEvent::set_event_handler(Some(move |event| {
        proxy_menu.send_event(UserEvent::MenuEvent(event)).ok();
    }));

    const ICON_BYTES: &[u8] = include_bytes!("../assets/icon.ico");
    let image = image::load_from_memory_with_format(ICON_BYTES, ImageFormat::Ico)
        .context("Failed to load icon from memory.")?
        .into_rgba8();

    let icon = Icon::from_rgba(image.to_vec(), image.width(), image.height())
        .context("Failed to create icon from RGBA data.")?;

    let tray_menu = Menu::new();
    let quit_item = MenuItem::new("Quit", true, None);
    tray_menu.append(&quit_item).ok();

    let _tray_icon = TrayIconBuilder::new()
        .with_menu(Box::new(tray_menu))
        .with_icon(icon)
        .build()
        .context("Failed to build tray icon.")?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        if let Event::UserEvent(user_event) = event {
            match user_event {
                UserEvent::TrayIconEvent(tray_event) => {
                    info!("\u{25CF} Received tray icon event: {:?}", tray_event);
                }
                UserEvent::MenuEvent(menu_event) => {
                    info!("\u{25CF} Received menu event: {:?}", menu_event);
                    if menu_event.id == quit_item.id() {
                        *control_flow = ControlFlow::Exit;
                    }
                }
            }
        }
    });
}
