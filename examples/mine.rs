#![allow(unused)]
use tokio::{
	runtime,
	//task,
	time::{sleep, Duration},
};
/*
use std::{
	time::Duration,
	thread,
};
*/
use std::sync::{Mutex, Arc};

fn main(){
	let rt = runtime::Builder::new_multi_thread()
		.worker_threads(1)
		.enable_all()
		.build()
		.unwrap();

	rt.block_on(async{
		for i in 0..30{
			println!("\x1b[32mLOOPING SPAWN()\x1b[0m");
			tokio::spawn(async move{
				println!("\x1b[34;1mhere {}\x1b[0m", i);
				sleep(Duration::from_millis(1000)).await;
				println!("\x1b[33;1mhere {}\x1b[0m", i);
			});
		}
		let mut x = 0;
		for i in 0..300000000{
			x += 1;
		}
		println!("spin {}", x);
	});

}


/*
fn main(){
	//let mut rt = tokio::runtime::Runtime::new().unwrap();
	let rt = runtime::Builder::new_current_thread().build().unwrap();

	rt.block_on(async{
		println!("\x1b[93mMY MAIN STARTS NOW\x1b[0m");
		let mut data = Arc::new(Mutex::new(0));
		for i in 0..3 {
			println!("\x1b[93mMY MAIN SPAWNING TASKS\x1b[0m");
			let mut wrapper = Arc::clone(&data);
			let handle = tokio::spawn(async move{
				println!("\x1b[93mhello {}, {:?}\x1b[0m", i, thread::current().id());
				let mut wrapper = wrapper.lock().unwrap();
				*wrapper += 1;
			});
			handle.await;
		}
		println!("\x1b[93mMY MAIN EXITS WITH DATA : {}\x1b[0m", data.lock().unwrap());
	})
}
*/
