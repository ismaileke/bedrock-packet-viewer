#![windows_subsystem = "windows"]


// bedrock tool olabilir içinde packet viewer skin catcher pack catcher world catcher vs. gibi şeyler olabilir





use iced::{
    widget::{button, column, container, row, text, Image, scrollable, text_input, Column},
    Application, Command, Element, Length, Settings, Theme, Background, Color, Vector, Subscription,
};
use iced::widget::canvas::{self, Cache, Canvas, Fill, Geometry, LineCap, Path, Program, Stroke, Style};
use iced::mouse::Cursor;
use bedrock_client::client;
use std::sync::{Arc, Mutex};
use iced::window::icon;

// Embed logo file into the project
const LOGO_BYTES: &[u8] = include_bytes!("assets/logo.png");
const PACKETS_ICON: &[u8] = include_bytes!("assets/packets.png");
const STATS_ICON: &[u8] = include_bytes!("assets/stats.png");
const SETTINGS_ICON: &[u8] = include_bytes!("assets/settings.png");

struct DarkContainer {
    theme: ThemeType,
}

impl container::StyleSheet for DarkContainer {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(Color::from_rgb(
                if self.theme == ThemeType::Dark { 0.08 } else { 0.95 },
                if self.theme == ThemeType::Dark { 0.11 } else { 0.96 },
                if self.theme == ThemeType::Dark { 0.16 } else { 0.98 },
            ))),
            text_color: Some(if self.theme == ThemeType::Dark { 
                Color::WHITE 
            } else { 
                Color::from_rgb(0.2, 0.2, 0.2)
            }),
            border_radius: 0.0.into(),
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
        }
    }
}

struct SidebarContainer {
    theme: ThemeType,
}

impl container::StyleSheet for SidebarContainer {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(Color::from_rgb(
                if self.theme == ThemeType::Dark { 0.12 } else { 0.90 },
                if self.theme == ThemeType::Dark { 0.15 } else { 0.92 },
                if self.theme == ThemeType::Dark { 0.22 } else { 0.95 },
            ))),
            text_color: Some(if self.theme == ThemeType::Dark { 
                Color::WHITE 
            } else { 
                Color::from_rgb(0.2, 0.2, 0.2)
            }),
            border_radius: 15.0.into(),
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
        }
    }
}

struct MainContainer {
    theme: ThemeType,
}

impl container::StyleSheet for MainContainer {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(Color::from_rgb(
                if self.theme == ThemeType::Dark { 0.13 } else { 0.92 },
                if self.theme == ThemeType::Dark { 0.16 } else { 0.94 },
                if self.theme == ThemeType::Dark { 0.23 } else { 0.97 },
            ))),
            text_color: Some(if self.theme == ThemeType::Dark { 
                Color::WHITE 
            } else { 
                Color::from_rgb(0.2, 0.2, 0.2)
            }),
            border_radius: 15.0.into(),
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
        }
    }
}

struct ButtonStyle {
    is_selected: bool,
    theme: ThemeType,
    disabled: bool, // Yeni alan ekledik
}

impl Default for ButtonStyle {
    fn default() -> Self {
        Self {
            is_selected: false,
            theme: ThemeType::Dark,
            disabled: false,
        }
    }
}

impl button::StyleSheet for ButtonStyle {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        let color = if self.disabled {
            // Disabled durumunda yarı saydam arkaplan
            if self.theme == ThemeType::Dark {
                Color::from_rgba(0.12, 0.15, 0.20, 0.5)
            } else {
                Color::from_rgba(0.80, 0.82, 0.85, 0.5)
            }
        } else if self.is_selected {
            // Normal selected durumu
            if self.theme == ThemeType::Dark {
                Color::from_rgb(0.18, 0.21, 0.28)
            } else {
                Color::from_rgb(0.88, 0.90, 0.95)
            }
        } else {
            // Normal durum
            if self.theme == ThemeType::Dark {
                Color::from_rgb(0.15, 0.18, 0.25)
            } else {
                Color::from_rgb(0.85, 0.88, 0.92)
            }
        };

        let border_color = if self.disabled {
            // Disabled durumunda yarı saydam border
            if self.theme == ThemeType::Dark {
                Color::from_rgba(0.3, 0.3, 0.3, 0.5)
            } else {
                Color::from_rgba(0.7, 0.7, 0.7, 0.5)
            }
        } else {
            // Normal durumda saydam border
            Color::TRANSPARENT
        };

        button::Appearance {
            background: Some(Background::Color(color)),
            border_radius: 12.0.into(),
            shadow_offset: Vector::new(0.0, if self.is_selected { 1.0 } else { 2.0 }),
            text_color: if self.disabled {
                // Disabled durumunda daha soluk text rengi
                if self.theme == ThemeType::Dark {
                    Color::from_rgb(0.5, 0.5, 0.5)
                } else {
                    Color::from_rgb(0.7, 0.7, 0.7)
                }
            } else if self.theme == ThemeType::Dark {
                Color::WHITE
            } else {
                Color::from_rgb(0.2, 0.2, 0.2)
            },
            border_width: if self.disabled { 1.0 } else { 0.0 },
            border_color,
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        let mut active = self.active(style);
        active.background = Some(Background::Color(if self.is_selected {
            if self.theme == ThemeType::Dark {
                Color::from_rgb(0.20, 0.23, 0.30)
            } else {
                Color::from_rgb(0.90, 0.92, 0.97)
            }
        } else if self.theme == ThemeType::Dark {
            Color::from_rgb(0.17, 0.20, 0.27)
        } else {
            Color::from_rgb(0.82, 0.85, 0.90)
        }));
        active.shadow_offset = Vector::new(0.0, if self.is_selected { 2.0 } else { 4.0 });
        active
    }
}

struct DetailContainer {
    theme: ThemeType,
}

