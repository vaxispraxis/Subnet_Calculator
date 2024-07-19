use eframe::egui; 
use ipnetwork::IpNetwork;
use std::net::{IpAddr, Ipv4Addr};
//using eframe and egui for the ui
//using ipnetwork for help with the ip caculations
fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Subnet Calculator",
        native_options,
        Box::new(|_cc| Ok(Box::new(SubnetCalculatorApp::default()))),
    );
}
//defining the app stat
//this struct hold all the data the app needs, like the ip address input and results of the subnet calculation (network address first/last host etc.)
#[derive(Default)]
struct SubnetCalculatorApp {
    ip_input: String,
    network: String,
    first_host: String,
    last_host: String,
    broadcast: String,
    next_subnet: String,
    subnet_mask: String,
    result_message: String,
}
//building the app interface
impl eframe::App for SubnetCalculatorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("IPv4 Subnet Calculator");

            ui.horizontal(|ui| {
                ui.label("IP Address / CIDR:");
                ui.text_edit_singleline(&mut self.ip_input);
            });

            if ui.button("Calculate").clicked() {
                self.calculate_subnet();
            }

            ui.separator();

            ui.label(format!("Network Address: {}", self.network));
            ui.label(format!("First Host: {}", self.first_host));
            ui.label(format!("Last Host: {}", self.last_host));
            ui.label(format!("Broadcast Address: {}", self.broadcast));
            ui.label(format!("Next Subnet: {}", self.next_subnet));
            ui.label(format!("Subnet Mask: {}", self.subnet_mask));

            if !self.result_message.is_empty() {
                ui.separator();
                ui.label(&self.result_message);
            }
        });
    }
}
//calculating subnet information
//this function does the actual work of caculating the ip address and cider input
impl SubnetCalculatorApp {
    fn calculate_subnet(&mut self) {
        let ip_str = self.ip_input.trim();

        match ip_str.parse::<IpNetwork>() {
            Ok(ip_network) => {
                match ip_network {
                    IpNetwork::V4(ipv4_network) => {
                        let network = ipv4_network.network();
                        let broadcast = ipv4_network.broadcast();
                        let first_host = increment_ip(network, 1);
                        let last_host = decrement_ip(broadcast, 1);
                        let next_subnet = increment_ip(broadcast, 1);
                        let subnet_mask = ipv4_network.mask();

                        self.network = network.to_string();
                        self.first_host = first_host.to_string();
                        self.last_host = last_host.to_string();
                        self.broadcast = broadcast.to_string();
                        self.next_subnet = next_subnet.to_string();
                        self.subnet_mask = subnet_mask.to_string();
                        self.result_message = String::new();
                    }
                    IpNetwork::V6(_) => {
                        self.result_message = "IPv6 is not supported".to_string();
                        self.clear_results();
                    }
                }
            }
            Err(_) => {
                self.result_message = "Invalid IP address or subnet mask".to_string();
                self.clear_results();
            }
        }
    }

    fn clear_results(&mut self) {
        self.network.clear();
        self.first_host.clear();
        self.last_host.clear();
        self.broadcast.clear();
        self.next_subnet.clear();
        self.subnet_mask.clear();
    }
}

fn increment_ip(ip: Ipv4Addr, increment: u32) -> Ipv4Addr {
    let ip_u32 = u32::from(ip);
    Ipv4Addr::from(ip_u32.wrapping_add(increment))
}

fn decrement_ip(ip: Ipv4Addr, decrement: u32) -> Ipv4Addr {
    let ip_u32 = u32::from(ip);
    Ipv4Addr::from(ip_u32.wrapping_sub(decrement))
}