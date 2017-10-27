pub struct DrawIterator {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    threads: usize,
    thread_number: usize,
}

impl DrawIterator {
    pub fn new(width: usize, height: usize, threads: usize, thread_number: usize) -> DrawIterator {
        DrawIterator {
            x: thread_number,
            y: 0,
            width,
            height,
            threads,
            thread_number,
        }
    }
}

// How to iterate over the image for parallel drawing
impl Iterator for DrawIterator {
    // x, y
    type Item = (usize, usize);

    fn next(&mut self) -> Option<(usize, usize)> {
        self.y += 1;

        if self.y >= self.height {
            self.y = 0;
            self.x += self.threads;
        }

        if self.x < self.width {
            Some((self.x, self.y))
        } else {
            None
        }
    }
}
