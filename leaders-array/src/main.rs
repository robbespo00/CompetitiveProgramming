/*
* Write a program to print all the LEADERS in the array. 
* An element is leader if it is greater than all the elements
* to its right side. The rightmost element is always a leader.
*
* INPUT: the first line of input contains an integer T denoting the
* number of test cases. The description of T test cases follows.
* The first line of each test case contains a single integer N denoting
* the size of array. The second line contains N space-separated integers
* A1, A2, ..., A_N denoting the elements of the array.
*
* OUTPUT: print all the leaders
*/


fn main() {
    let mut a: [i32; 6] = [1,2,3,4,8,6];
    

    
    let size = a.len();
    
    leaders_array(size, &mut a);
}


fn leaders_array(size: usize, arr: &mut [i32]){
    
    let mut max = arr[size-1];

    println!("Leader: {}", max);

    for i in (1..size).rev() {
        if max < arr[i] {
            println!("Leader: {}", arr[i]);
            max = arr[i];
        }
    }
}