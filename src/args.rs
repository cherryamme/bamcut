use clap::Parser;
use clap::builder::styling::{AnsiColor, Effects, Styles};

fn styles() -> Styles {
    Styles::styled()
        .header(AnsiColor::Yellow.on_default() | Effects::BOLD)
        .usage(AnsiColor::Yellow.on_default() | Effects::BOLD)
        .literal(AnsiColor::Blue.on_default() | Effects::BOLD)
        .placeholder(AnsiColor::Green.on_default())
}
#[derive(Parser, Debug, Clone)]
#[command(version, author, about, long_about = None, styles = styles())]
#[command(
    help_template = "{usage-heading} {usage} \nVersion: {version} {about-section}Author:{author} Email:jiancghen2@genomics.cn/cherryamme@qq.com\n {all-args} {tab}"
)]
pub struct Args {
    /// The path of input file
    #[arg(short, long)]
    pub inputs: String,
	/// The region to extract (e.g., chr1:1000-2000)
	#[arg(short, long)]
	pub region: String,
    /// The name of outdir
    #[arg(short, long, default_value = "bamcut.fq.gz")]
    pub outfile: String,
    /// The maximum number of reads to cut in the region
    #[arg(short, long)]
    pub max_reads: Option<usize>,
}
