extern crate enet_sys as enet;

fn main() {
    if unsafe { enet::enet_initialize() } != 0 {
        println!("error");
    }

    println!("yop la");
    unsafe { 
        let s = enet::server_new();
        loop {
            enet::handle_event(s);
            std::thread::sleep(std::time::Duration::from_millis(100));
        }

        enet::enet_host_destroy(s);
    }

    unsafe { enet::enet_deinitialize(); }
}


