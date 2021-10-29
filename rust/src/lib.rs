#![no_std]
#![allow(dead_code,unused_variables)]

use core::slice;
use core::iter::Iterator;

const WHITESPACE: &[u8] = b" \t\n\r";

pub trait SliceSubsequence<T> {
    fn trim_start(&self, needle: &[T]) -> &[T];
    fn trim_end(&self, needle: &[T]) -> &[T];
    fn first_index_of(&self, needle: &[T]) -> Option<usize>;
    //fn extract_between(&self, before: &[T], after: &[T]) -> Option<&[T]>;
    fn lines<'a>(&'a self, needle: &'a [T]) -> LineIterator<'a, T>;

    fn trim(&self, needle: &[T]) -> &[T];

    fn contains_seq(&self, needle: &[T]) -> bool {
        self.first_index_of(needle).is_some()
    }
}

pub struct LineIterator<'a, T> {
    inner: &'a [T],
    needle: &'a [T],
    pos: usize,
}

impl<'a, T: PartialEq> Iterator for LineIterator<'a, T> {
    type Item = &'a [T];

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos == self.inner.len() {
            return None;
        }
        let search = &self.inner[self.pos..];
        match search.first_index_of(self.needle) {
            None => {
                let pos = self.pos;
                self.pos = self.inner.len();
                Some(&self.inner[pos..])
            }
            Some(idx) => {
                self.pos += idx + self.needle.len();
                Some(&search[..idx])
            }
        }
    }
}

fn last_index_of<T: PartialEq>(s: &[T], needle: &[T]) -> usize {
    let mut len = 0;
    for i in s {
        if needle.contains(i) {
            len += 1;
        } else {
            break;
        }
    }
    len
}

fn last_index_of_rev<T: PartialEq>(s: &[T], needle: &[T]) -> usize {
    let mut len = s.len();
    for i in s.iter().rev() {
        if needle.contains(i) {
            len -= 1;
        } else {
            break;
        }
    }
    len
}

impl<T: PartialEq> SliceSubsequence<T> for &[T] {
    fn trim_start(&self, needle: &[T]) -> &[T] {
        &self[last_index_of(self, needle)..]
    }

    fn trim_end(&self, needle: &[T]) -> &[T] {
        &self[..last_index_of_rev(self, needle)]
    }

    fn first_index_of(&self, needle: &[T]) -> Option<usize> {
        if self.len() >= needle.len() {
            for i in 0..self.len() - needle.len() + 1 {
                if self[i..i + needle.len()] == needle[..] {
                    return Some(i);
                }
            }
        }
        None
    }

    fn lines<'a>(&'a self, needle: &'a [T]) -> LineIterator<'a, T> {
        LineIterator {
            inner: self,
            needle,
            pos: 0,
        }
    }

    fn trim(&self, needle: &[T]) -> &[T] {
        let start = last_index_of(self, needle);
        let end = last_index_of_rev(self, needle);
        if start >= end {
            // empty
            &self[0..0]
        } else {
            &self[start..end]
        }
    }
}

/*
// for some reason, passing actual pointers like this freezes the n64
#[no_mangle]
pub extern "C" fn parse_cheats_ffi(
    cheat_file: *mut u8, cheat_file_len: usize,
    boot_cheats: *mut u32, boot_cheats_len: usize,
    in_game_cheats: *mut u32, in_game_cheats_len: usize,
) -> u8 {
    let cheat_file = unsafe { slice::from_raw_parts(cheat_file, cheat_file_len) };
    let boot_cheats = unsafe { slice::from_raw_parts_mut(boot_cheats, boot_cheats_len) };
    let in_game_cheats = unsafe { slice::from_raw_parts_mut(in_game_cheats, in_game_cheats_len) };

    parse_cheats(cheat_file, boot_cheats, in_game_cheats)
}
*/

// in C world, usize is actually u32, no idea why but that's a problem for another day...
// but if we put u32 here it crashes, can only send usize
#[no_mangle]
pub extern "C" fn parse_cheats_ffi(
    cheat_file: usize, cheat_file_len: usize,
    boot_cheats: usize, boot_cheats_len: usize,
    in_game_cheats: usize, in_game_cheats_len: usize,
) -> u8 {

/*
        unsafe {
        let cheat_file = cheat_file as *mut u8;
        let cheat_file = slice::from_raw_parts_mut(cheat_file, 4);
        cheat_file[1] = b'a';
        0
        }
*/
    //unsafe { screen_text_ptr(b"from rust woot" as *const u8 as u32); }

    //let cheat_file = unsafe { slice::from_raw_parts(cheat_file as *const u8, cheat_file_len as usize) };
    let cheat_file = unsafe { slice::from_raw_parts_mut(cheat_file as *mut u8, cheat_file_len as usize) };
    let boot_cheats = unsafe { slice::from_raw_parts_mut(boot_cheats as *mut u32, boot_cheats_len as usize) };
    let in_game_cheats = unsafe { slice::from_raw_parts_mut(in_game_cheats as *mut u32, in_game_cheats_len as usize) };
    
    dbg(100);
    
    //cheat_file[1] = b'a';
    
    //0

    parse_cheats(cheat_file, boot_cheats, in_game_cheats)
}