impl container::StyleSheet for DetailContainer {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(Color::from_rgb(
                if self.theme == ThemeType::Dark { 0.15 } else { 0.88 },
                if self.theme == ThemeType::Dark { 0.18 } else { 0.88 },
                if self.theme == ThemeType::Dark { 0.25 } else { 0.90 },
            ))),
            text_color: Some(if self.theme == ThemeType::Dark { 
                Color::WHITE 
            } else { 
                Color::from_rgb(0.2, 0.2, 0.2)
            }),
            border_radius: 8.0.into(),
            border_width: 0.0,  // Border'ı kaldır
            border_color: Color::TRANSPARENT, // Saydam border
        }
    }
}

// Logo container'ı için struct
struct LogoContainer {
    theme: ThemeType,
}

impl container::StyleSheet for LogoContainer {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(if self.theme == ThemeType::Dark {
                Color::from_rgb(0.15, 0.18, 0.25)  // Biraz daha koyu
            } else {
                Color::from_rgb(0.9, 0.92, 0.95)   // Biraz daha açık
            })),
            text_color: Some(Color::WHITE),
            border_radius: 12.0.into(),  // Daha az yuvarlak
            border_width: 0.0,           // Kenarlık yok
            border_color: Color::TRANSPARENT,
        }
    }
}

// Menü tiplerini tanımlayalım
#[derive(Debug, Clone, PartialEq)]
enum MenuType {
    Home,
    Packets,
    Statistics,
    Settings,
}

// Tema tiplerini tanımlayalım
#[derive(Debug, Clone, PartialEq)]
enum ThemeType {
    Dark,
    Light,
}

// Kullanılmayan MenuContainer'ı kaldıralım
// Kullanılmayan State enum'ını kaldıralım
// Kullanılmayan ConsoleContainer'ı kaldıralım
// Kullanılmayan ScrollableStyle'ı kaldıralım

#[derive(Debug, Clone)]
struct Packet {
    name: String,
    packet_type: String,
    content: String,
    timestamp: String,
    size: usize,
}

#[derive(Debug, Clone)]
struct PacketStats {
    unique_types: std::collections::HashMap<String, usize>,
    total_bytes: usize,
}

impl Default for PacketStats {
    fn default() -> Self {
        Self {
            unique_types: std::collections::HashMap::new(),
            total_bytes: 0,
        }
    }
}

// Yeni bir struct ekleyelim runtime için
#[derive(Clone)]
struct Runtime {
    tx: tokio::sync::mpsc::UnboundedSender<Message>,
    rx: std::sync::Arc<tokio::sync::Mutex<tokio::sync::mpsc::UnboundedReceiver<Message>>>,
}

impl Runtime {
    fn new() -> Self {
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        Self { 
            tx, 
            rx: std::sync::Arc::new(tokio::sync::Mutex::new(rx))
        }
    }

    fn spawn<F>(&self, future: F) 
    where
        F: std::future::Future<Output = ()> + Send + 'static,
    {
        tokio::spawn(future);
    }
}

// PacketViewer struct'ına runtime ekleyelim
struct PacketViewer {
    packets: Vec<Packet>,
    selected_packet: Option<usize>,
    detail_height: f32,
    active_menu: MenuType,
    theme: ThemeType,
    server_ip: String,
    server_port: String,
    rgb_offset: f32,
    connection_logs: Arc<Mutex<Vec<String>>>,
    packet_stats: PacketStats,
    connected: bool,
    tx: Option<tokio::sync::mpsc::UnboundedSender<Message>>,
    runtime: Option<Runtime>,
    capturing: bool,  // Yeni alan
    filter_text: String,  // Yeni alan ekleyelim
}

#[derive(Debug, Clone)]
enum Message {
    PacketSelected(usize),
    MenuSelected(MenuType),
    ThemeChanged(ThemeType),
    ExitApp,
    LogoClicked,
    ServerIpChanged(String),
    ServerPortChanged(String),
    Connect,
    PacketReceived(String, String, usize),
    ConnectionError(String),
    OpenGithub,
    OpenAuthUrl,
    AuthCodeChanged(()),
    ClientConnected,
    AnimateDetails(bool),
    UpdatePackets,  // Yeni mesaj
    ToggleCapture,  // Yeni mesaj
    FilterChanged(String),
}

