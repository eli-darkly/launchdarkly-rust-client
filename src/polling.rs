
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};
use std::thread;
use std::thread::JoinHandle;
use std::time;

use feature_store::FeatureStore;
use requestor::Requestor;


pub struct PollingProcessor {
	task: Arc<PollingProcessorTask>,
	handle: Option<JoinHandle<()>>,
	ready: Receiver<()>
}

struct PollingProcessorTask {
	requestor: Requestor,
	interval: u64,
	store: Arc<Mutex<FeatureStore>>,
	ready: Mutex<Sender<()>>
}

impl PollingProcessor {
	pub fn new(sdk_key: String, store: &Arc<Mutex<FeatureStore>>, base_uri: &String, interval: u64) -> PollingProcessor {
		let (tx, rx): (Sender<()>, Receiver<()>) = mpsc::channel();
		let task: Arc<PollingProcessorTask> = Arc::new(PollingProcessorTask {
			requestor: Requestor::new(sdk_key, base_uri),
			interval: interval,
			store: store.clone(),
			ready: Mutex::new(tx)
		});
		PollingProcessor {
			task: task,
			handle: None,
			ready: rx
		}
	}

	pub fn start(&mut self) -> &Receiver<()> {
		let task = self.task.clone();
		let handle = thread::spawn(move || {
			loop {
				match task.requestor.get_all_flags() {
					Ok(flags) => {
						let mut store = task.store.lock().unwrap();
						store.init(&flags);
						// signal that we're ready
						task.ready.lock().unwrap().send(()).unwrap();
					},
					_ => () // TODO: error logging
				}
				thread::sleep(time::Duration::from_millis(task.interval));
			}
		});
		self.handle = Some(handle);
		&self.ready
	}

	// TODO: need a way to stop the task.
}
