# 📊 TimeWise Analytics

**TimeWise Analytics** é um projeto em Rust que implementa uma solução pura de regressão linear para análise e previsão de séries temporais. Este projeto foi desenvolvido como estudo de caso acadêmico com foco em desempenho, legibilidade, testes automatizados e clareza matemática.

---

## 🔍 Objetivo

Desenvolver um sistema capaz de:

- 📥 Importar e processar dados de séries temporais (simulados manualmente)
- 📈 Aplicar **Regressão Linear Simples**
- 🧮 Calcular métricas de avaliação: **R² (coeficiente de determinação)** e **MSE (erro quadrático médio)**
- 📅 Realizar previsões futuras com base na linha de tendência
- ✅ Cobrir com testes automatizados

---

## 🛠️ Estrutura do Projeto

```plaintext
timewise_analytics/
├── src/
│   ├── main.rs          # Ponto de entrada da aplicação
│   ├── lib.rs           # Declaração dos módulos
│   ├── regressao.rs     # Regressão linear pura
│   ├── metricas.rs      # Cálculo de R² e MSE
│   └── previsao.rs      # Geração de previsões futuras
├── tests/
│   └── main_tests.rs    # Testes unitários
└── README.md            # Documentação do projeto

## 🚀 Como Executar
 
-> 1.Clone o repositorio:

git clone https://github.com/Dieggo775/timewise_analytics.git
cd timewise_analytics

-> Copile o projeto

cargo run

## Como executar os teste

Os testes automatizados validam:
    A implementa;'ao da regressao linear
    O calculo das metricas R² e MSE
    As previsoes baseadas nos coeficientes

Para rodar o testes:
    cargo test

👨‍💻 Autor
Desenvolvido por Diego Araujo
Aluno de Análise e Desenvolvimento de Sistemas - 2º Ano
Projeto acadêmico para a disciplina de Analise e Desenvolvimento de Sistemas
Instituição: Fecaf
