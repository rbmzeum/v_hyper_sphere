use num_traits::Float;
use std::f64::consts::PI;

/// Вычисляет объём n-мерной гиперсферы
fn hyper_sphere_volume<T>(radius: T, dimension: usize) -> T
where
    T: Float,
{
    let pi = T::from(PI).unwrap(); // Преобразуем PI в тип T
    let radius_pow_n = radius.powi(dimension as i32); // r^n
    let gamma_value = gamma_half_integer(T::from(dimension as f64).unwrap()); // Гамма-функция от (n/2 + 1)

    // Общая формула: V_n = (π^(n/2) / Γ(n/2 + 1)) * r^n
    (pi.powf(T::from(dimension as f64).unwrap() / T::from(2.0).unwrap()) / gamma_value) * radius_pow_n
}

/// Вычисляет гамма-функцию для полуцелых значений (n/2 + 1)
fn gamma_half_integer<T>(x: T) -> T
where
    T: Float,
{
    let n = x.to_usize().unwrap(); // Преобразуем x в usize (размерность)
    if n % 2 == 0 {
        // Для чётных n: Γ(n/2 + 1) = (n/2)!
        factorial(T::from(n / 2).unwrap())
    } else {
        // Для нечётных n: Γ(n/2 + 1) = (n)!! / 2^{(n+1)/2} * sqrt(π)
        let double_factorial = double_factorial(T::from(n).unwrap());
        let sqrt_pi = T::from(PI).unwrap().sqrt();
        double_factorial / T::from(2.0).unwrap().powf(T::from(n + 1).unwrap() / T::from(2.0).unwrap()) * sqrt_pi
    }
}

/// Вычисляет факториал числа
fn factorial<T>(n: T) -> T
where
    T: Float,
{
    let mut result = T::one();
    for i in 1..=n.to_usize().unwrap() {
        result = result * T::from(i).unwrap();
    }
    result
}

/// Вычисляет двойной факториал числа
fn double_factorial<T>(n: T) -> T
where
    T: Float,
{
    let mut result = T::one();
    let mut i = n.to_usize().unwrap();
    while i >= 1 {
        result = result * T::from(i).unwrap();
        i = i.saturating_sub(2); // Уменьшаем на 2
    }
    result
}

fn main() {
    let radius = 1.0; // Фиксированный радиус
    println!("Объём n-мерной гиперсферы с радиусом r:");
    println!("{:<10} {:<10} {:<20}", "Dimension", "Radius", "Volume");
    println!("{}", "-".repeat(40));
    for dimension in 1..=22 {
        let volume = hyper_sphere_volume(radius, dimension);
        println!("{:<10} {:<10} {:<20.6}", dimension, radius, volume);
    }
}