// Application trait implementasyonu
impl Application for PacketViewer {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let runtime = Runtime::new();
        let tx = runtime.tx.clone();
        (
            PacketViewer {
                packets: Vec::new(), // Start with empty packets list
                selected_packet: None,
                detail_height: 0.0,
                active_menu: MenuType::Home,
                theme: ThemeType::Dark,
                server_ip: String::from("127.0.0.1"),
                server_port: String::from("19132"),
                rgb_offset: 0.0,
                connection_logs: Arc::new(Mutex::new(Vec::new())),
                packet_stats: PacketStats::default(),
                connected: false,
                tx: Some(tx),
                runtime: Some(runtime),
                capturing: true,  // Varsayılan olarak açık
                filter_text: String::new(),  // Initialize filter
            },
            Command::none()
        )
    }

    // Window title
    fn title(&self) -> String {
        String::from("Bedrock Packet Viewer")
    }

    fn subscription(&self) -> Subscription<Message> {
        struct EventStream;
        
        if let Some(_runtime) = &self.runtime {
            let rx = _runtime.rx.clone();
            
            iced::subscription::unfold(
                std::any::TypeId::of::<EventStream>(),
                rx,
                move |rx| async move {
                    let msg = {
                        let mut guard = rx.lock().await;
                        match guard.recv().await {
                            Some(message) => (message, rx.clone()),
                            None => panic!("Channel closed unexpectedly")
                        }
                    };
                    msg
                }
            )
        } else {
            Subscription::none()
        }
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::PacketSelected(index) => {
                if Some(index) == self.selected_packet {
                    self.selected_packet = None;
                    return Command::perform(async {
                        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
                    }, |_| Message::AnimateDetails(false));
                } else {
                    self.selected_packet = Some(index);
                    self.detail_height = 0.0;
                    return Command::perform(async {
                        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
                    }, |_| Message::AnimateDetails(true));
                }
            }
            Message::AnimateDetails(opening) => {
                if opening {
                    if self.detail_height < 150.0 {
                        self.detail_height += 25.0;
                        Command::perform(async {
                            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
                        }, |_| Message::AnimateDetails(true))
                    } else {
                        Command::none()
                    }
                } else {
                    if self.detail_height > 0.0 {
                        self.detail_height -= 25.0;
                        Command::perform(async {
                            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
                        }, |_| Message::AnimateDetails(false))
                    } else {
                        self.detail_height = 0.0;
                        Command::none()
                    }
                }
            }
            Message::MenuSelected(menu_type) => {
                // Menü değişikliğini sadece disabled değilse uygula
                if !self.is_menu_disabled(&menu_type) {
                    self.active_menu = menu_type;
                }
                Command::none()
            }
            Message::ThemeChanged(theme) => {
                self.theme = theme;
                Command::none()
            }
            Message::ExitApp => {
                std::process::exit(0);
            }
            Message::LogoClicked => {
                self.active_menu = MenuType::Home;
                Command::none()
            }
            Message::ServerIpChanged(ip) => {
                self.server_ip = ip;
                Command::none()
            }
            Message::ServerPortChanged(port) => {
                self.server_port = port;
                Command::none()
            }
            Message::Connect => {
                self.connected = true;
                self.active_menu = MenuType::Home;

                if let Ok(mut logs) = self.connection_logs.lock() {
                    logs.clear();
                    logs.push("Connecting...".to_string());
                }
                
                let server_ip = self.server_ip.clone();
                let server_port = self.server_port.parse::<u16>().unwrap_or(19132);
                let tx = self.tx.clone().unwrap();
                let logs = self.connection_logs.clone();

                if let Some(runtime) = &self.runtime {
                    runtime.spawn(async move {
                        match client::create(
                            server_ip.clone(),
                            server_port,
                            "1.21.80".to_string(),
                            false,
                            {
                                let tx = tx.clone();
                                let logs = logs.clone();
                                move |code: &str, url: &str| {
                                    let message = format!("Auth Code: {} URL: {}", code, url);
                                    if let Ok(mut logs) = logs.lock() {
                                        logs.clear();
                                        logs.push(message.clone());
                                    }
                                    let _ = tx.send(Message::ConnectionError(message));
                                }
                            }
                        ).await {
                            Some(mut client) => {
                                let tx_packets = tx.clone();
                                
                                client.set_packet_callback(move |packet_name| {
                                    let packet_name = packet_name.to_string();
                                    let tx = tx_packets.clone();
                                    
                                    tokio::spawn(async move {
                                        let msg = Message::PacketReceived(
                                            packet_name.clone(),
                                            format!("Packet content: {}", packet_name),
                                            packet_name.len()
                                        );

                                        let _ = tx.send(msg);
                                    });
                                });

                                if let Err(e) = client.connect() {
                                    let _ = tx.send(Message::ConnectionError(format!("Connection error: {}", e)));
                                }
                            }
                            None => {
                                let _ = tx.send(Message::ConnectionError("Failed to create client".to_string()));
                            }
                        }
                    });
                }

                Command::none()
            },
            Message::AuthCodeChanged(_) => {
                if self.connected {
                    Command::perform(async {
                        tokio::time::sleep(std::time::Duration::from_millis(16)).await;
                    }, |_| Message::AuthCodeChanged(()))
                } else {
                    Command::none()
                }
            },
            Message::ClientConnected => {
                self.connected = true;
                Command::none()
            },
            Message::PacketReceived(packet_type, content, size) => {
                // Sadece capturing true iken paketleri kaydet
                if self.capturing {
                    let timestamp = chrono::Local::now().format("%H:%M:%S").to_string();
                    let new_packet = Packet {
                        name: format!("Packet_{}", self.packets.len() + 1),
                        packet_type: packet_type.clone(),
                        content,
                        timestamp,
                        size: size.max(1),
                    };
                    self.packets.insert(0, new_packet);
                    self.active_menu = MenuType::Packets;
                    Command::batch(vec![
                        Command::perform(async {}, |_| Message::UpdatePackets)
                    ])
                } else {
                    Command::none()
                }
            },
            Message::UpdatePackets => {
                Command::none()
            },
            Message::ConnectionError(error) => {
                if let Ok(mut logs) = self.connection_logs.lock() {
                    logs.clear();
                    logs.push(error.clone());
                }
                
                if !error.contains("Auth Code:") {
                    self.connected = false;
                }
                
                Command::none()
            },
            Message::OpenGithub => {
                if let Err(e) = open::that("https://github.com/ismaileke") {
                    println!("Failed to open GitHub: {}", e);
                }
                Command::none()
            },
            Message::OpenAuthUrl => {
                // Url'i kopyala
                let url = if let Ok(logs) = self.connection_logs.lock() {
                    if let Some(log) = logs.last() {
                        if let Some(url_start) = log.find("URL: ") {
                            Some(log[url_start + 5..].to_string())
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                };

                // Eğer URL bulunduysa async işlem başlat
                if let Some(url) = url {
                    Command::perform(
                        async move {
                            if let Err(e) = open::that(&url) {
                                println!("[DEBUG] Failed to open URL: {}", e);
                            }
                        },
                        |_| Message::AuthCodeChanged(())
                    )
                } else {
                    println!("[DEBUG] No URL found to open");
                    Command::none()
                }
            },
            Message::ToggleCapture => {
                self.capturing = !self.capturing;
                Command::none()
            },
            Message::FilterChanged(filter) => {
                self.filter_text = filter;
                Command::none()
            },
        }
    }

    fn view(&self) -> Element<Message> {
        let sidebar = self.view_sidebar();
        let main_content = match self.active_menu {
            MenuType::Home => self.view_home(),
            MenuType::Packets => {
                self.view_packets()
            },
            MenuType::Statistics => self.view_statistics(),
            MenuType::Settings => self.view_settings(),
        };

        container(
            row![
                sidebar,
                main_content,
            ]
            .spacing(10)
            .padding(5)
        )
        .style(iced::theme::Container::Custom(Box::new(DarkContainer { 
            theme: self.theme.clone() 
        })))
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}

// PacketViewer's own methods
impl PacketViewer {
    pub fn is_menu_disabled(&self, menu: &MenuType) -> bool {
        match menu {
            MenuType::Packets | MenuType::Statistics => {
                // Paket listesi boşsa bu menüler devre dışı
                self.packets.is_empty()
            }
            _ => false // Diğer menüler her zaman aktif
        }
    }

    fn view_sidebar(&self) -> Element<Message> {
        container(
            column![
                // Logo area
                {
                    let logo_button = button(
                        container(
                            Image::new(iced::widget::image::Handle::from_memory(LOGO_BYTES))
                                .width(60)
                                .height(60)
                        )
                        .padding(12)
                        .width(Length::Fixed(160.0))
                        .height(Length::Fixed(80.0))
                        .center_x()
                        .center_y()
                        .style(iced::theme::Container::Custom(Box::new(LogoContainer { 
                            theme: self.theme.clone() 
                        })))
                    )
                    .style(iced::theme::Button::Custom(Box::new(ButtonStyle { 
                        is_selected: false,
                        theme: self.theme.clone(),
                        disabled: false  // Add this
                    })));

                    if !self.is_menu_disabled(&MenuType::Home) {
                        logo_button.on_press(Message::LogoClicked)
                    } else {
                        logo_button
                    }
                },

                // Menu items
                container(
                    column![
                        // Packets menu
                        {
                            let packets_button = button(
                                container(
                                    row![
                                        Image::new(iced::widget::image::Handle::from_memory(PACKETS_ICON))
                                            .width(24)
                                            .height(24),
                                        text("Packets")
                                            .size(16)
                                            .horizontal_alignment(iced::alignment::Horizontal::Center)
                                    ]
                                    .spacing(8)
                                )
                                .width(Length::Fill)
                                .height(Length::Fill)
                                .center_x()
                                .center_y()
                                .padding(8)
                            )
                            .style(iced::theme::Button::Custom(Box::new(ButtonStyle { 
                                is_selected: self.active_menu == MenuType::Packets,
                                theme: self.theme.clone(),
                                disabled: self.is_menu_disabled(&MenuType::Packets)
                            })))
                            .width(Length::Fixed(160.0))
                            .height(Length::Fixed(60.0));

                            if !self.is_menu_disabled(&MenuType::Packets) {
                                packets_button.on_press(Message::MenuSelected(MenuType::Packets))
                            } else {
                                packets_button
                            }
                        },

                        // Statistics menu 
                        {
                            let stats_button = button(
                                container(
                                    row![
                                        Image::new(iced::widget::image::Handle::from_memory(STATS_ICON))
                                            .width(24)
                                            .height(24),
                                        text("Statistics")
                                            .size(16)
                                            .horizontal_alignment(iced::alignment::Horizontal::Center)
                                    ]
                                    .spacing(8)
                                )
                                .width(Length::Fill)
                                .height(Length::Fill)
                                .center_x()
                                .center_y()
                            )
                            .style(iced::theme::Button::Custom(Box::new(ButtonStyle { 
                                is_selected: self.active_menu == MenuType::Statistics,
                                theme: self.theme.clone(),
                                disabled: self.is_menu_disabled(&MenuType::Statistics)
                            })))
                            .width(Length::Fixed(160.0))
                            .height(Length::Fixed(60.0));

                            if !self.is_menu_disabled(&MenuType::Statistics) {
                                stats_button.on_press(Message::MenuSelected(MenuType::Statistics))
                            } else {
                                stats_button
                            }
                        },

                        // Settings menu
                        {
                            let settings_button = button(
                                container(
                                    row![
                                        Image::new(iced::widget::image::Handle::from_memory(SETTINGS_ICON))
                                            .width(24)
                                            .height(24),
                                        text("Settings")
                                            .size(16)
                                            .horizontal_alignment(iced::alignment::Horizontal::Center)
                                    ]
                                    .spacing(8)
                                )
                                .width(Length::Fill)
                                .height(Length::Fill)
                                .center_x()
                                .center_y()
                            )
                            .style(iced::theme::Button::Custom(Box::new(ButtonStyle { 
                                is_selected: self.active_menu == MenuType::Settings,
                                theme: self.theme.clone(),
                                disabled: false  // Add this
                            })))
                            .width(Length::Fixed(160.0))
                            .height(Length::Fixed(60.0));

                            if !self.is_menu_disabled(&MenuType::Settings) {
                                settings_button.on_press(Message::MenuSelected(MenuType::Settings))
                            } else {
                                settings_button
                            }
                        },

                        // Spacer
                        container(text("")).height(Length::Fill),
                        
                        // Exit button
                        button(
                            container(
                                text("Exit")
                                    .size(16)
                                    .horizontal_alignment(iced::alignment::Horizontal::Center)
                            )
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .center_x()
                            .center_y()
                        )
                        .on_press(Message::ExitApp)
                        .width(Length::Fixed(160.0))
                        .height(Length::Fixed(60.0))
                        .style(iced::theme::Button::Custom(Box::new(ButtonStyle { 
                            is_selected: false,
                            theme: self.theme.clone(),
                            disabled: false  // Add this
                        })))
                    ]
                    .spacing(10)
                )
                .width(Length::Fill)
            ]
            .spacing(15)
        )
        .style(iced::theme::Container::Custom(Box::new(SidebarContainer { theme: self.theme.clone() })))
        .width(Length::Fixed(160.0))
        .height(Length::Fill)
        .padding(10)
        .into()
    }
    fn view_home(&self) -> Element<Message> {
        let title = text("Bedrock Packet Viewer")
            .size(32)
            .width(Length::Fill)
            .horizontal_alignment(iced::alignment::Horizontal::Center);

        let mut main_content = column![title]
            .spacing(20)
            .align_items(iced::Alignment::Center);

        if !self.connected { // Remove unnecessary parentheses
            // Bağlı değilken gösterilecek içerik
            main_content = main_content
                .push(
                    text("Welcome to Bedrock Packet Viewer")
                        .size(18)
                        .width(Length::Fill)
                        .horizontal_alignment(iced::alignment::Horizontal::Center)
                )
                .push(container(text("")).height(Length::Fixed(40.0)))
                .push(
                    container(
                        column![
                            text("Server IP").size(16),
                            text_input("Enter server IP", &self.server_ip)
                                .on_input(Message::ServerIpChanged)
                                .padding(10)
                                .size(16)
                                .style(iced::theme::TextInput::Custom(Box::new(TextInputStyle { 
                                    theme: self.theme.clone(),
                                    rgb_offset: self.rgb_offset
                                })))
                        ]
                        .spacing(10)
                    )
                    .width(Length::Fixed(300.0))
                )
                .push(
                    container(
                        column![
                            text("Server Port").size(16),
                            text_input("Enter server port", &self.server_port)
                                .on_input(Message::ServerPortChanged)
                                .padding(10)
                                .size(16)
                                .style(iced::theme::TextInput::Custom(Box::new(TextInputStyle { 
                                    theme: self.theme.clone(),
                                    rgb_offset: self.rgb_offset
                                })))
                        ]
                        .spacing(10)
                    )
                    .width(Length::Fixed(300.0))
                )
                .push(
                    button(
                        container(
                            text("Connect")
                                .size(16)
                                .horizontal_alignment(iced::alignment::Horizontal::Center)
                        )
                        .width(Length::Fixed(150.0))
                        .padding(12)
                        .center_x()
                        .center_y()
                    )
                    .style(iced::theme::Button::Custom(Box::new(ButtonStyle { 
                        is_selected: false,
                        theme: self.theme.clone(),
                        disabled: false  // Add this
                    })))
                    .width(Length::Fixed(150.0))
                    .on_press(Message::Connect)
                );
        } else {
            // Bağlıyken gösterilecek içerik
            if let Ok(logs) = self.connection_logs.lock() {
                if let Some(log) = logs.last() {
                    if log.contains("Auth Code:") && log.contains("URL:") {
                        let parts: Vec<&str> = log.split(" URL: ").collect();
                        let auth_code = parts[0].trim_start_matches("Auth Code: ");
                        
                        main_content = main_content
                            .push(container(text("")).height(Length::Fixed(40.0)))
                            .push(
                                text(format!("Server:  {}:{}", self.server_ip, self.server_port))
                                    .size(18)
                                    .style(if self.theme == ThemeType::Dark {
                                        Color::from_rgb(240.0,218.0,216.0)
                                    } else {
                                        Color::from_rgb(0.2, 0.2, 0.2)
                                    })
                            )
                            .push(
                                container(
                                    column![
                                        CenteredTextInput::new(auth_code)
                                            .style(iced::theme::TextInput::Custom(Box::new(TextInputStyle {
                                                theme: self.theme.clone(),
                                                rgb_offset: self.rgb_offset,
                                            })))
                                            .on_input(|_| Message::AuthCodeChanged(())),
                                        button(
                                            text("Click to open browser")
                                                .size(16)
                                                .style(if self.theme == ThemeType::Dark {
                                                    Color::from_rgb(0.4, 0.6, 0.9)
                                                } else {
                                                    Color::from_rgb(0.2, 0.4, 0.8)
                                                })
                                        )
                                        .on_press(Message::OpenAuthUrl)
                                        .style(iced::theme::Button::Text)
                                    ]
                                    .spacing(10)
                                    .align_items(iced::Alignment::Center)
                                )
                                .padding(10)
                            );
                    } else {
                        main_content = main_content
                            .push(container(text("")).height(Length::Fixed(40.0)))
                            .push(
                                text("Connecting...")
                                    .size(16)
                            );
                    }
                }
            }
        }

        let content = container(main_content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_y();

        let content_with_github = container(
            column![
                content,
                container(
                    button(
                        text("GitHub")
                            .size(14)
                            .style(if self.theme == ThemeType::Dark {
                                Color::from_rgb(0.4, 0.6, 0.9)
                            } else {
                                Color::from_rgb(0.2, 0.4, 0.8)
                            })
                    )
                    .on_press(Message::OpenGithub)
                    .style(iced::theme::Button::Text)
                )
                .width(Length::Fill)
                .align_x(iced::alignment::Horizontal::Right)
                .padding(20)
            ]
        )
        .width(Length::Fill)
        .height(Length::Fill);

        container(content_with_github)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(iced::theme::Container::Custom(Box::new(MainContainer { 
                theme: self.theme.clone() 
            })))
            .into()
    }

    fn view_packets(&self) -> Element<Message> {
        let content = container(
            column![
                // Header kısmı
                container(
                    column![
                        text("Bedrock Packet Viewer").size(28).width(Length::Fill),
                        row![
                            // ...existing filter and control elements...
                            text_input("Filter packets...", &self.filter_text)
                                .on_input(Message::FilterChanged)
                                .padding(8)
                                .width(Length::Fixed(200.0))
                                .style(iced::theme::TextInput::Custom(Box::new(TextInputStyle {
                                    theme: self.theme.clone(),
                                    rgb_offset: self.rgb_offset,
                                }))),
                            container(text("")).width(Length::Fill), // Spacer
                            text("Total: ").size(16),
                            text(self.packets.len().to_string()).size(16),
                            button(
                                container(
                                    text(if self.capturing { "Stop Capture" } else { "Start Capture" })
                                        .size(14)
                                        .width(Length::Fixed(100.0))
                                        .horizontal_alignment(iced::alignment::Horizontal::Center)
                                )
                            )
                            .style(iced::theme::Button::Custom(Box::new(ButtonStyle {
                                is_selected: self.capturing,
                                theme: self.theme.clone(),
                                disabled: false
                            })))
                            .width(Length::Fixed(120.0))
                            .padding(8)
                            .on_press(Message::ToggleCapture)
                        ]
                        .spacing(15)
                        .align_items(iced::Alignment::Center)
                    ]
                    .spacing(10)
                )
                .padding(20)
                .width(Length::Fill),

                // Packet listesi container'ı
                container(
                    scrollable(
                        if self.packets.is_empty() {
                            // Doğrudan Element<Message> tipini belirtiyoruz
                            container(
                                text("No packets received yet...").size(20)
                            )
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .center_x()
                            .center_y()
                            .into()
                        } else {
                            // Column'u oluşturup sonra Element'e dönüştürüyoruz
                            let column: Element<_> = Column::with_children(
                                self.packets.iter().enumerate()
                                    .filter(|(_, p)| {
                                        if self.filter_text.is_empty() {
                                            true
                                        } else {
                                            p.packet_type.to_lowercase().contains(&self.filter_text.to_lowercase())
                                        }
                                    })
                                    .map(|(i, packet)| {
                                        let mut packet_content = column![
                                            container(
                                                button(
                                                    container(
                                                        row![
                                                            text(&packet.timestamp).size(14).width(Length::Fixed(100.0)),
                                                            text(&packet.packet_type).size(14).width(Length::Fill),
                                                            text(format!("{} bytes", packet.size)).size(14).width(Length::Fixed(100.0))
                                                        ]
                                                        .spacing(10)
                                                    )
                                                    .padding(10)
                                                )
                                                .style(iced::theme::Button::Custom(Box::new(ButtonStyle {
                                                    is_selected: Some(i) == self.selected_packet,
                                                    theme: self.theme.clone(),
                                                    disabled: false
                                                })))
                                                .width(Length::Fill)
                                                .on_press(Message::PacketSelected(i))
                                            )
                                            .padding(5)
                                        ];

                                        // Seçili paket detaylarını ekle
                                        if Some(i) == self.selected_packet {
                                            packet_content = packet_content.push(
                                                container(
                                                    column![
                                                        text("Packet Details").size(16),
                                                        text(format!("Name: {}", packet.name)).size(14),
                                                        text(format!("Type: {}", packet.packet_type)).size(14),
                                                        text(format!("Content: {}", packet.content)).size(14),
                                                        text(format!("Time: {}", packet.timestamp)).size(14),
                                                        text(format!("Size: {} bytes", packet.size)).size(14)
                                                    ]
                                                    .spacing(5)
                                                )
                                                .padding(10)
                                                .style(iced::theme::Container::Custom(Box::new(DetailContainer {
                                                    theme: self.theme.clone()
                                                })))
                                            );
                                        }

                                        packet_content.into()
                                    }).collect()
                            )
                            .spacing(10)
                            .into();
                            
                            column
                        }
                    )
                    .height(Length::Fill)
                )
                .style(iced::theme::Container::Custom(Box::new(DetailContainer {
                    theme: self.theme.clone()
                })))
                .width(Length::Fill)
                .height(Length::Fill)
            ]
            .spacing(10)
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(20)
        .style(iced::theme::Container::Custom(Box::new(MainContainer {
            theme: self.theme.clone()
        })));

        content.into()
    }

    fn view_statistics(&self) -> Element<Message> {
        let stats = self.calculate_stats();
        let max_count = self.packets.len().max(1) as f32;
        
        // İstatistiklerdeki yazıların pozisyonlarını düzenle
        let mut sorted_types: Vec<_> = stats.unique_types.iter().collect();
        sorted_types.sort_by(|a, b| {
            // Önce sayıya göre azalan sıralama
            let count_cmp = b.1.cmp(a.1);
            if count_cmp == std::cmp::Ordering::Equal {
                // Eğer sayılar eşitse, paket ismine göre sırala
                a.0.cmp(b.0)
            } else {
                count_cmp
            }
        });

        container(
            column![
                text("Packet Statistics").size(28).width(Length::Fill),
                
                // Line graph - clone stats here
                container(
                    custom_widget(stats.clone(), max_count, self.theme.clone())
                )
                .padding(20)
                .width(Length::Fill)
                .height(Length::Fixed(300.0)),

                // Stats summary
                container(
                    column![
                        text(format!("Unique Packet Types: {}", stats.unique_types.len())).size(16),
                        text(format!("Total Packets: {}", self.packets.len())).size(16),
                        text(format!("Total Size: {} bytes", stats.total_bytes)).size(16),
                    ]
                    .spacing(10)
                )
                .padding(20)
                .style(iced::theme::Container::Custom(Box::new(DetailContainer { 
                    theme: self.theme.clone() 
                })))
            ]
            .spacing(20)
        )
        .style(iced::theme::Container::Custom(Box::new(MainContainer { 
            theme: self.theme.clone() 
        })))
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(20)
        .into()
    }

    fn calculate_stats(&self) -> PacketStats {
        let mut stats = PacketStats {
            unique_types: std::collections::HashMap::new(),
            total_bytes: 0,
        };

        for packet in self.packets.iter() {
            *stats.unique_types.entry(packet.packet_type.clone()).or_insert(0) += 1;
            stats.total_bytes += packet.size;
        }

        stats
    }

    fn view_settings(&self) -> Element<Message> {
        container(
            column![
                // Başlık
                text("Settings")
                    .size(28)
                    .width(Length::Fill),

                // Tema seçimi
                container(
                    column![
                        text("Theme").size(16),
                        row![
                            // Tema metni
                            text(if self.theme == ThemeType::Dark {
                                "Dark Theme"
                            } else {
                                "Light Theme"
                            })
                            .size(14),
                            
                            // Boşluk ekleyerek toggle'ı sağa kaydıralım
                            container(text(""))
                                .width(Length::Fill),
                            
                            // Toggle switch
                            button(
                                container(
                                    // Kaydırılan daire
                                    container(text(""))
                                        .width(Length::Fixed(20.0))
                                        .height(Length::Fixed(20.0))
                                        .style(iced::theme::Container::Custom(Box::new(ToggleStyle { 
                                            is_active: true,
                                            theme: self.theme.clone()
                                        })))
                                )
                                .width(Length::Fixed(50.0))
                                .height(Length::Fixed(24.0))
                                .padding(2)
                                .align_x(if self.theme == ThemeType::Light {
                                    iced::alignment::Horizontal::Right
                                } else {
                                    iced::alignment::Horizontal::Left
                                })
                                .style(iced::theme::Container::Custom(Box::new(ToggleStyle { 
                                    is_active: false,
                                    theme: self.theme.clone()
                                })))
                            )
                            .style(iced::theme::Button::Custom(Box::new(ButtonStyle { 
                                is_selected: self.active_menu == MenuType::Settings,
                                theme: self.theme.clone(),
                                disabled: false  // Add this
                            })))
                            .on_press(Message::ThemeChanged(
                                if self.theme == ThemeType::Dark {
                                    ThemeType::Light
                                } else {
                                    ThemeType::Dark
                                }
                            )),
                        ]
                        .spacing(10)
                        .align_items(iced::Alignment::Center)
                    ]
                    .spacing(10)
                )
                .padding(20)
                .style(iced::theme::Container::Custom(Box::new(DetailContainer { 
                    theme: self.theme.clone() 
                }))),

                // Spacer
                container(text("")).height(Length::Fill),
            ]
            .spacing(20)
        )
        .style(iced::theme::Container::Custom(Box::new(MainContainer { 
            theme: self.theme.clone() 
        })))
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(20)
        .into()
    }
}

// New style for bar graph
struct BarContainer {
    color: Color,
}

impl container::StyleSheet for BarContainer {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(self.color)),
            text_color: Some(Color::WHITE),
            border_radius: 8.0.into(),
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
        }
    }
}

