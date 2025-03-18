// src/lib.rs

pub unsafe fn multiply_array(ptr: *const i32, len: usize) -> i32 {
    let mut product = 1;
    for i in 0..len { // Troca de 1 para 0, desta forma a multiplicação ocorre de forma correta.
        product *= *ptr.offset(i as isize);
    }
    product
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiply_array() {
        let arr = [2, 3, 4];
        let product = unsafe { multiply_array(arr.as_ptr(), arr.len()) };
        assert_eq!(product, 24);
    }
}
 
fn main() { //Por se tratar de uma função Main ela gera um "Warning" no test
    println!("Hello, world!");
}