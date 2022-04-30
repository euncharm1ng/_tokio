#![allow(unused)]

use tokio::runtime;

fn main(){
	let rt = Builder::new_multi_thread()
		.worker_threads(2)
		.build()
		.unwrap();

	rt.block_on(async{
		for i in 0..3{
			tokio::spawn(async{
				println!("here");
			});
		}
	});
}