// New struct for toggle switch style
struct ToggleStyle {
    is_active: bool,
    theme: ThemeType,
}

impl container::StyleSheet for ToggleStyle {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        let (bg_color, circle_color) = if self.theme == ThemeType::Dark {
            if self.is_active {
                (Color::from_rgb(0.2, 0.4, 0.8), Color::WHITE)
            } else {
                (Color::from_rgb(0.2, 0.2, 0.2), Color::from_rgb(0.6, 0.6, 0.6))
                }
        } else {
            if self.is_active {
                (Color::from_rgb(0.4, 0.6, 0.9), Color::WHITE)
            } else {
                (Color::from_rgb(0.8, 0.8, 0.8), Color::from_rgb(0.9, 0.9, 0.9))
            }
        };

        container::Appearance {
            background: Some(Background::Color(bg_color)),
            text_color: Some(circle_color),
            border_radius: 15.0.into(),
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
        }
    }
}

// Add struct for TextInput style
struct TextInputStyle {
    theme: ThemeType,
    rgb_offset: f32,  // RGB animasyonu için offset
}

// Then impl block
impl text_input::StyleSheet for TextInputStyle {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> text_input::Appearance {
        let rgb_color = Color::from_rgb(
            (self.rgb_offset.sin() * 0.5 + 0.5) as f32,
            ((self.rgb_offset + 2.094).sin() * 0.5 + 0.5) as f32,  // 2.094 = 2π/3
            ((self.rgb_offset + 4.189).sin() * 0.5 + 0.5) as f32,  // 4.189 = 4π/3
        );

