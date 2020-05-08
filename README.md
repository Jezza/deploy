
Just a crappy little program that can download job artefacts from a gitlab runner.

Just configure it via `toml`, and away you go.

## Usage

```shell script
deploy service

deploy service service2
```

##Configuration

```toml

host = "gitlab.example.com"

# Default download folder. 
output = "/srv/www/deploy"

# simply run `deploy service`, and this configuration will be "deployed". 
[deploy.service]

# Gitlab project name ("group/name" for example)
project = "dev/service"

# The git reference for a specific pipeline that was run.
# In this case, the master brach would be used.
ref_name = "master"

# The artifact path.
file = "service.jar"

# The name of the job itself.
job = "Build Service"

# [Optional] Sets the output folder, uses this instead of the global output if present.
output = "/srv/deploy"

[deploy.service2]
project = "dev/service2"
ref_name = "master"
file = "service2.jar"
job = "Build Service"

```

By default, it'll look for a configuration file within the standard config directories.
Linux: ~/.config/deploys.toml
Windows: Some `appdata` crap. (The application will spit out the location it tried to use, if it finds none.)

You can also override the config file via a switch `--config file`
