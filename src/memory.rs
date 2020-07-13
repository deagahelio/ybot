use std::fs::File;
use std::fs::OpenOptions;
use std::io::{Seek, SeekFrom, Read};
use std::io::prelude::*;
use std::io;
use std::process::Command;

const BASE_ADDRESS: u32 = 0x36831F8;

pub fn get_process_pid(process_name: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let mut pid = Command::new("pidof").arg(process_name).output()?.stdout;
    pid.pop();
    Ok(std::str::from_utf8(&pid)?.parse()?)
}

pub struct GDMemory {
    mem: File,
    last_x_pos_address: u32,
    last_y_pos_address: u32,
    last_is_dead_address: u32,
    last_is_practice_mode_address: u32,
}

impl GDMemory {
    pub fn from_pid(pid: u32) -> io::Result<Self> {
        Ok(Self {
            mem: OpenOptions::new().read(true).write(true).open(format!("/proc/{}/mem", pid))?,
            last_x_pos_address: 0,
            last_y_pos_address: 0,
            last_is_dead_address: 0,
            last_is_practice_mode_address: 0,
        })
    }

    pub fn get_addr(&mut self, mut base: u32, offsets: Vec<u32>) -> io::Result<u32> {
        base += offsets[0];
        for offset in offsets.iter().skip(1) {
            base = self.read_int(base)?;
            base += offset;
        }
        Ok(base)
    }

    pub fn read_int(&mut self, addr: u32) -> io::Result<u32> {
        self.mem.seek(SeekFrom::Start(addr as u64))?;
        let mut buffer = [0; 4];
        self.mem.read_exact(&mut buffer)?;
        Ok(u32::from_le_bytes(buffer))
    }

    pub fn read_float(&mut self, addr: u32) -> io::Result<f32> {
        self.mem.seek(SeekFrom::Start(addr as u64))?;
        let mut buffer = [0; 4];
        self.mem.read_exact(&mut buffer)?;
        Ok(f32::from_le_bytes(buffer))
    }

    pub fn write_float(&mut self, addr: u32, val: f32) -> io::Result<()> {
        self.mem.seek(SeekFrom::Start(addr as u64))?;
        self.mem.write(&val.to_le_bytes())?;
        Ok(())
    }

    pub fn read_bool(&mut self, addr: u32) -> io::Result<bool> {
        self.mem.seek(SeekFrom::Start(addr as u64))?;
        let mut buffer = [0];
        self.mem.read_exact(&mut buffer)?;
        Ok(buffer[0] != 0)
    }

    pub fn get_x_pos(&mut self) -> io::Result<f32> {
        self.read_float(self.last_x_pos_address).or_else(|_| {
            self.last_x_pos_address = self.get_addr(BASE_ADDRESS, vec![0x164, 0x224, 0x67C])?;
            self.read_float(self.last_x_pos_address)
        })
    }

    pub fn set_x_pos(&mut self, val: f32) -> io::Result<()> {
        self.write_float(self.last_x_pos_address, val).or_else(|_| {
            self.last_x_pos_address = self.get_addr(BASE_ADDRESS, vec![0x164, 0x224, 0x67C])?;
            self.write_float(self.last_x_pos_address, val)
        })
    }

    pub fn get_y_pos(&mut self) -> io::Result<f32> {
        self.read_float(self.last_y_pos_address).or_else(|_| {
            self.last_y_pos_address = self.get_addr(BASE_ADDRESS, vec![0x164, 0x224, 0x680])?;
            self.read_float(self.last_y_pos_address)
        })
    }

    pub fn set_y_pos(&mut self, val: f32) -> io::Result<()> {
        self.write_float(self.last_y_pos_address, val).or_else(|_| {
            self.last_y_pos_address = self.get_addr(BASE_ADDRESS, vec![0x164, 0x224, 0x680])?;
            self.write_float(self.last_y_pos_address, val)
        })
    }

    pub fn is_dead(&mut self) -> io::Result<bool> {
        self.read_bool(self.last_is_dead_address).or_else(|_| {
            self.last_is_dead_address = self.get_addr(BASE_ADDRESS, vec![0x164, 0x39C])?;
            self.read_bool(self.last_is_dead_address)
        })
    }

    pub fn is_practice_mode(&mut self) -> io::Result<bool> {
        self.read_bool(self.last_is_practice_mode_address).or_else(|_| {
            self.last_is_practice_mode_address = self.get_addr(BASE_ADDRESS, vec![0x164, 0x495])?;
            self.read_bool(self.last_is_practice_mode_address)
        })
    }

    pub fn update_addresses(&mut self) -> io::Result<()> {
        self.last_x_pos_address = self.get_addr(BASE_ADDRESS, vec![0x164, 0x224, 0x67C])?;
        self.last_is_dead_address = self.get_addr(BASE_ADDRESS, vec![0x164, 0x39C])?;
        Ok(())
    }
}