        text_input::Appearance {
            background: Background::Color(if self.theme == ThemeType::Dark {
                Color::from_rgb(0.15, 0.18, 0.25)
            } else {
                Color::from_rgb(0.95, 0.95, 0.95)
            }),
            border_radius: 8.0.into(),
            border_width: 1.0,
            border_color: rgb_color,
            icon_color: if self.theme == ThemeType::Dark {
                Color::WHITE
            } else {
                Color::BLACK
            },
        }
    }

    fn focused(&self, style: &Self::Style) -> text_input::Appearance {
        let active = self.active(style);
        let rgb_color = Color::from_rgb(
            (self.rgb_offset.sin() * 0.5 + 0.5) as f32,
            ((self.rgb_offset + 2.094).sin() * 0.5 + 0.5) as f32,
            ((self.rgb_offset + 4.189).sin() * 0.5 + 0.5) as f32,
        );

        text_input::Appearance {
            border_color: rgb_color,
            ..active
        }
    }

    fn placeholder_color(&self, _style: &Self::Style) -> Color {
        if self.theme == ThemeType::Dark {
            Color::from_rgba(1.0, 1.0, 1.0, 0.4)
        } else {
            Color::from_rgba(0.0, 0.0, 0.0, 0.4)
        }
    }

    fn value_color(&self, _style: &Self::Style) -> Color {
        if self.theme == ThemeType::Dark {
            Color::WHITE
        } else {
            Color::BLACK
        }
    }

