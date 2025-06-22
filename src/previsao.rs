// Funções de previsão com base na regressão linear
// Retorna a previsão de y dado um x, com base nos coeficientes da regressão
pub fn prever_valor(x: f64, a: f64, b: f64) -> f64 {
    a * x + b
}

// Gera previsões para os próximos `n` períodos a partir do último x
///
// # Argumentos:
// - `a`, `b`: coeficientes da regressão linear
// - `ultimo_x`: último valor da sequência conhecida (ex: último dia)
// - `quantidade`: quantos períodos futuros você quer prever
//
// # Retorno:
// - Vetor com os valores previstos
pub fn prever_multiplos(a: f64, b: f64, ultimo_x: f64, quantidade: usize) -> Vec<f64> {
    (1..=quantidade)
        .map(|i| {
            let proximo_x = ultimo_x + i as f64;
            prever_valor(proximo_x, a, b)
        })
        .collect()
}
