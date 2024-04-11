use std::arch::asm;

macro_rules! label {
    ($label:expr) => {
        #[allow(named_asm_labels)]
        unsafe {
            asm!(concat!($label, ":"));
        }
    };
}

macro_rules! goto {
    ($label:expr) => {
        unsafe {
            asm!(concat!("jmp ", $label));
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loop_test() {
        let mut i = 0;
        label!("here");
        i += 1;
        if i < 10 {
            goto!("here");
        }
        assert_eq!(i, 10);
    }
}
