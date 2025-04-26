fn remove_duplicates(nums: &mut Vec<i32>) -> usize {
    if nums.is_empty() {
        return 0;
    }
    let mut slow = 0; // Pointer for the unique elements
    for fast in 1..nums.len() {
        // Fast pointer to iterate through the array
        println!(
            "Looping: Slow at {} - Fast at {} = Array {:?}",
            slow, fast, &nums
        );
        if nums[fast] != nums[slow] {
            slow += 1;
            println!(
                "Modifying: Slow nums[{}] Fast nums[{}] = {}",
                slow, fast, nums[fast]
            );
            nums[slow] = nums[fast];
            println!(
                "Modified: Slow at {} - Fast at {} = Array {:?}",
                slow, fast, &nums
            );
        }
    }
    slow + 1 // The number of unique elements
}

fn main() {
    let mut numbers = vec![1, 1, 2, 2, 3, 4, 4, 4, 5];
    let unique_count = remove_duplicates(&mut numbers);
    println!("Unique elements count: {}", unique_count);
    println!("Modified vector: {:?}", &numbers[..unique_count]);
}
