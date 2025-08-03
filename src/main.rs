use anyhow::Result;
use log::{info, warn, error};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::time;
use warp::Filter;
use embedded_graphics::{
    prelude::*,
    pixelcolor::Rgb565,
    mono_font::MonoTextStyle,
    primitives::{Circle, Rectangle, PrimitiveStyle, PrimitiveStyleBuilder},
    draw_target::DrawTarget,
};

// Conditional imports based on features
#[cfg(feature = "hardware")]
use rppal::gpio::{Gpio, OutputPin};
#[cfg(feature = "hardware")]
use rppal::spi::{Bus, Mode, SlaveSelect, Spi};
#[cfg(feature = "hardware")]
use st7789::{ST7789};

// Data structures for API responses
#[derive(Serialize, Deserialize, Clone)]
struct SensorData {
    temperature: f32,
    humidity: f32,
    timestamp: u64,
}

#[derive(Serialize, Deserialize)]
struct SystemStatus {
    uptime_seconds: u64,
    led_status: bool,
    last_sensor_reading: Option<SensorData>,
    display_content: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct LedControl {
    state: bool,
}

// Mock display for simulation
#[cfg(feature = "simulation")]
struct MockDisplay {
    content: String,
    width: u32,
    height: u32,
}

#[cfg(feature = "simulation")]
impl MockDisplay {
    fn new(width: u32, height: u32) -> Self {
        Self {
            content: String::new(),
            width,
            height,
        }
    }
    
    fn clear(&mut self) {
        self.content.clear();
        self.content.push_str(&format!("╔{}╗\n", "═".repeat(self.width as usize - 2)));
        for _ in 0..(self.height - 2) {
            self.content.push_str(&format!("║{}║\n", " ".repeat(self.width as usize - 2)));
        }
        self.content.push_str(&format!("╚{}╝\n", "═".repeat(self.width as usize - 2)));
    }
    
    fn add_text(&mut self, text: &str, x: u32, y: u32) {
        // Simple text positioning simulation
        let lines: Vec<&str> = self.content.lines().collect();
        let mut new_content = String::new();
        
        for (i, line) in lines.iter().enumerate() {
            if i == y as usize && y < self.height {
                let mut chars: Vec<char> = line.chars().collect();
                let start_pos = (x + 1) as usize; // Account for border
                
                if start_pos < chars.len() {
                    for (j, ch) in text.chars().enumerate() {
                        if start_pos + j < chars.len() - 1 { // Don't overwrite right border
                            chars[start_pos + j] = ch;
                        }
                    }
                }
                new_content.push_str(&chars.iter().collect::<String>());
            } else {
                new_content.push_str(line);
            }
            new_content.push('\n');
        }
        self.content = new_content;
    }
    
    fn get_content(&self) -> &str {
        &self.content
    }
}

// Display trait for abstraction
trait Display {
    fn clear(&mut self) -> Result<()>;
    fn draw_text(&mut self, text: &str, x: u32, y: u32, color: Rgb565) -> Result<()>;
    fn get_display_content(&self) -> Option<String>;
}

// Hardware display implementation
#[cfg(feature = "hardware")]
struct HardwareDisplay {
    display: ST7789<Spi, OutputPin, OutputPin>,
}

#[cfg(feature = "hardware")]
impl Display for HardwareDisplay {
    fn clear(&mut self) -> Result<()> {
        self.display.clear(Rgb565::BLACK)?;
        Ok(())
    }
    
    fn draw_text(&mut self, text: &str, x: u32, y: u32, color: Rgb565) -> Result<()> {
        use embedded_graphics::text::Text;
        use embedded_graphics::mono_font::ascii::FONT_10X20;
        let text_style = MonoTextStyle::new(&FONT_10X20, color);
        Text::new(text, Point::new(x as i32, y as i32), text_style)
            .draw(&mut self.display)?;
        Ok(())
    }
    
    fn get_display_content(&self) -> Option<String> {
        None // Hardware display doesn't provide content string
    }
}

// Simulation display implementation
#[cfg(feature = "simulation")]
struct SimulationDisplay {
    mock_display: MockDisplay,
}

#[cfg(feature = "simulation")]
impl Display for SimulationDisplay {
    fn clear(&mut self) -> Result<()> {
        self.mock_display.clear();
        Ok(())
    }
    
