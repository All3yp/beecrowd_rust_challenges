pub fn salary_increase(salary: f64) -> f64 {
    if salary <= 400.00 {
        return salary * 1.15;
    }
    else if salary >= 400.01 && salary <= 800.00 {
        return salary * 1.12;
    }
    else if salary >= 800.01 && salary <= 1200.00 {
        return salary * 1.10;
    }
    else if salary >= 1200.01 && salary <= 2000.00 {
        return salary * 1.07;
    }
    else {
        return salary * 1.04;
    }
}
