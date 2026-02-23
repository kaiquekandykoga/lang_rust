use std::env;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OperatingSystem {
	Mac,
	Windows,
	Linux,
	Other,
}

#[derive(Debug)]
struct AppCheck {
	name: &'static str,
	candidates: Vec<PathBuf>,
	installed_path: Option<PathBuf>,
}

impl AppCheck {
	fn new(name: &'static str, candidates: Vec<PathBuf>) -> Self {
		Self {
			name,
			candidates,
			installed_path: None,
		}
	}
}

fn main() {
	let operating_system = detect_operating_system();
	let mut apps = build_apps(operating_system);
	detect_installed_apps(&mut apps);
	print_report(&apps);
}

fn detect_operating_system() -> OperatingSystem {
	match env::consts::OS {
		"macos" => OperatingSystem::Mac,
		"windows" => OperatingSystem::Windows,
		"linux" => OperatingSystem::Linux,
		_ => OperatingSystem::Other,
	}
}

fn build_apps(operating_system: OperatingSystem) -> Vec<AppCheck> {
	if operating_system != OperatingSystem::Mac {
		eprintln!("Warning: this sample currently checks macOS-style application paths.");
	}

	vec![
		AppCheck::new("Bitwarden", bitwarden_candidates()),
		AppCheck::new("Chrome", chrome_candidates()),
		AppCheck::new("Firefox", firefox_candidates()),
		AppCheck::new("MacPorts", macports_candidates()),
		AppCheck::new("Ollama", ollama_candidates()),
		AppCheck::new("Rancher Desktop", rancherdesktop_candidates()),
		AppCheck::new("Safari", safari_candidates()),
		AppCheck::new("Visual Studio Code", visualstudiocode_candidates()),
		AppCheck::new("WhatsApp", whatsapp_candidates()),
	]
}

fn detect_installed_apps(apps: &mut [AppCheck]) {
	for app in apps {
		app.installed_path = app.candidates.iter().find(|path| path.exists()).cloned();
	}
}

fn print_report(apps: &[AppCheck]) {
	println!("\x1b[32mInstalled\x1b[0m");
	for app in apps.iter().filter(|app| app.installed_path.is_some()) {
		if let Some(path) = &app.installed_path {
			println!("{:<24} {:<16}", app.name, path.display());
		}
	}

	println!("\n\x1b[31mNot Installed\x1b[0m");
	for app in apps.iter().filter(|app| app.installed_path.is_none()) {
		println!("{:<24}", app.name);
	}
}

fn bitwarden_candidates() -> Vec<PathBuf> {
	vec![
		PathBuf::from("/Applications/Bitwarden.app"),
		home_app_path("Bitwarden.app"),
	]
}

fn chrome_candidates() -> Vec<PathBuf> {
	vec![
		PathBuf::from("/Applications/Google Chrome.app"),
		PathBuf::from("/Applications/Google Chrome Dev.app"),
		home_app_path("Google Chrome.app"),
		home_app_path("Google Chrome Dev.app"),
	]
}

fn firefox_candidates() -> Vec<PathBuf> {
	vec![
		PathBuf::from("/Applications/Firefox.app"),
		PathBuf::from("/Applications/Firefox Developer Edition.app"),
		home_app_path("Firefox.app"),
		home_app_path("Firefox Developer Edition.app"),
	]
}

fn macports_candidates() -> Vec<PathBuf> {
	vec![PathBuf::from("/opt/local/bin/port")]
}

fn ollama_candidates() -> Vec<PathBuf> {
	vec![
		PathBuf::from("/Applications/Ollama.app"),
		home_app_path("Ollama.app"),
	]
}

fn rancherdesktop_candidates() -> Vec<PathBuf> {
	vec![
		PathBuf::from("/Applications/Rancher Desktop.app"),
		home_app_path("Rancher Desktop.app"),
	]
}

fn safari_candidates() -> Vec<PathBuf> {
	vec![
		PathBuf::from("/Applications/Safari.app"),
		home_app_path("Safari.app"),
	]
}

fn visualstudiocode_candidates() -> Vec<PathBuf> {
	vec![
		PathBuf::from("/Applications/Visual Studio Code.app"),
		home_app_path("Visual Studio Code.app"),
	]
}

fn whatsapp_candidates() -> Vec<PathBuf> {
	vec![
		PathBuf::from("/Applications/WhatsApp.app"),
		home_app_path("WhatsApp.app"),
	]
}

fn home_app_path(app_name: &str) -> PathBuf {
	home_dir().join("Applications").join(app_name)
}

fn home_dir() -> PathBuf {
	env::var_os("HOME")
		.map(PathBuf::from)
		.unwrap_or_else(|| Path::new("/").to_path_buf())
}