    fn draw_text(&mut self, text: &str, x: u32, y: u32, _color: Rgb565) -> Result<()> {
        self.mock_display.add_text(text, x / 10, y / 20); // Scale down coordinates
        Ok(())
    }
    
    fn get_display_content(&self) -> Option<String> {
        Some(self.mock_display.get_content().to_string())
    }
}

// Shared application state
#[derive(Clone)]
struct AppState {
    #[cfg(feature = "hardware")]
    led_pin: Arc<Mutex<OutputPin>>,
    #[cfg(feature = "simulation")]
    led_status: Arc<Mutex<bool>>,
    sensor_data: Arc<Mutex<Option<SensorData>>>,
    display: Arc<Mutex<Option<Box<dyn Display + Send>>>>,
    start_time: Instant,
}

impl AppState {
    fn new() -> Result<Self> {
        #[cfg(feature = "hardware")]
        {
            let gpio = Gpio::new()?;
            let led_pin = gpio.get(18)?.into_output();
            
            // Initialize display hardware
            let display: Option<Box<dyn Display + Send>> = match Self::init_hardware_display(&gpio) {
                Ok(disp) => Some(Box::new(disp)),
                Err(e) => {
                    warn!("Failed to initialize hardware display: {}", e);
                    None
                }
            };
            
            Ok(AppState {
                led_pin: Arc::new(Mutex::new(led_pin)),
                sensor_data: Arc::new(Mutex::new(None)),
                display: Arc::new(Mutex::new(display)),
                start_time: Instant::now(),
            })
        }
        
        #[cfg(feature = "simulation")]
        {
            info!("Running in simulation mode");
            let display: Option<Box<dyn Display + Send>> = Some(Box::new(SimulationDisplay {
                mock_display: MockDisplay::new(50, 15),
            }));
            
            Ok(AppState {
                led_status: Arc::new(Mutex::new(false)),
                sensor_data: Arc::new(Mutex::new(None)),
                display: Arc::new(Mutex::new(display)),
                start_time: Instant::now(),
            })
        }
    }
    
