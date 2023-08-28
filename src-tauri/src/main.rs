// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use arm_command::{ArmCommand, ARM_PRODUCT, ARM_VENDOR};

mod arm_command;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn up(name: &str) -> &str {
    let Ok(context) =  libusb::Context::new() else {
        return "No context"
    };

    let Some(handle) = context
        .open_device_with_vid_pid(ARM_VENDOR, ARM_PRODUCT) else {
            return "Device not found or missing permissions";
        };

    let mut cmd = ArmCommand::new();
    let Some(c) = arm_command::ARM_CONTROL::from(name) else {return "Command not found"};
    cmd.add_control(c, arm_command::DIRECTION::UP);
    let Ok(_) = cmd.send(&handle) else {return "Cannot send"};
    ""
}

#[tauri::command]
fn down(name: &str) -> &str {
    let Ok(context) =  libusb::Context::new() else {
        return "No context"
    };

    let Some(handle) = context
        .open_device_with_vid_pid(ARM_VENDOR, ARM_PRODUCT) else {
            return "Device not found or missing permissions";
        };

    let mut cmd = ArmCommand::new();
    let Some(c) = arm_command::ARM_CONTROL::from(name) else {return "Command not found"};
    cmd.add_control(c, arm_command::DIRECTION::DOWN);
    let Ok(_) = cmd.send(&handle) else {return "Cannot send"};
    ""
}

#[tauri::command]
fn zero(name: &str) -> &str {
    let Ok(context) =  libusb::Context::new() else {
        return "No context"
    };

    let Some(handle) = context
        .open_device_with_vid_pid(ARM_VENDOR, ARM_PRODUCT) else {
            return "Device not found or missing permissions";
        };

    let mut cmd = ArmCommand::new();
    let Some(c) = arm_command::ARM_CONTROL::from(name) else {return "Command not found"};
    cmd.add_control(c, arm_command::DIRECTION::ZERO);
    let Ok(_) = cmd.send(&handle) else {return "Cannot send"};
    ""
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![up, down, zero])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
