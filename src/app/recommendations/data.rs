use serde::Deserialize;

use getset::Getters;
use yew::html::ImplicitClone;

#[derive(Clone, Debug, Deserialize, Getters, PartialEq, Default)]
#[getset(get="pub")]
pub struct Recommendations{
    #[serde(alias="potentialBuy")]
    potential_buys: Option<Vec<PotentialBuy>>,
    #[serde(alias="reducePriceBuy")]
    reduce_cost_buys: Option<Vec<ReduceCostBuy>>,
    #[serde(alias="potentialSell")]
    potential_sells: Option<Vec<PotentialSell>>,
    #[serde(alias="sellWithStop")]
    stop_sells: Option<Vec<SellWithStop>>,
    #[serde(alias="notEnoughData")]
    unknown: Option<Vec<Unknown>>,
}

impl ImplicitClone for Recommendations{

}

#[derive(Clone, Debug, Deserialize, Getters, PartialEq)]
#[getset(get="pub")]
pub struct PotentialBuy {
    token: String,
    #[serde(alias="currentAmount")]
    current_amount: f64,
    #[serde(alias="currentPrice")]
    current_price: f64,
    #[serde(alias="historicData")]
    historic_data: HistoricData,
    loss: f64,
}

#[derive(Clone, Debug, Deserialize, Getters, PartialEq)]
#[getset(get="pub")]
pub struct ReduceCostBuy{
    token: String,
    #[serde(alias="currentAmount")]
    current_amount: f64,
    #[serde(alias="currentPrice")]
    current_price: f64,
    #[serde(alias="historicData")]
    historic_data: HistoricData,
    #[serde(alias="amount")]
    amount_to_buy: f64,
    #[serde(alias="totalCost")]
    cost: f64,
}

#[derive(Clone, Debug, Deserialize, Getters, PartialEq)]
#[getset(get="pub")]
pub struct PotentialSell{
    token: String,
    #[serde(alias="currentAmount")]
    current_amount: f64,
    #[serde(alias="currentPrice")]
    current_price: f64,
    #[serde(alias="historicData")]
    historic_data: HistoricData,
    gain: f64
}

#[derive(Clone, Debug, Deserialize, Getters, PartialEq)]
#[getset(get="pub")]
pub struct SellWithStop{
    token: String,
    #[serde(alias="currentAmount")]
    current_amount: f64,
    #[serde(alias="currentPrice")]
    current_price: f64,
    #[serde(alias="historicData")]
    historic_data: HistoricData,
    #[serde(alias="stopPrice")]
    stop_price: f64,
    #[serde(alias="gainOnStop")]
    gain_on_stop: f64,
}
#[derive(Clone, Debug, Deserialize, Getters, PartialEq)]
#[getset(get="pub")]
pub struct Unknown{
    token: String,
    #[serde(alias="currentAmount")]
    current_amount: Option<f64>,
    #[serde(alias="currentPrice")]
    current_price: Option<f64>,
    #[serde(alias="historicData")]
    historic_data: Option<HistoricData>,
}

#[derive(Clone, Debug, Deserialize, Getters, PartialEq)]
#[getset(get="pub")]
pub struct HistoricData {
    price: f64,
    amount: f64,
    total: f64,
}