    fn selection_color(&self, _style: &Self::Style) -> Color {
        if self.theme == ThemeType::Dark {
            Color::from_rgba(1.0, 1.0, 1.0, 0.2)
        } else {
            Color::from_rgba(0.0, 0.0, 0.0, 0.2)
        }
    }

    // Eksik metodları ekleyelim
    fn disabled_color(&self, _style: &Self::Style) -> Color {
        if self.theme == ThemeType::Dark {
            Color::from_rgba(1.0, 1.0, 1.0, 0.3)
        } else {
            Color::from_rgba(0.0, 0.0, 0.0, 0.3)
        }
    }

    fn disabled(&self, style: &Self::Style) -> text_input::Appearance {
        let active = self.active(style);
        text_input::Appearance {
            background: Background::Color(if self.theme == ThemeType::Dark {
                Color::from_rgb(0.1, 0.13, 0.2)
            } else {
                Color::from_rgb(0.9, 0.9, 0.9)
            }),
            ..active
        }
    }
}

// Fix the builder pattern for CenteredTextInput
struct CenteredTextInput;

impl CenteredTextInput {
    fn new<'a>(value: &str) -> text_input::TextInput<'a, Message> {
        text_input::TextInput::new("", value)
            .width(Length::Fixed(130.0))
            .padding(10)
            .size(18)
    }
}

