fn bubble_sort_base(arr: &mut [i32]) {
    let n = arr.len();
    for i in 0..n {
        for j in 0..n-i-1 {
            if arr[j] > arr[j+1] {
                arr.swap(j, j+1);
            }
        }
    }
}

fn bubble_sort<T: PartialOrd>(arr: &mut [T]) {
    let n = arr.len();
    for i in 0..n {
        for j in 0..n-i-1 {
            if arr[j] > arr[j+1] {
                arr.swap(j, j+1);
            }
        }
    }
}

fn main() {
    let mut arr0 = [64, 34, 25, 12, 25, 22, 11, 90];
    bubble_sort_base(&mut arr0);
    println!("排序后的数组0: {:?}", arr0);

    let mut arr1 = [64, 34, 25, 12, 22, 11, 90];
    bubble_sort(&mut arr1);
    println!("排序后的数组1: {:?}", arr1);

    let mut arr2 = ['c', 'a', 'd', 'f', 'e', 'b'];
    bubble_sort(&mut arr2);
    println!("排序后的数组2: {:?}", arr2);
}
