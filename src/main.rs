use std::path::Path;
use structopt::StructOpt;
use serde::Deserialize;
use git2;
use dirs;

#[derive(Debug, StructOpt)]
#[structopt(name = "cloudgrabber", about = "Tool for grabbing data from trickest cloud repo.")]
struct Opt {
    #[structopt(long = "output", default_value = "ips")]
    output: String,

    #[structopt(long = "company")]
    company: String,
}

#[derive(Debug, Deserialize)]
struct SSLInfo {
    #[serde(rename = "IP Address")]
    ip_address: String,

    #[serde(rename = "Common Name")]
    common_name: String,

    #[serde(rename = "Organization")]
    organization: String,

    #[serde(rename = "Subject Alternative DNS Name")]
    subject_alt_dns_name: String,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let opt = Opt::from_args();

    let home_dir = dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Could not determine the home directory."))?;
    let ssl_dir = home_dir.join(Path::new(".cloudgrabber").join("cloud").join("ssl"));
    let repo_dir = home_dir.join(".cloudgrabber").join("cloud");
    let repo_url = "https://github.com/trickest/cloud.git";

    if !ssl_dir.exists() || ssl_dir.read_dir()?.next().is_none() {
        git2::Repository::clone(repo_url, &repo_dir)?;
    }

    let dir_entries = ssl_dir.read_dir()?;
    for entry in dir_entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let mut reader = csv::Reader::from_path(&path)?;
            for result in reader.deserialize() {
                let record: SSLInfo = result?;
                if record.organization == opt.company {
                    let output = match &*opt.output {
                        "ips" => record.ip_address,
                        "dns-name" => record.subject_alt_dns_name,
                        "common-name" => record.common_name,
                        "domains" => {
                            let dns_name = record.subject_alt_dns_name;
                            if dns_name.contains('|') {
                                let dns_names: Vec<&str> = dns_name.split('|').collect();
                                dns_names.join("\n")
                            } else {
                                dns_name
                            }
                        }
                        _ => return Err(anyhow::anyhow!("Unsupported output format. Supported formats are ips, dns-name, common-name, domains.")),
                    };
                    println!("{}", output);
                }
            }
        }
    }

    Ok(())
}
