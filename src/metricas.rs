// Métricas de avaliação (R², MSE)
// Calcula o Coeficiente de Determinacao (R²)
// R² = 1 - (SSE / SST)
// SST: Soma total dos quadrados (variacao dos dados em relacao a media)

pub fn calcular_r2(y_real: &[f64], y_predito: &[f64]) -> Result<f64, String> {
    if y_real.len() != y_predito.len() || y_real.is_empty() {
        return Err(String::from("Os vetores devem ter o mesmo tamanho e nao podem estar vazios"));
    }

    let media_y = y_real.iter().sum::<f64>() / y_real.len() as f64;

    let sse: f64 = y_real.iter()
        .zip(y_predito)
        .map(|(y, y_hat)| (y - y_hat).powi(2))
        .sum();

    let sst: f64 = y_real.iter()
        .map(|y| (y - media_y).powi(2))
        .sum();

    if sst == 0.0 {
        return Err(String::from("A variancia dos dados e zero. R² indefinido."));
    }

    Ok(1.0 - (sse / sst))
}

// Calcula o Erro Quadratico Medio (MSE)
// MSE = media dos quadrados dos erros (y_real - y_predito)
pub fn calcular_mse(y_real: &[f64], y_predito: &[f64]) -> Result<f64, String> {
    if y_real.len() != y_predito.len() || y_real.is_empty() {
        return Err(String::from("Os vetores devem ter o mesmo tamanho e nao podem estar vazios."));
    }

    let mse: f64 = y_real.iter()
        .zip(y_predito)
        .map(|(y, y_hat)| (y - y_hat).powi(2))
        .sum::<f64>() / y_real.len() as f64;

    Ok(mse)
}