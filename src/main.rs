mod regressao;
mod metricas;
mod previsao;

use regressao::regressao_linear;
use metricas::{calcular_r2, calcular_mse};
use previsao::prever_multiplos;

fn main() {
    println!("🚀 Projeto TimeWise Analytics iniciado com sucesso!");

    let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let y = vec![2.0, 4.0, 6.0, 8.0, 10.0];

    match regressao_linear(&x, &y) {
        Ok((a, b)) => {
            println!("Coeficientes: a = {:.2}, b = {:.2}", a, b);

            let y_predito: Vec<f64> = x.iter().map(|xi| a * xi + b).collect();

            if let Ok(r2) = calcular_r2(&y, &y_predito) {
                println!("R² = {:.4}", r2);
            }

            if let Ok(mse) = calcular_mse(&y, &y_predito) {
                println!("MSE = {:.4}", mse);
            }

            let previsoes = prever_multiplos(a, b, 5.0, 3);
            println!("Próximas previsões: {:?}", previsoes);
        }
        Err(e) => {
            println!("Erro ao calcular regressão: {}", e);
        }
    }
}
