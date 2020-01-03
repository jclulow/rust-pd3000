use std::process::exit;

pub struct PD3000 {
    handle: rusb::DeviceHandle<rusb::GlobalContext>,
}

impl PD3000 {
    pub fn open() -> PD3000 {
        let devs = match rusb::devices() {
            Err(e) => {
                eprintln!("ERROR: get devices: {}", e);
                exit(2);
            }
            Ok(d) => d
        };

        let devs: Vec<rusb::Device<rusb::GlobalContext>> =
            devs.iter().filter(|dev|
        {
            let dd = match dev.device_descriptor() {
                Err(_) => return false,
                Ok(dd) => dd
            };

            if dd.vendor_id() != 0x0FA8 || dd.product_id() != 0xA030 {
                return false;
            }

            true
        }).collect();

        if devs.len() < 1 {
            eprintln!("ERROR: no USB device found");
            exit(2); /* XXX */
        } else if devs.len() > 1 {
            eprintln!("ERROR: multiple matching USB devices found");
            exit(2); /* XXX */
        }
        let dev = &devs[0];

        println!("device: {:#?}", dev);

        let dh = match dev.open() {
            Err(e) => {
                eprintln!("ERROR: open device: {}", e);
                exit(2);
            }
            Ok(dh) => dh
        };

        PD3000 {
            handle: dh
        }
    }

    pub fn writes(&self, s: &str) {
        self.handle.write_bulk(2, s.as_bytes(),
            std::time::Duration::from_secs(5)).unwrap();
    }

    pub fn writec(&self, c: char) {
        let msg: [u8; 1] = [ c as u8 ];
        self.handle.write_bulk(2, &msg,
            std::time::Duration::from_secs(5)).unwrap();
    }

    pub fn mode_vertical_scroll(&self) {
        /*
         * Data is written into the second row and transferred to the first row
         * when carriage return is received, leaving the second row empty.
         */
        self.writec(0x12 as char);
    }

    pub fn mode_normal(&self) {
        /*
         * Data can be written into either row.  Moves to the left most digit of
         * the other row when line is full.
         */
        self.writec(0x11 as char);
    }

    pub fn cursor_hide(&self) {
        self.writec(0x14 as char);
    }

    pub fn cursor_show(&self) {
        self.writec(0x13 as char);
    }

    pub fn move_to(&self, x: u8, y: u8) {
        let mut x = x;
        let mut y = y;
        if x > 19 {
            x = 19;
        }
        if y > 1 {
            y = 1;
        }
        let pos: u8 = y * 20 + x;

        self.writec(0x10 as char);
        self.writec(pos as char);
    }

    pub fn scroller(&self, s: &str) {
        /*
         * XXX string must be no more than 45 characters?
         */
        self.writec(0x05 as char);
        self.writes(s);
        self.writec(0x0d as char);
    }

    pub fn reset(&self) {
        /*
         * 1F: All characters are erased and all settings are returned to the
         * power-on reset conditions.
         */
        self.writec(0x1F as char);
    }
}
