fn fibnocci(n: u32) -> i32 {
	if n == 0 || n == 1 {
		return 1;
	}

	(fibnocci(n - 1) + fibnocci(n - 2)) as i32
}

fn fahrenheit_to_centigrade (t: f32) -> f32 {
	(t - 32.) * 5. / 9.
}

fn centigrade_to_fahrenheit (t: f32) -> f32 {
	t * 9. / 5. + 32.
}

fn main() {
    println!("{}", fibnocci(10));
    println!("{}", centigrade_to_fahrenheit(5.));
    println!("{}", fahrenheit_to_centigrade(41.));
}