struct LineGraph {
    stats: PacketStats,
    max_count: f32,
    theme: ThemeType,
    cache: Cache,
}

impl Program<Message, iced::Renderer> for LineGraph {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &iced::Renderer,
        _theme: &Theme,
        bounds: iced::Rectangle,
        _cursor: Cursor,
    ) -> Vec<Geometry> {
        let geom = self.cache.draw(renderer, bounds.size(), |frame| {
            let width = bounds.width;
            let height = bounds.height;
            
            // Sort and take top 10 packets with stable sorting
            let mut sorted_types: Vec<_> = self.stats.unique_types.iter().collect();
            sorted_types.sort_by(|a, b| {
                // Önce sayıya göre azalan sıralama
                let count_cmp = b.1.cmp(a.1);
                if count_cmp == std::cmp::Ordering::Equal {
                    // Eğer sayılar eşitse, paket ismine göre artan sıralama
                    a.0.cmp(b.0)
                } else {
                    count_cmp
                }
            });
            sorted_types.truncate(10); // 20 yerine 10 paket göster

            if !sorted_types.is_empty() {
                let step = width / (sorted_types.len() as f32 - 1.0).max(1.0);
                let max_count = sorted_types.first().map(|(_, count)| **count).unwrap_or(1) as f32;
                let scale = height / max_count;

                // Draw line
                let path = Path::new(|p| {
                    let mut first = true;
                    for (i, (_, count)) in sorted_types.iter().enumerate() {
                        let x = i as f32 * step;
                        let y = height - (**count as f32 * scale);
                        
                        if first {
                            p.move_to(iced::Point::new(x, y));
                            first = false;
                        } else {
                            p.line_to(iced::Point::new(x, y));
                        }
                    }
                });

                let color = if self.theme == ThemeType::Dark {
                    Color::from_rgb(0.4, 0.6, 0.9)
                } else {
                    Color::from_rgb(0.2, 0.4, 0.8)
                };

                frame.stroke(
                    &path,
                    Stroke {
                        width: 2.0,
                        style: Style::Solid(color),
                        line_cap: LineCap::Round,
                        ..Stroke::default()
                    },
                );

                // Draw points and labels with smart positioning
                let mut last_y_position: Option<f32> = None;  // Explicit type annotation
                const MIN_VERTICAL_SPACING: f32 = 25.0; // Minimum dikey boşluk
                
                for (i, (packet_type, count)) in sorted_types.iter().enumerate() {
                    let x = i as f32 * step;
                    let y = height - (**count as f32 * scale);
                    
                    // Nokta çizimi
                    frame.fill(
                        &Path::circle(iced::Point::new(x, y), 4.0),
                        Fill {
                            style: Style::Solid(color),
                            ..Fill::default()
                        },
                    );

                    // Etiket pozisyonunu hesapla
                    let mut text_y = y - 15.0; // Varsayılan pozisyon (üstte)
                    
                    // Eğer bir önceki etiketle çakışma varsa, pozisyonu ayarla
                    if let Some(last_y) = last_y_position {
                        if (text_y - last_y).abs() < MIN_VERTICAL_SPACING {
                            // Eğer önceki etiket yakınsa, bu etiketi altta göster
                            text_y = y + 20.0;
                        }
                    }
                    
                    // Etiket metnini çiz
                    let text = canvas::Text {
                        content: format!("{} ({})", packet_type, count),
                        position: iced::Point::new(x - 30.0, text_y),
                        color: if self.theme == ThemeType::Dark {
                            Color::WHITE
                        } else {
                            Color::from_rgb(0.2, 0.2, 0.2)
                        },
                        size: 12.0,
                        ..canvas::Text::default()
                    };
                    frame.fill_text(text);
                    
                    // Son y pozisyonunu güncelle
                    last_y_position = Some(text_y);
                }
            }
        });

        vec![geom]
    }
}

fn custom_widget(stats: PacketStats, max_count: f32, theme: ThemeType) -> Canvas<LineGraph, Message> {
    Canvas::new(LineGraph {
        stats,
        max_count,
        theme,
        cache: Cache::new(),
    })
    .width(Length::Fill)
    .height(Length::Fill)
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> iced::Result {
    let mut settings = Settings::default();
    
    // İkon dosyasını byte array olarak yükle
    let icon_bytes = include_bytes!("assets/logo.ico");
    
    settings.window = iced::window::Settings {
        size: (1024, 700),
        resizable: false,
        decorations: true,
        min_size: None,
        max_size: None,
        icon: Some(iced::window::icon::from_file_data(
            icon_bytes,
            None // Format otomatik tespit edilsin
        ).expect("Failed to load icon")),
        position: iced::window::Position::Centered,
        ..Default::default()
    };
    settings.antialiasing = true;
    settings.default_text_size = 16.0;
    settings.default_font = iced::Font::with_name("Minecraft");

    // Sonra uygulamayı başlat
    let app = PacketViewer::run(settings);


    app
}
