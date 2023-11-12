fn write_hello(buf: &mut [u8]) {
    buf[0] = b'H';
    buf[1] = b'e';
    buf[2] = b'l';
    buf[3] = b'l';
    buf[4] = b'o';
}