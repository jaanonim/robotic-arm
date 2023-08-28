use std::{thread, time::Duration};

extern crate libusb;

pub const ARM_VENDOR: u16 = 0x1267;
pub const ARM_PRODUCT: u16 = 0;

pub enum DIRECTION {
    UP = 0x555555,   //0b01
    DOWN = 0xaaaaa,  //0b10
    ZERO = 0x000000, //0b00
}

#[derive(Clone)]
pub enum ARM_CONTROL {
    GRIP = 0x000003,
    WRIST = 0x00000c,
    ELBOW = 0x000030,
    SHOULDER = 0x0000c0,
    ROTATION = 0x000300,
    LED = 0x030000,
}

impl ARM_CONTROL {
    pub fn from(input: &str) -> Option<ARM_CONTROL> {
        match input {
            "grip" => Some(ARM_CONTROL::GRIP),
            "wrist" => Some(ARM_CONTROL::WRIST),
            "elbow" => Some(ARM_CONTROL::ELBOW),
            "shoulder" => Some(ARM_CONTROL::SHOULDER),
            "rotation" => Some(ARM_CONTROL::ROTATION),
            "led" => Some(ARM_CONTROL::LED),
            &_ => None,
        }
    }
}

pub struct ArmCommand {
    cmd: [u8; 3],
}

impl ArmCommand {
    pub fn new() -> Self {
        ArmCommand { cmd: [0, 0, 0] }
    }

    pub fn add_control(&mut self, control: ARM_CONTROL, direction: DIRECTION) -> &Self {
        let c = control as u32;
        let d = direction as u32;

        let mut v: u32 = 0;
        for i in 0..3 {
            v += (self.cmd[i] as u32) << (i * 8)
        }

        let res = (v & !c) | (d & c);

        for i in 0..3 {
            self.cmd[i] = ((res >> (i * 8)) & 0xff) as u8
        }
        self
    }

    pub fn send(&self, handle: &libusb::DeviceHandle<'_>) -> Result<usize, libusb::Error> {
        for i in 0..3 {
            print!("{:#010b} ", self.cmd[i]);
        }
        println!();
        handle.write_control(0x40, 6, 0x100, 0, &self.cmd, Duration::new(0, 0))
    }
}

pub fn test() {
    let context = libusb::Context::new().unwrap();
    let handle = context
        .open_device_with_vid_pid(ARM_VENDOR, ARM_PRODUCT)
        .ok_or("Device not found or missing permissions")
        .unwrap();

    let mut cmd = ArmCommand::new();

    for control in [
        ARM_CONTROL::GRIP,
        ARM_CONTROL::WRIST,
        ARM_CONTROL::ELBOW,
        ARM_CONTROL::SHOULDER,
        ARM_CONTROL::ROTATION,
        ARM_CONTROL::LED,
    ] {
        cmd.add_control(control.clone(), DIRECTION::UP);
        cmd.send(&handle).unwrap();

        thread::sleep(Duration::new(1, 500));

        cmd.add_control(control.clone(), DIRECTION::DOWN);
        cmd.send(&handle).unwrap();

        thread::sleep(Duration::new(1, 0));

        cmd.add_control(control.clone(), DIRECTION::ZERO);
        cmd.send(&handle).unwrap();
    }
}
