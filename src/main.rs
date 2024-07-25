fn mod_exp(base: u128, exp: u128, modulus: u128) -> u128 {
    let mut result = 1;
    let mut base = base % modulus;
    let mut exp = exp;

    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulus;
        }
        exp = exp >> 1;
        base = (base * base) % modulus;
    }

    result
}

fn main() {
    let base1: u128 = 207827272827;
    let exp1: u128 = 6267776557776;
    let base2: u128 = 554647474747;
    let exp2: u128 = 9911111111111;
    let modulus: u128 = 178928928928928;

    let result1 = mod_exp(base1, exp1, modulus);
    let result2 = mod_exp(base2, exp2, modulus);

    let final_result = (result1 + result2) % modulus;

    println!("The result of (20782727282728287373833^626777655777666776 + 5546474747476647447647^9911111111111111113334) % 17892892892892829282 is {}", final_result);
}

