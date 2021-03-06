//Dado uma lista contendo n+1 inteiros onde cada inteiro está entre 1 e n (incluso), prove que pelo menos um número duplicado deve existir. 
//Assumindo que há apenas um número duplicado, encontre-o.

fn main() {
    let n: Vec<u32> = vec!(1, 2, 2);
    println!("duplicated: {}", find_duplicated(n));
}

fn find_duplicated(nums: Vec<u32>) -> u32 {
    let mut hare: u32 = nums[0];
    let mut tortoise: u32 = nums[0];

    loop {        
        tortoise = nums[tortoise as usize];
        hare = nums[nums[hare as usize] as usize];
        if tortoise == hare {
            break;
        }
    }

    let mut ptr1 = nums[0];
    let mut ptr2 = tortoise;
    while ptr1 != ptr2 {
        ptr1 = nums[ptr1 as usize];
        ptr2 = nums[ptr2 as usize];
    }

    return ptr1;
}
