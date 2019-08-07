fn main() {
    let boiling = 212.0;
    // boiling = to_celcius(boiling); This will fail
    let boiling = to_celcius(boiling);

    println!("212 Farenheight is {} Celcius", boiling);

    let mut freezing = 0.0;
    freezing = to_farenheight(freezing);

    println!("0 Celcius is {} Farenheight", freezing)
}

const OFFSET: f32 = 32.0;

fn to_celcius(temp: f32) -> f32 {
    (temp - OFFSET) * (5.0/9.0)
}

fn to_farenheight(temp: f32) -> f32 {
    (temp * (9.0/5.0)) + OFFSET
}