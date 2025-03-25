use actix_web::{web, App, Error, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

fn gcd(a: u32, b: u32) -> u32 {
    if b > 0 {
        gcd(b, a % b)
    } else {
        a
    }
}

#[allow(dead_code)]
fn factorial(n: u128) -> u128 {
    let mut fact = 1;
    for i in 1..=n {
        fact *= dbg!(i);
    }
    fact
}

#[test]
fn test_factorial() {
    assert_eq!(24, factorial(4))
}

#[allow(dead_code)]
fn collatz_sequence(mut n1: u128) -> u128 {
    let mut arr = 1;
    while n1 > 1 {
        n1 = if n1 % 2 == 0 { n1 / 2 } else { 3 * n1 + 1 };
        arr += 1;
    }
    arr
}

#[actix_web::main]
async fn main() -> Result<(), Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_index))
            .route("/", web::post().to(post_calculate))
    });

    println!("The server is Serving on http://localhost:3000 ");

    server.bind("127.0.0.1:3000")?.run().await?;

    Ok(())
}

#[derive(Deserialize)]
struct CalculationParameters {
    a: u32,
    b: Option<u32>,
    operation: String,
}

async fn post_calculate(form: web::Form<CalculationParameters>) -> impl Responder {
    let response = match form.operation.as_str() {
        "gcd" => {
            if form.a == 0 || form.b.unwrap_or(0) == 0 {
                HttpResponse::BadRequest()
                    .content_type("text/html")
                    .body(r#"<h1 style="color: teal;">Error</h1><p>Computing the gcd of zeroes is boring</p>"#)
            } else {
                let result = gcd(form.a, form.b.unwrap());
                HttpResponse::Ok().content_type("text/html").body(format!(
                    r#"<h1 style="color: teal;">GCD Calculation</h1>
                    <p>The Greatest Common Divisor of the numbers {} and {} is <b>{}</b></p>
                    <marquee> This Program Is A Product of Jagoum While Studying Rust </marquee>"#,
                    form.a,
                    form.b.unwrap(),
                    result
                ))
            }
        }
        "factorial" => {
            let result = factorial(form.a as u128);
            HttpResponse::Ok().content_type("text/html").body(format!(
                r#"<h1 style="color: teal;">Factorial Calculation</h1>
                <p>The factorial of {} is <b>{}</b></p>
                <marquee> This Program Is A Product of Jagoum While Studying Rust </marquee>"#,
                form.a, result
            ))
        }
        "collatz" => {
            let result = collatz_sequence(form.a as u128);
            HttpResponse::Ok().content_type("text/html").body(format!(
                r#"<h1 style="color: teal;">Collatz Sequence Calculation</h1>
                <p>The Collatz sequence length for {} is <b>{}</b></p>
                <marquee> This Program Is A Product of Jagoum While Studying Rust </marquee>"#,
                form.a, result
            ))
        }
        _ => HttpResponse::BadRequest()
            .content_type("text/html")
            .body(r#"<h1 style="color: teal;">Error</h1><p>Invalid operation</p>"#),
    };
    response
}

async fn get_index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(
            r#"
            <title>Arithmetic Calculator</title>
            <body bgcolor="lagona">
            <h1 style="color: teal;"><center>Welcome To My Arithmetic Calculator</center></h1>
            <form action="/" method="post">
            <label for="operation" style="color: teal;">Choose an operation:</label>
            <select name="operation" id="operation">
                <option value="gcd">GCD</option>
                <option value="factorial">Factorial</option>
                <option value="collatz">Collatz Sequence</option>
            </select>
            <br>
            <label for="a" style="color: teal;">First Number:</label>
            <input type="number" id="a" name="a" required>
            <br>
            <label for="b" style="color: teal;">Second Number (only for GCD):</label>
            <input type="number" id="b" name="b">
            <br>
            <button type="submit">Compute</button>
            </form>
            <marquee style="color: teal;">This Program Is A Product of Jagoum While Studying Rust</marquee>
            </body>
            "#,
        )
}
