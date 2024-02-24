struct TPLink;

impl TPLink {
    pub fn get_devices() -> Vec<SmartDevice> {
        for (addr, data) in discover().unwrap() {
			let mut connected_devices: Vec<SmartDevice> = vec![];
			let sysinfo = data.sysinfo();
			println!("{}\t{}\t{}\t{}", addr, sysinfo.alias, sysinfo.hw_type, sysinfo.model);
			match &*sysinfo.model {
				"KL130(US)" | "KL110(US)" => {
					let device = KL110::from_addr(addr);
					println!("Device name: {}", device.sysinfo().unwrap().alias);
				}
				"HS100(US)" => {
					let device = HS100::from_addr(addr);
					println!("Device name: {}", device.sysinfo().unwrap().alias);
				}
				"HS105(US)" => {
					let device = HS105::from_addr(addr);
					println!("Device name: {}", device.sysinfo().unwrap().alias);
				}
				"LB120(US)" => {
					let device = LB120::from_addr(addr);
					println!("Device name: {}", device.sysinfo().unwrap().alias);
				}
				"LB100(US)" | "LB110(US)" | "LB200(E26)"=> {
					let device = LB110::from_addr(addr);
					println!("Device name: {}", device.sysinfo().unwrap().alias);
				}
				&_ => {
					println!("device not yet supported");
				}
			}
	  	}
    }
}