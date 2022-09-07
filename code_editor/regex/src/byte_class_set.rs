use crate::Range;

#[derive(Clone, Debug)]
pub(crate) struct ByteClassSet(Box<[u8]>);

impl ByteClassSet {
    pub(crate) fn len(&self) -> u16 {
        self.0[255] as u16 + 2
    }

    pub(crate) fn get(&self, byte: u8) -> u8 {
        self.0[byte as usize] as u8
    }
}

#[derive(Debug)]
pub(crate) struct Builder([bool; 256]);

impl Builder {
    pub(crate) fn new() -> Self {
        Self([false; 256])
    }

    pub(crate) fn build(self) -> ByteClassSet {
        let mut byte_classes = vec![0; 256];
        let mut byte_class = 0u8;
        let mut index = 0;
        loop {
            byte_classes[index] = byte_class as u8;
            if index == 255 {
                break;
            }
            if self.0[index] {
                byte_class += 1;
            }
            index += 1;
        }
        ByteClassSet(byte_classes.into_boxed_slice())
    }

    pub(crate) fn insert(&mut self, byte_range: Range<u8>) {
        if byte_range.start > 0 {
            self.0[byte_range.start as usize - 1] = true;
        }
        self.0[byte_range.end as usize] = true;
    }
}