    #[cfg(feature = "hardware")]
    fn init_hardware_display(gpio: &Gpio) -> Result<HardwareDisplay> {
        // SPI configuration for ST7789
        let spi = Spi::new(Bus::Spi0, SlaveSelect::Ss0, 8_000_000, Mode::Mode0)?;
        
        // GPIO pins for display control
        let dc = gpio.get(24)?.into_output(); // Data/Command pin
        let reset = gpio.get(25)?.into_output(); // Reset pin
        
        // Initialize ST7789 display (simplified for version 0.6)
        let display = ST7789::new(spi, dc, reset, 240, 240);
        
        info!("ST7789 display initialized successfully");
        Ok(HardwareDisplay { display })
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();
    
    #[cfg(feature = "hardware")]
    info!("Starting Raspberry Pi Zero Rust application (Hardware Mode)");
    
    #[cfg(feature = "simulation")]
    info!("Starting Raspberry Pi Zero Rust application (Simulation Mode)");

    // Initialize application state
    let app_state = AppState::new()?;
    info!("Application initialized successfully");

    // Start sensor reading task
    let sensor_state = app_state.clone();
    tokio::spawn(async move {
        sensor_reading_task(sensor_state).await;
    });

    // Start LED task
    let led_state = app_state.clone();
    tokio::spawn(async move {
        led_task(led_state).await;
    });

    // Start display update task
    let display_state = app_state.clone();
    tokio::spawn(async move {
        display_update_task(display_state).await;
    });

    // Setup web API routes
    let api_state = app_state.clone();
    let routes = setup_routes(api_state);

    info!("Starting web server on port 3030");
    warp::serve(routes)
        .run(([0, 0, 0, 0], 3030))
        .await;

    Ok(())
}

// Simulated sensor reading task
async fn sensor_reading_task(state: AppState) {
    let mut interval = time::interval(Duration::from_secs(5));
    
    loop {
        interval.tick().await;
        
        // Simulate reading temperature and humidity sensors
        let temperature = simulate_temperature_reading();
        let humidity = simulate_humidity_reading();
        
        let sensor_data = SensorData {
            temperature,
            humidity,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        };
        
        // Update shared state
        if let Ok(mut data) = state.sensor_data.lock() {
            *data = Some(sensor_data.clone());
        }
        
        info!("Sensor reading: {:.1}°C, {:.1}% humidity", temperature, humidity);
        
        // Log warning if temperature is too high
        if temperature > 30.0 {
            warn!("High temperature detected: {:.1}°C", temperature);
        }
    }
}

// LED task (hardware or simulation)
async fn led_task(state: AppState) {
    let mut interval = time::interval(Duration::from_millis(1000));
    let mut led_on = false;
    
    loop {
        interval.tick().await;
        
        #[cfg(feature = "hardware")]
        {
            if let Ok(mut pin) = state.led_pin.lock() {
                if led_on {
                    pin.set_high();
                } else {
                    pin.set_low();
                }
            }
        }
        
        #[cfg(feature = "simulation")]
        {
            if let Ok(mut status) = state.led_status.lock() {
                *status = led_on;
            }
        }
        
        led_on = !led_on;
    }
}

// Display update task
async fn display_update_task(state: AppState) {
    let mut interval = time::interval(Duration::from_secs(2));
    
    loop {
        interval.tick().await;
        
        // Get current sensor data
        let sensor_data = if let Ok(data) = state.sensor_data.lock() {
            data.clone()
        } else {
            None
        };
        
        // Update display if available
        if let Ok(mut display_opt) = state.display.lock() {
            if let Some(ref mut display) = display_opt.as_mut() {
                if let Err(e) = update_display_content(display.as_mut(), &sensor_data, &state) {
                    error!("Failed to update display: {}", e);
                }
            }
        }
    }
}

// Function to update display content
fn update_display_content(
    display: &mut dyn Display,
    sensor_data: &Option<SensorData>,
    state: &AppState,
) -> Result<()> {
    // Clear display
    display.clear()?;
    
    // Display title
    display.draw_text("Hello World!", 10, 30, Rgb565::WHITE)?;
    display.draw_text("Pi Zero Monitor", 10, 60, Rgb565::WHITE)?;
    
    // Display sensor data if available
    if let Some(data) = sensor_data {
        let temp_text = format!("Temp: {:.1}C", data.temperature);
        display.draw_text(&temp_text, 10, 90, Rgb565::WHITE)?;
        
        let humidity_text = format!("Humidity: {:.1}%", data.humidity);
        display.draw_text(&humidity_text, 10, 120, Rgb565::WHITE)?;
        
        // Color-coded temperature warning
        if data.temperature > 30.0 {
            display.draw_text("HIGH TEMP!", 10, 150, Rgb565::RED)?;
        }
    } else {
        display.draw_text("No sensor data", 10, 90, Rgb565::WHITE)?;
    }
    
    // Display uptime
    let uptime = state.start_time.elapsed().as_secs();
    let uptime_text = format!("Uptime: {}s", uptime);
    display.draw_text(&uptime_text, 10, 180, Rgb565::WHITE)?;
    
    // LED status indicator
    #[cfg(feature = "hardware")]
    let led_status = if let Ok(pin) = state.led_pin.lock() {
        pin.is_set_high()
    } else {
        false
    };
    
    #[cfg(feature = "simulation")]
    let led_status = if let Ok(status) = state.led_status.lock() {
        *status
    } else {
        false
    };
    
    let led_color = if led_status { Rgb565::GREEN } else { Rgb565::RED };
    display.draw_text("LED", 10, 210, led_color)?;
    
    // Print to console in simulation mode
    #[cfg(feature = "simulation")]
    {
        if let Some(content) = display.get_display_content() {
            println!("\n🖥️  LCD Display Content:");
            println!("{}", content);
            println!("📊 Status: LED={}, Temp={:.1}°C", 
                if led_status { "ON" } else { "OFF" },
                sensor_data.as_ref().map(|d| d.temperature).unwrap_or(0.0)
            );
        }
    }
    
    Ok(())
}

// Setup web API routes
fn setup_routes(
    state: AppState,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["content-type"])
        .allow_methods(vec!["GET", "POST", "PUT"]);

    // GET /status - System status
    let status_route = warp::path("status")
        .and(warp::get())
        .and(with_state(state.clone()))
        .and_then(get_status_handler);

    // GET /sensor - Latest sensor data
    let sensor_route = warp::path("sensor")
        .and(warp::get())
        .and(with_state(state.clone()))
        .and_then(get_sensor_handler);

    // POST /led - Control LED
    let led_route = warp::path("led")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_state(state.clone()))
        .and_then(control_led_handler);

