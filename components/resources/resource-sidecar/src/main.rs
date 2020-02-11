use std::env;

enum Mode {
    Version,
    Load,
    Push
}

impl Mode {
    fn from_env(env_var: &str) -> Result<Mode, String> {
        let var_string = env::var(env_var).expect(&format!("{} environment variable not found", env_var));

        match var_string.as_str() {
            "Version" => Ok(Mode::Version),
            "Load" => Ok(Mode::Load),
            "Push" => Ok(Mode::Push),
            _ => Err(format!("Mode string {} not recognised", var_string))
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Waiting for pod to finish...");
    waitForPod().await?;

    println!("Processing...");
    let mode = Mode::from_env("RESOURCE_MODE").expect("Could not load mode from environment");
    return match mode {
        Mode::Version => addVersion(),
        Mode::Load => Ok(()),
        Mode::Push => Ok(())
    }
}

async fn waitForPod() -> anyhow::Result<()> {
    return Ok(());
}

fn addVersion() -> anyhow::Result<()> {
    return Ok(());
}