const SUCCESS: u8 = 0;
const INVALID_CODE_LINE: u8 = 1;
const INVALID_LINE: u8 = 2;

pub fn parse_cheats(cheat_file: &[u8], boot_cheats: &mut [u32], in_game_cheats: &mut [u32]) -> u8 {
    dbg(101);
    let mut repeater = false;
    let mut boot_cheats_idx = 0;
    let mut in_game_cheats_idx = 0;
    dbg(102);
    for line in cheat_file.lines(b"\n") {
        dbg(200);
        let line = line.trim(WHITESPACE);
        if line.is_empty() || line.starts_with(b"#") || line == b"---" {
            continue; // empty or comment or whatever the starting thing is
        } else if line.ends_with(b":") {
            repeater = false;
        } else if line.starts_with(b"- ") {
            // we found the start of a code
            let line = line.trim_start(b"- ");
            let line = line.trim(WHITESPACE);
            let mut line = line.lines(b" ");
            match (line.next(), line.next(), line.next()) {
                (Some(address), Some(value), None) => {
                    // proper line
                    let address = hex_to_u32(address.trim(WHITESPACE));
                    let value = hex_to_u32(value.trim(WHITESPACE));
                    //println!("address: {:X}, value: {:X}", address, value);
                    //println!("dec address: {}, value: {}", address, value);

                    // Do not check code types within "repeater data"
                    if repeater {
                        repeater = false;
                        in_game_cheats[in_game_cheats_idx] = address;
                        in_game_cheats_idx += 1;
                        in_game_cheats[in_game_cheats_idx] = value;
                        in_game_cheats_idx += 1;
                        continue;
                    }

                    //println!("address >> 24: {:X}", address >> 24);
                    // Determine destination cheat_list for the code type
                    match address >> 24
                    {
                        // Uncessary code types
                        0x20 | // Clear code list
                        0xCC | // Exception Handler Selection
                        0xDE => // Entry Point
                        continue,

                        // Boot-time cheats
                        0xEE | // Disable Expansion Pak
                        0xF0 | // 8-bit Boot-Time Write
                        0xF1 | // 16-bit Boot-Time Write
                        0xFF => { // Cheat Engine Location
                            boot_cheats[boot_cheats_idx] = address;
                            boot_cheats_idx += 1;
                            boot_cheats[boot_cheats_idx] = value;
                            boot_cheats_idx += 1;
                        }

                        // In-game cheats
                        0x50 => { // Repeater/Patch
                            // Validate repeater count
                            if (address & 0x0000FF00) == 0 {
                                repeater = true;
                                in_game_cheats[in_game_cheats_idx] = address;
                                in_game_cheats_idx += 1;
                                in_game_cheats[in_game_cheats_idx] = value;
                                in_game_cheats_idx += 1;
                            }
                        }
                        // todo: was fallthrough from default in C, does that even work?
                        0xD0 | // 8-bit Equal-To Conditional
                        0xD1 | // 16-bit Equal-To Conditional
                        0xD2 | // 8-bit Not-Equal-To Conditional
                        0xD3 => { // 16-bit Not-Equal-To Conditional
                            // Validate 16-bit codes
                            if (address & 0x01000001) == 0x01000001 {
                                continue; // todo: or error
                            }

                            in_game_cheats[in_game_cheats_idx] = address;
                            in_game_cheats_idx += 1;
                            in_game_cheats[in_game_cheats_idx] = value;
                            in_game_cheats_idx += 1;
                        }
                        // Everything else
                        _ => {
                            if address != 0
                            {
                                // TODO: Support special code types! :)
                            }
                            // Validate 16-bit codes
                            if (address & 0x01000001) == 0x01000001 {
                                continue; // todo: or error
                            }

                            in_game_cheats[in_game_cheats_idx] = address;
                            in_game_cheats_idx += 1;
                            in_game_cheats[in_game_cheats_idx] = value;
                            in_game_cheats_idx += 1;
                        }
                    }
                }
                _ => return INVALID_CODE_LINE,
            }
        } else {
            //println!("bad line: '{}'", String::from_utf8_lossy(line));
            return INVALID_LINE;
        }
    }
    dbg(103);
    SUCCESS
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

fn hex_to_u32(str: &[u8]) -> u32 {
    use core::ptr::null_mut;
    unsafe { strtoul(str.as_ptr(), null_mut(), 16) }
}

#[no_mangle]
pub extern "C" fn test_rust_print() {
    //unsafe { test_c_print(); }
    //unsafe { screen_text_num(9); } 
    // sent 18446744073709551515 = 4294967195 = FFFF_FF9B
    // sent 0xffff_ffff_ffff_ffff = 4294967295 = FFFF_FFFF
    unsafe { screen_text_num(0xffff_ffff_ffff_ffff); }
    //unsafe { screen_text(b"rust test\0".as_ptr()); }
}

#[no_mangle]
pub extern "C" fn rust_call_test(num: usize) {
    unsafe { screen_text_num(num); }
}

#[no_mangle]
pub extern "C" fn rust_call_test_ptr(num: usize) -> u8 {
    unsafe {
        let cheat_file = num as *mut u8;
        let cheat_file = slice::from_raw_parts_mut(cheat_file, 4);
        cheat_file[1] = b'a';
        //screen_text_num(cheat_file[0].into());
        0
    }
}

fn dbg(num: usize) {
    #[cfg(not(test))]
    unsafe { screen_text_num(num); }
    #[cfg(test)]
    println!("dbg: {}", num);
}

/*
#[no_mangle]
pub extern "C" fn rust_call_test_bla() {
    //rust_call_test(b"c rust test\0".as_ptr());
    rust_call_test([99, 32, 114, 117, 115, 116, 32, 116, 101, 115, 116, 0].as_ptr());
}
*/

extern "C" {
    fn strtoul(s: *const u8, endp: *mut *mut u8, base: i32) -> u32; // todo: is base i32 ?
    fn screen_text(msg: *const u8);
    fn screen_text_num(num: usize);
    fn screen_text_ptr(ptr: u32);
    fn test_c_print();
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_hex_to_u32() {
        println!("i: {:?}", b"c rust test\0");
        assert_eq!(hex_to_u32(b"0x09"), 9);
    }

    #[test]
    fn test_trim_start() {
        let buf = &b"    bla"[..];
        let buf = buf.trim_start(WHITESPACE);
        assert_eq!(buf, b"bla");

        let buf = &b"\n\t\r   \rbla"[..];
        let buf = buf.trim_start(WHITESPACE);
        assert_eq!(buf, b"bla");
    }

    #[test]
    fn test_trim_end() {
        let buf = &b"bla    "[..];
        let buf = buf.trim_end(WHITESPACE);
        assert_eq!(buf, b"bla");

        let buf = &b"bla\n\t\r   \r"[..];
        let buf = buf.trim_end(WHITESPACE);
        assert_eq!(buf, b"bla");
    }

    #[test]
    fn test_trim() {
        let buf = &b"    bla    "[..];
        let buf = buf.trim(WHITESPACE);
        assert_eq!(buf, b"bla");

        let buf = &b"\n\t\r   \rbla\n\t\r   \r"[..];
        let buf = buf.trim(WHITESPACE);
        assert_eq!(buf, b"bla");
    }

    #[test]
    fn test_lines() {
        let lines = &b"\
line1
line2\r
line3
"[..];
        let mut lines = lines.lines(b"\n");
        assert_eq!(lines.next(), Some(&b"line1"[..]));
        assert_eq!(lines.next(), Some(&b"line2\r"[..]));
        assert_eq!(lines.next(), Some(&b"line3"[..]));
        assert_eq!(lines.next(), None);
        assert_eq!(lines.next(), None);

        let lines = &b"F10004E4 2400"[..];
        let mut lines = lines.lines(b" ");
        assert_eq!(lines.next(), Some(&b"F10004E4"[..]));
        assert_eq!(lines.next(), Some(&b"2400"[..]));
        assert_eq!(lines.next(), None);
    }

    #[test]
    fn test_parse_cheats() {
        let cheats_file = &b"\
---

A:
  - F10004E4 2400
  - EE000000 0000

B:
  - 8138EDA0 2400

        "[..];
        let mut boot_cheats = [0u32; 6];
        let mut in_game_cheats = [0u32; 6];
        let ok = parse_cheats(cheats_file, &mut boot_cheats, &mut in_game_cheats);
        assert_eq!(ok, SUCCESS);
        assert_eq!(boot_cheats, [0xF10004E4, 0x2400, 0xEE000000, 0x0000, 0, 0]);
        assert_eq!(in_game_cheats, [0x8138EDA0, 0x2400, 0, 0, 0, 0]);


        let cheats_file = &b"\
---
# Legend of Zelda, The - Ocarina of Time (USA) (Rev 2)

Master Code:
  - F10004E4 2400
  - EE000000 0000

In Health (ASM):
  - 8138EDA0 2400
        "[..];
        let mut boot_cheats = [0u32; 6];
        let mut in_game_cheats = [0u32; 6];
        let ok = parse_cheats(cheats_file, &mut boot_cheats, &mut in_game_cheats);
        assert_eq!(ok, SUCCESS);
        assert_eq!(boot_cheats, [0xF10004E4, 0x2400, 0xEE000000, 0x0000, 0, 0]);
        assert_eq!(in_game_cheats, [0x8138EDA0, 0x2400, 0, 0, 0, 0]);
        
        let cheats_file = &b"wootwootwootwootwootwootwootwoot\0"[..];
        let cheats_file = &b"wootwootwootwootwootwootwootwoot"[..];
        let mut boot_cheats = [0u32; 6];
        let mut in_game_cheats = [0u32; 6];
        let ok = parse_cheats(cheats_file, &mut boot_cheats, &mut in_game_cheats);
        assert_eq!(ok, INVALID_LINE);
        assert_eq!(boot_cheats, [0, 0, 0, 0, 0, 0]);
        assert_eq!(in_game_cheats, [0, 0, 0, 0, 0, 0]);
    }
}
