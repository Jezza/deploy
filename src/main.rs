use std::collections::HashMap;

use serde::Deserialize;
use structopt::StructOpt;
use gitlab::Gitlab;
use std::path::PathBuf;

#[derive(StructOpt, Debug)]
struct Args {
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
//	println!("{:#?}", args);

	let config = {
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
//	println!("{:#?}", config);

	let token = rpassword::read_password_from_tty(Some("Token: "))?;
	if token.is_empty() {
		eprintln!("No token provided.");
		return Ok(())
	}


	let gl = {
		let host = config.host;

		use gitlab::GitlabBuilder;
		let mut builder = GitlabBuilder::new(host, &token);
		builder.cert_insecure();
		builder.build()?
	};

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

	let path = if let Some(output) = deploy.output.as_deref().or(output) {
		let mut path = PathBuf::new();
		path.push(output);
		path.push(&deploy.file);
		path
	} else {
		eprintln!("No output folder specified for {}", name);
		return Ok(());
	};

	let project = gl.project_by_name::<_, Vec<(&'static str, &'static str)>, _, _>(&deploy.project, vec![])?;

	let options = {
		let mut opts = std::fs::OpenOptions::new();
		opts.write(true);
		opts.create(true);
		opts.truncate(true);
        opts
	};

	println!("Creating file \"{}\"", path.display());
	let mut file = options.open(&path)?;

	gl.artifact(
		project.id,
		&deploy.ref_name,
        &deploy.file,
        &deploy.job,
		&mut file,
	)?;

	Ok(())
}
