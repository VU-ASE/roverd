use futures::future::join_all;

use std::path::PathBuf;
use std::process::Stdio;
use std::{fs::File, time::Duration};

use tokio::{
    process::{Child, Command},
    select,
    sync::broadcast,
    time,
};

use crate::error::Error;

struct SpawnedProcess {
    name: String,
    child: Child,
}

impl SpawnedProcess {
    async fn terminate(&mut self) -> Result<(), Error> {
        if let Some(id) = self.child.id() {
            unsafe {
                println!("Sending terminate to {}", self.name);
                libc::kill(id as i32, libc::SIGTERM);
            }
        }
        Ok(())
    }

    async fn kill(&mut self) -> Result<(), Error> {
        println!("Sending kill to {}", self.name);
        self.child.kill().await?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum ProcessState {
    Started,
    Stopped,
    Killed,
}

#[derive(Debug, Clone)]
pub struct Process {
    pub pid: i32,
    pub last_exit_code: Option<i32>,
    pub name: String,
    pub command: String,
    pub log_file: PathBuf,
    pub state: ProcessState,
}


#[derive(Debug, Clone)]
pub struct ProcessManager {
    pub processes: Vec<Process>,
}

impl ProcessManager {
    pub async fn start(&self) -> Result<(), Error> {
        let (shutdown_tx, _) = broadcast::channel::<()>(1);
        let mut spawned: Vec<SpawnedProcess> = vec![];

        // Spawn all processes
        for p in &self.processes {
            let file = File::create(p.log_file.clone())?;
            let stdout = Stdio::from(file.try_clone()?);
            let stderr = Stdio::from(file);

            let mut command = Command::new("sh");
            command
                .arg("-c")
                .arg(p.command.clone())
                .stdout(stdout)
                .stderr(stderr);
            match command.spawn() {
                Ok(child) => {
                    spawned.push(SpawnedProcess {
                        name: p.name.clone(),
                        child,
                    });
                }
                Err(e) => {
                    println!("Failed to spawn process {}: {}", p.name, e);
                    shutdown_tx.send(())?;
                    break;
                }
            }
        }

        let mut handles = Vec::new();


        // tokio::spawn(tokio::task::spawn_blocking(move || {
        //     // Monitor each process
        for mut proc in spawned {
            let mut shutdown_rx = shutdown_tx.subscribe();
            let process_shutdown_tx = shutdown_tx.clone();

            let handle = tokio::spawn(async move {
                loop {
                    select! {
                        // Wait for process completion
                        status = proc.child.wait() => {
                            match status {
                                Ok(status) => {
                                    if !status.success() {
                                        println!("Process {} exited with error: {}", proc.name, status);
                                        process_shutdown_tx.send(()).ok();
                                    }
                                }
                                Err(e) => {
                                    println!("Error waiting for process {}: {}", proc.name, e);
                                    process_shutdown_tx.send(()).ok();
                                }
                            }
                            break;
                        }
                        // Wait for shutdown signal
                        _ = shutdown_rx.recv() => {
                            println!("Terminating process: {}", proc.name);
                            if let Err(e) = proc.terminate().await {
                                println!("Error terminating process {:?}: {:?}", proc.name, e);
                            }

                            // Wait for 1 second before sending KILL signal
                            time::sleep(Duration::from_secs(1)).await;

                            match proc.child.try_wait() {
                                Ok(None) => {
                                    println!("Force killing process: {}", proc.name);
                                    if let Err(e) = proc.kill().await {
                                        println!("Error killing process {:?}: {:?}", proc.name, e);
                                    }
                                },
                                Ok(Some(status)) => {
                                    println!("Successfully terminated child: {:?} with {:?}", proc.child, status)
                                },
                                Err(e) => {
                                    panic!("Error: {:?}", e);
                                }
                            }

                            break;
                        }
                    }
                }
            });
            handles.push(handle);
        }
            // Wait for all processes to complete
            // let done_processes = join_all(handles).await;

            // for result in done_processes {
            //     match result {
            //         Ok(_) => info!("Process completed successfully"),
            //         Err(e) => error!("Process failed: {}", e),
            //     }
            // }
        // }));

        
        Ok(())
    }

    pub async fn stop(&self) -> Result<(), Error> {
        Ok(())
    }
}
