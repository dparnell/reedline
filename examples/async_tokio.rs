use std::time::Duration;
use futures::{pin_mut, select};
use futures::FutureExt;
use futures_timer::Delay;
use reedline::{DefaultPrompt, Reedline, Signal};

#[tokio::main]
async fn main() -> std::io::Result<()> {

    let mut line_editor = Reedline::create();
    let prompt = DefaultPrompt::default();

    loop {
        let mut delay = Delay::new(Duration::from_millis(5_000)).fuse();
        let signal = line_editor.next_line(&prompt).fuse();

        pin_mut!(signal);

        select! {
            _ = delay => println!("\nTICK!!!!"),
            maybe_sig = signal => {
                match maybe_sig {
                    Ok(sig) => {
                        match sig {
                            Signal::Success(buffer) => {
                                println!("We processed: {buffer}");
                            }
                            Signal::CtrlD | Signal::CtrlC => {
                                println!("\nAborted!");
                                break Ok(());
                            }
                        }
                    },
                    Err(e) => println!("ERROR: {}", e)
                }
            }

        }
    }
}