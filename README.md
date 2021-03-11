# Linear Regression

#### Goals:
* Pour ce projet vous devrez créer un programme qui predit le prix d une voiture en utilisant l’entrainement par fonction linéaire avec un algorithme du gradient.
* Toute fonction que ferait le travail à votre place est interdie (predict, calcul de thetas, ...), vous devrez tout recoder vous même.

Regarder [subject.pdf](fr.subject.pdf) pour plus d'informations.

## Requirements:
* `rust`
* `cargo`
* `rust-ini = "0.16.1"`
* `colored = "2"`
* `charts = "0.3.0"`
* `csv = "1.1.5"`
* `serde = { version = "1", features = ["derive"] }`

## Setup:
```
git clone https://github.com/LudovicLemaire/42-linear-regression.git
cd 42-linear-regression
cargo build
```

## Show datas:
```
cargon run --bin show < ./data/SacramentocrimeJanuary2006.csv
```
|Show datas on a graph                                                |
|---------------------------------------------------------------------|
|![chart](charts/chart.svg)                                 |
|That is all Sacramento crimes in January 2006, with position long/lat|


## Training phase:
```
cargo run --bin train < ./data/data.csv
```
|Learning curve                                          |Training curve                                          |MSE                               |
|--------------------------------------------------------|--------------------------------------------------------|----------------------------------|
|![chart_learning_curve](charts/chart_learning_curve.svg)|![chart_multiple_curve](charts/chart_multiple_curve.svg)|![chart_mse](charts/chart_mse.svg)|


## Predict phase:
```
cargon run --bin predict
```
|Linear Regression                                       |
|--------------------------------------------------------|
|![chat_final](charts/chat_final.svg)                    |