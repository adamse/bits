struct BitStream {
    /// current word
    pos: usize,

    /// bits already read in the current word
    read: usize,

    /// bit storage, limited to whole words of bits?
    bits: Vec<u64>,
}

impl BitStream {
    fn new(bits: Vec<u64>) -> Self {
        BitStream {
            pos: 0,
            read: 0,
            bits,
        }
    }

    /// read some bits
    ///
    /// returns the read bits in the lower bits of the little-endian u64
    fn read(&mut self, count: usize) -> u64 {
        // most naive implementation
        assert!(count <= 64);
        assert!(count > 0);

        // bits left to read in current word
        let leftover = 64 - self.read;
        // dbg!(leftover);

        // bits to read from current and next word
        let toread_current = leftover.min(count);
        let toread_next = count - toread_current;
        // dbg!(toread_current);
        // dbg!(toread_next);

        // mask and shift for bits in the current word
        let mask = if toread_current == 64 { !0 } else { (1 << toread_current) - 1 };
        let shift = self.read;

        let part1 = (self.bits[self.pos] >> shift) & mask;

        // mask and shift for bits in the next word
        let mask = (1 << toread_next) - 1;
        let shift = toread_current;
        let part2 = if toread_next > 0 { (self.bits[self.pos + 1] & mask) << shift } else { 0 };

        self.read = (self.read + toread_current) % 64 + toread_next;
        self.pos = if toread_current >= leftover { self.pos + 1 } else { self.pos };

        return part1 | part2;
    }
}

fn main() {
    let word = 0x0807060504030201u64;
    let bits = vec![word; 1];
    let mut bits = BitStream::new(bits);

    // read all 64 bits
    assert!(bits.read(64) == word);
    assert!(bits.read == 0);
    assert!(bits.pos == 1);

    let bits = vec![word; 2];
    let mut bits = BitStream::new(bits);

    // read all half a word
    assert!(bits.read(32) == 0x04030201);
    assert!(bits.read == 32);
    assert!(bits.pos == 0);

    // read a full word
    assert!(bits.read(64) == 0x0403020108070605);
    assert!(bits.read == 32);
    assert!(bits.pos == 1);

    // read a byte
    assert!(bits.read(8) == 0x05);
    assert!(bits.read == 40);
    assert!(bits.pos == 1);

    // read 2 bits, should be first bits of 0x6 which is 0b10
    assert!(bits.read(2) == 0b10);
    assert!(bits.read == 42);
    assert!(bits.pos == 1);

    // read 2 bits, should be last bits of 0x6 which is 0b01
    assert!(bits.read(2) == 0b01);
    assert!(bits.read == 44);
    assert!(bits.pos == 1);

    // read 4 bits, should be 0
    assert!(bits.read(4) == 0);
    assert!(bits.read == 48);
    assert!(bits.pos == 1);

    // read the rest
    assert!(bits.read(16) == 0x0807);
    assert!(bits.read == 0);
    assert!(bits.pos == 2);
}
