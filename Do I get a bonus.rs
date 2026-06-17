fn bonus_time(salary: u64, bonus: bool) -> String {
    let mut new_salary: u64 = salary;
    if bonus {
        new_salary *= 10;
    }
    format!("¥{}", new_salary)
}