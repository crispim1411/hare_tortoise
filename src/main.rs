//Dado uma lista contendo n+1 inteiros onde cada inteiro está entre 1 e n (incluso), prove que pelo menos um número duplicado deve existir. 
//Assumindo que há apenas um número duplicado, encontre-o.

use rand::Rng;
const N: u32 = 7;

fn main() {
    let mut n: Vec<u32> = (1..N).collect();
    let rand = rand::thread_rng().gen_range(1..=N);
    n.push(rand);
    println!("List: {:?}", n);
    println!("duplicated: {}", find_duplicated(n));
}

fn find_duplicated(nums: Vec<u32>) -> u32 {
    let nums: Vec<usize> = nums
                            .iter()
                            .map(|&x| x as usize)
                            .collect();
                            
    let mut hare = nums[0];
    let mut tortoise = nums[0];
    
    loop {        
        tortoise = nums[tortoise];
        hare = nums[nums[hare]];
        if tortoise == hare {
            break;
        }
    }

    let mut ptr1 = nums[0];
    let mut ptr2 = tortoise;
    while ptr1 != ptr2 {
        ptr1 = nums[ptr1];
        ptr2 = nums[ptr2];
    }

    return ptr1 as u32;
}
