use timewise_analytics::regressao::regressao_linear;
use timewise_analytics::metricas::{calcular_r2, calcular_mse};

#[test]
fn test_regressao_linear() {
    let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let y = vec![2.0, 4.0, 6.0, 8.0, 10.0];

    let resultado = regressao_linear(&x, &y);
    assert!(resultado.is_ok());

    let (a, b) = resultado.unwrap();
    assert!((a - 2.0).abs() < 1e-6); //Inclinacao
    assert!((b - 0.0).abs() < 1e-6); //Intercepto
}

#[test]
fn test_calcular_r2() {
    let y_real = vec![2.0, 4.0, 6.0, 8.0, 10.0];
    let y_predito = vec![2.1, 4.0, 5.9, 8.0, 10.1];

    let resultado = calcular_r2(&y_real, &y_predito);
    assert!(resultado.is_ok());

    let r2 = resultado.unwrap();
    assert!(r2 >= 0.99); //R² deve estar muito proximo de 1
}

#[test]
fn test_calcular_mse() {
    let y_real = vec![2.0, 4.0, 6.0, 8.0, 10.0];
    let y_predito = vec![2.1, 4.0, 5.9, 8.0, 10.1];

    let resultado = calcular_mse(&y_real, &y_predito);
    assert!(resultado.is_ok());

    let mse = resultado.unwrap();
    assert!(mse < 0.02); // Erro quadratico medio pequeno
}

use timewise_analytics::previsao::{prever_valor, prever_multiplos};

#[test]
fn test_prever_valor() {
    let a = 2.0;
    let b = 1.0;
    let x = 4.0;
    let esperado = 9.0;

    let y = prever_valor(x, a, b);
    assert!((y - esperado).abs() < 1e-6);
}

#[test]
fn test_prever_multiplos() {
    let a = 2.0;
    let b = 1.0;
    let ultimo_x = 5.0;
    let previsoes = prever_multiplos(a, b, ultimo_x, 3);

    let esperado = vec![13.0, 15.0, 17.0];
    for (y, y_esperado) in previsoes.iter().zip(esperado.iter()) {
        assert!((*y - *y_esperado).abs() < 1e-6);
    }
}
