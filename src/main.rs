// תרגיל 1 – סכום איברים
// פונקציה שמקבלת &Vec<i32> ומחזירה את סכום כל האיברים
fn sum_elements(v: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for i in 0..v.len() {
        sum += v[i];
    }
    sum
}

// תרגיל 2 – מציאת מקסימום
// פונקציה שמחזירה את הערך המקסימלי בווקטור
// דרישות: שימוש באינדקסים, ללא max(), ללא iterator
fn find_maximum(v: &Vec<i32>) -> i32 {
    let mut max = v[0];
    for i in 1..v.len() {
        if v[i] > max {
            max = v[i];
        }
    }
    max
}

fn main() {
    println!("=== תרגיל 1 – סכום איברים ===");
    let numbers1: Vec<i32> = vec![10, 20, 30, 40, 50, 60];
    println!("וקטור: {:?}", numbers1);
    let sum = sum_elements(&numbers1);
    println!("סכום כל האיברים: {}\n", sum);

    println!("=== תרגיל 2 – מציאת מקסימום ===");
    let numbers2: Vec<i32> = vec![45, 12, 89, 23, 56, 78, 34, 91];
    println!("וקטור: {:?}", numbers2);
    let max = find_maximum(&numbers2);
    println!("הערך המקסימלי: {}", max);
}
