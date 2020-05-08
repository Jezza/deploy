use std::collections::HashMap;
use std::path::PathBuf;

use gitlab::Gitlab;
use serde::Deserialize;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Args {
	#[structopt(parse(from_os_str), long)]
	config: Option<PathBuf>,

	deploys: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct Config {
	host: String,

	output: Option<String>,
	deploy: HashMap<String, Deploy>,
}

#[derive(Deserialize, Debug)]
struct Deploy {
	project: String,
	ref_name: String,
	file: String,
	job: String,
	output: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let args: Args = StructOpt::from_args();

	let config = if let Some(config) = args.config {
		config
	} else {
		let mut config = match dirs::config_dir() {
			Some(config) => config,
			None => {
				eprintln!("Unable to locate configuration directory.");
				return Ok(());
			}
		};
		config.push("deploys.toml");
		config
	};

	if !config.exists() {
		eprintln!("Configuration file does not exist. [{}]", config.display());
		return Ok(());
	}

	let content = std::fs::read_to_string(&config)?;
	let config = toml::from_str::<Config>(&content)?;

	let token = rpassword::read_password_from_tty(Some("Token: "))?;
	if token.is_empty() {
		eprintln!("No token provided.");
		return Ok(());
	}

	let gl = gitlab::GitlabBuilder::new(&config.host, &token)
		.cert_insecure()
		.build()?;

	for deploy in &args.deploys {
		let name = &*deploy;
		if let Some(deploy) = config.deploy.get(name) {
			download(&gl, config.output.as_deref(), name, deploy)?;
		} else {
			eprintln!("Unknown deploy key: {}", name);
		}
	}

	Ok(())
}

fn download(gl: &Gitlab, output: Option<&str>, name: &str, deploy: &Deploy) -> Result<(), Box<dyn std::error::Error>> {
	let path = match deploy.output.as_deref().or(output) {
		Some(output) => {
			let mut path = PathBuf::from(output);
			path.push(&deploy.file);
			path
		},
		None => {
			eprintln!("No output folder specified for {}", name);
			return Ok(());
		}
	};

	let project = gl.project_by_name::<_, Vec<(&'static str, &'static str)>, _, _>(&deploy.project, vec![])?;

	if path.exists() {
		let backup = {
			let mut backup = path.clone();
			let ext = backup.extension()
				.map(|ext| format!("{}.bak", ext.to_string_lossy()))
				.unwrap_or_else(|| format!(".bak"));
			backup.set_extension(&ext);
			backup
		};

		println!("Moving \"{}\" to \"{}\"", path.display(), backup.display());
		std::fs::rename(&path, backup)?;
	}

	println!("Creating file \"{}\"", path.display());
	let mut file = std::fs::OpenOptions::new()
		.write(true)
		.create_new(true)
		.open(&path)?;

	gl.artifact(
		project.id,
		&deploy.ref_name,
		&deploy.file,
		&deploy.job,
		&mut file,
	)?;

	Ok(())
}
