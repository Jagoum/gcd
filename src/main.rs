use actix_web::{web, App, Error, HttpResponse, HttpServer};
use serde::Deserialize;

fn gcd(a: u32, b: u32) -> u32 {
    if b > 0 {
        gcd(b, a % b)
    } else {
        a
    }
    // while b > 0 {
    //     a = b;
    //     b = a % b;
    // }
    // a
}

#[allow(dead_code)]
/// This function is to calculate the factorial of  a number
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
fn fizzbuzz(a: u32) -> u32 {
    let _ = a;
    a
}

#[allow(dead_code)]
///The Collatz Sequence is defined as follows, for an arbitrary n1 greater than zero:
///- If ni is 1, then the sequence terminates at ni.
///- If ni is even, then ni+1 = ni / 2.
///- If ni is odd, then ni+1 = 3 * ni + 1.
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
        .route("/", web::post().to(post_gcd))
    });

    println!("The server is Serving on http://localhost:3000 ");
    
    server.bind("127.0.0.1:3000")?
    .run()
    .await?;

    Ok(())

}
#[derive(Deserialize)]
struct GcdParameters {
    a: u32,
    b: u32,
}
async fn post_gcd(form: web::Form<GcdParameters>) -> HttpResponse {
    if form.a == 0 || form.b == 0 {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("<h1>Computing the gcd of zeroes is boring</h1>");
    }
    let response = format!(
        r#" <div class="div"> <pre><h1><center><font color="olive"> Amazing !!!</font></center></h1>
    <center><h4> The Greatest Common Divisor of the numbers {} and {} is <font color="red"><b>{}</b></font> </h4> </center>
    <marquee><font color="blue"> This Program Is A Product of Jagoum While Studying Rust </font></marquee>
    </pre>
    </div>
    \n"#,
        form.a,
        form.b,
        gcd(form.a, form.b)
    );
    HttpResponse::Ok().content_type("text/html").body(response)
}

async fn get_index() -> HttpResponse {
HttpResponse::Ok()
.content_type("text/html")
.body(
r#"
<title>GCD Calculator</title>
<body bgcolor = "lagona">
<font color="teal"><h1><center> Welcome To My GCD Calculator </center></h1> </font> 
<form action="/" method="post">
<font color="sky-blue"> First Number: <input type="number" name="a"/> </font>
<font color="sky-blue"> Second Number: <input type="number" name="b"/> </font>
<button type="submit">Compute GCD</button>
</form>
</body>
"#,
)
}
