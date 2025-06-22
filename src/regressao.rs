// Funcao para calcular a regressao linear simples
// Formula: y = a * x + b
// Onde:
// - a: inclinacao (coeficiente angular)
// - b: intercepto (coeficiente linear)

pub fn regressao_linear(x: &[f64], y: &[f64]) -> Result<(f64, f64), String> {
    // Verifica se os vetores tem o mesmo tamanho e contem pelo menos dois pontos
    if x.len() != y.len() {
        return Err(String::from("Os vetores x e y devem ter o mesmo tamanho."));
    }
    if x.len() < 2 {
        return Err(String::from("E necessario pelo menos dois pontos para regressao linear."));
    }

    let n = x.len() as f64;

    //Soma de x, y, x*y, e x^2
    let soma_x: f64 = x.iter().sum();
    let soma_y: f64 = y.iter().sum();
    let soma_xy: f64 = x.iter().zip(y).map(|(xi, yi)| xi * yi).sum();
    let soma_x2: f64 = x.iter().map(|xi| xi * xi).sum();

    //Calculo do coeficiente angular (a)
    let numerador_a = n * soma_xy - soma_x * soma_y;
    let denominador_a = n * soma_x2 - soma_x * soma_x;

    if denominador_a == 0.0 {
        return Err(String::from("Divisao por zero ao calcular a inclinacao (a)."));
    }

    let a = numerador_a / denominador_a;

    //Calculo do coeficiente linear (b)
    let media_x = soma_x / n;
    let media_y = soma_y / n;

    let b = media_y - a * media_x;

    Ok((a, b))
}