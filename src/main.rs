fn main() {
    let s = "
0 crw-r-----.  1 root   usbmon  243,     2 Jan 31 18:40 usbmon2
0 crw-------.  1 root   root     10,   126 Jan 31 18:40 userfaultfd
0 drwxr-xr-x.  4 root   root            80 Jan 31 18:40 v4l
0 crw-rw----.  1 root   tty       7,     0 Jan 31 18:40 vcs
0 crw-rw----.  1 root   tty       7,     1 Jan 31 18:40 vcs1
0 crw-rw----.  1 root   tty       7,     2 Jan 31 18:40 vcs2
0 crw-rw----.  1 root   tty       7,     3 Jan 31 18:40 vcs3
0 crw-rw----.  1 root   tty       7,     4 Jan 31 18:40 vcs4
0 crw-rw----.  1 root   tty       7,     5 Jan 31 18:40 vcs5
0 crw-rw----.  1 root   tty       7,     6 Jan 31 18:40 vcs6
0 crw-rw----.  1 root   tty       7,    63 Jan 31 19:07 vcs63
0 crw-rw----.  1 root   tty       7,   128 Jan 31 18:40 vcsa
0 crw-rw----.  1 root   tty       7,   129 Jan 31 18:40 vcsa1
0 crw-rw----.  1 root   tty       7,   130 Jan 31 18:40 vcsa2
0 crw-rw----.  1 root   tty       7,   131 Jan 31 18:40 vcsa3
0 crw-rw----.  1 root   tty       7,   132 Jan 31 18:40 vcsa4
0 crw-rw----.  1 root   tty       7,   133 Jan 31 18:40 vcsa5
0 crw-rw----.  1 root   tty       7,   134 Jan 31 18:40 vcsa6
0 crw-rw----.  1 root   tty       7,   191 Jan 31 19:07 vcsa63
0 crw-rw----.  1 root   tty       7,    64 Jan 31 18:40 vcsu
0 crw-rw----.  1 root   tty       7,    65 Jan 31 18:40 vcsu1
0 crw-rw----.  1 root   tty       7,    66 Jan 31 18:40 vcsu2
0 crw-rw----.  1 root   tty       7,    67 Jan 31 18:40 vcsu3
0 crw-rw----.  1 root   tty       7,    68 Jan 31 18:40 vcsu4
0 crw-rw----.  1 root   tty       7,    69 Jan 31 18:40 vcsu5
0 crw-rw----.  1 root   tty       7,    70 Jan 31 18:40 vcsu6
0 crw-rw----.  1 root   tty       7,   127 Jan 31 19:07 vcsu63
0 drwxr-xr-x.  2 root   root            60 Jan 31 18:40 vfio
0 crw-------.  1 root   root     10,   127 Jan 31 18:40 vga_arbiter
0 crw-------.  1 root   root     10,   137 Jan 31 18:40 vhci
0 crw-rw-rw-.  1 root   kvm      10,   238 Jan 31 18:40 vhost-net
0 crw-rw-rw-.  1 root   kvm      10,   241 Jan 31 18:40 vhost-vsock
0 crw-rw----+  1 root   video    81,     0 Jan 31 18:40 video0
0 crw-rw----+  1 root   video    81,     1 Jan 31 18:40 video1
0 crw-------.  1 root   root     10,   130 Jan 31 18:40 watchdog
0 crw-------.  1 root   root    246,     0 Jan 31 18:40 watchdog0
0 crw-rw-rw-.  1 root   root      1,     5 Jan 31 18:40 zero
0 brw-rw----.  1 root   disk    252,     0 Feb  1 18:46 zram0
";
    println!("{}", s.chars().count());

    bash_big_msg(String::from(s));
}

fn bash_big_msg(out: String) {
    let mut newstr: Vec<String> = Vec::new();
    newstr.push(String::new());

    let mut tempstr = String::new();

    for x in out.split('\n') {
        if tempstr.clone().chars().count() + x.chars().count() <= 2000 {
            tempstr += &format!("\n{x}")
        } else {
            newstr.push(tempstr.clone());
            tempstr = String::new();
        }
    }

    newstr.push(tempstr);

    for x in newstr.iter() {
        println!("START\n{}", x);
    }
}
