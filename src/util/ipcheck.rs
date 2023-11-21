/*
 * @Author: Image image@by.cx
 * @Date: 2023-11-21 14:56:42
 * @LastEditors: Image image@by.cx
 * @LastEditTime: 2023-11-21 16:06:45
 * @filePathColon: /
 * @Description: 
 * 
 * Copyright (c) 2023 by ${git_name_email}, All Rights Reserved. 
 */
pub mod checker{
    use std::net::Ipv4Addr;

    use std::io::{BufRead, BufReader};
    use rust_lapper::{Interval, Lapper};
    //国家列表
    pub enum Country{
        CN,
        US,
        JP,
        KR,
        RU,
        UK,
        FR,
        DE,
        IT,
        ES,
        CA,
        AU,
        BR,
        IN,
        MX,
        NL,
        SE,
        TR,
        PL,
        ID,
        AR,
    }
    fn ipv4_to_u32(ip: Ipv4Addr) -> u32 {
        let octets = ip.octets();
        ((octets[0] as u32) << 24) + ((octets[1] as u32) << 16) + ((octets[2] as u32) << 8) + (octets[3] as u32)
    }
    pub struct Checker{
        intervals:Vec<Interval<u32,()>>,
        lapper:Lapper<u32,()>,
    }

    impl Checker{
        pub fn new() -> Checker{
            Checker{
                intervals:Vec::new(),
                lapper:Lapper::new(Vec::new())
            }
        }
        pub fn read_ip(&mut self, country:Country){
            let mut path = "d:\\china_ip.txt";
            match country{
                Country::CN => {
                    path ="d:\\china_ip.txt";
                },
                _ => {
                    println!("Not CN");
                }
            }
            
            let in_file:std::fs::File = std::fs::File::open(path).expect("");
            let mut cidrs:Vec<String> = Vec::new();
            let reader = BufReader::new(in_file);
            for line in reader.lines() {
                cidrs.push(line.expect(""));
            }
            
            self.lapper = Lapper::new(self.intervals.clone());
        }
        pub fn is_empty(&self) -> bool{
            if self.intervals.len() > 0{
                return false;
            }
            return true;
        }
        pub fn check_ip_str(&self, ip:String, country:Country) -> bool{
            let ip: Ipv4Addr = ip.parse().unwrap();
            return self.check_ip(ip, country);
        }
        pub fn check_ip(&self, ip:Ipv4Addr, country:Country) -> bool{
            let ip = ipv4_to_u32(ip);
            let overlaps = self.lapper.find(ip, ip);
            if overlaps.count() > 0{
                return true;
            }
            return false;
        }
    }
}