    // GET /display - Get display content (simulation mode)
    let display_route = warp::path("display")
        .and(warp::get())
        .and(with_state(state.clone()))
        .and_then(get_display_handler);

    // GET / - Simple welcome message
    let hello_route = warp::path::end()
        .and(warp::get())
        .map(|| "Raspberry Pi Zero Rust Server is running!");

    status_route
        .or(sensor_route)
        .or(led_route)
        .or(display_route)
        .or(hello_route)
        .with(cors)
}

// Helper function to pass state to handlers
fn with_state(
    state: AppState,
) -> impl Filter<Extract = (AppState,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || state.clone())
}

// API Handlers
async fn get_status_handler(state: AppState) -> Result<impl warp::Reply, warp::Rejection> {
    let uptime = state.start_time.elapsed().as_secs();
    
    #[cfg(feature = "hardware")]
    let led_status = if let Ok(pin) = state.led_pin.lock() {
        pin.is_set_high()
    } else {
        false
    };
    
    #[cfg(feature = "simulation")]
    let led_status = if let Ok(status) = state.led_status.lock() {
        *status
    } else {
        false
    };
    
    let last_sensor_reading = if let Ok(data) = state.sensor_data.lock() {
        data.clone()
    } else {
        None
    };
    
    let display_content = if let Ok(display_opt) = state.display.lock() {
        display_opt.as_ref().and_then(|d| d.get_display_content())
    } else {
        None
    };
    
    let status = SystemStatus {
        uptime_seconds: uptime,
        led_status,
        last_sensor_reading,
        display_content,
    };
    
    Ok(warp::reply::json(&status))
}

async fn get_sensor_handler(state: AppState) -> Result<impl warp::Reply, warp::Rejection> {
    let sensor_data = if let Ok(data) = state.sensor_data.lock() {
        data.clone()
    } else {
        None
    };
    
    match sensor_data {
        Some(data) => Ok(warp::reply::json(&data)),
        None => Ok(warp::reply::json(&serde_json::json!({
            "error": "No sensor data available"
        }))),
    }
}

async fn control_led_handler(
    led_control: LedControl,
    state: AppState,
) -> Result<impl warp::Reply, warp::Rejection> {
    #[cfg(feature = "hardware")]
    {
        if let Ok(mut pin) = state.led_pin.lock() {
            if led_control.state {
                pin.set_high();
                info!("LED turned ON via API");
            } else {
                pin.set_low();
                info!("LED turned OFF via API");
            }
        } else {
            error!("Failed to control LED");
            return Ok(warp::reply::json(&serde_json::json!({
                "success": false,
                "error": "Failed to access GPIO"
            })));
        }
    }
    
    #[cfg(feature = "simulation")]
    {
        if let Ok(mut status) = state.led_status.lock() {
            *status = led_control.state;
            info!("LED turned {} via API (simulation)", if led_control.state { "ON" } else { "OFF" });
        }
    }
    
    Ok(warp::reply::json(&serde_json::json!({
        "success": true,
        "led_state": led_control.state
    })))
}

async fn get_display_handler(state: AppState) -> Result<impl warp::Reply, warp::Rejection> {
    let display_content = if let Ok(display_opt) = state.display.lock() {
        display_opt.as_ref().and_then(|d| d.get_display_content())
    } else {
        None
    };
    
    match display_content {
        Some(content) => Ok(warp::reply::json(&serde_json::json!({
            "display_content": content,
            "mode": "simulation"
        }))),
        None => Ok(warp::reply::json(&serde_json::json!({
            "display_content": "Hardware mode - content not available via API",
            "mode": "hardware"
        }))),
    }
}

// Sensor simulation functions
fn simulate_temperature_reading() -> f32 {
    // Simulate temperature between 18-35°C with some randomness
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::time::SystemTime;
    
    let mut hasher = DefaultHasher::new();
    SystemTime::now().hash(&mut hasher);
    let random_value = (hasher.finish() % 1000) as f32 / 1000.0;
    
    20.0 + (random_value * 15.0)
}

fn simulate_humidity_reading() -> f32 {
    // Simulate humidity between 30-80%
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::time::SystemTime;
    
    let mut hasher = DefaultHasher::new();
    (SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos() + 12345).hash(&mut hasher);
    let random_value = (hasher.finish() % 1000) as f32 / 1000.0;
    
    40.0 + (random_value * 40.0)
}
