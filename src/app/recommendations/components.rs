use yew::{Properties, Html, function_component, html};

use super::data::{Recommendations, PotentialBuy, ReduceCostBuy, SellWithStop, PotentialSell, Unknown};

#[derive(Properties, PartialEq)]
pub struct RecommendationProps {
    pub recommendations: Recommendations
}

#[derive(Properties, PartialEq)]
pub struct PotentialBuyProps {
    pub potential_buys: Vec<PotentialBuy>
}

#[derive(Properties, PartialEq)]
pub struct ReduceCostBuyProps {
    pub reduce_cost_buys: Vec<ReduceCostBuy>
}

#[derive(Properties, PartialEq)]
pub struct PotentialSellProps {
    pub potential_sells: Vec<PotentialSell>
}

#[derive(Properties, PartialEq)]
pub struct SellWithStopProps {
    pub sells_with_stop: Vec<SellWithStop>
}

#[derive(Properties, PartialEq)]
pub struct UnknownProps {
    pub unknowns: Vec<Unknown>
}

// [serde(alias="potentialSell")]
//     potential_sells: Option<Vec<PotentialSell>>,
//     #[serde(alias="sellWithStop")]
//     stop_sells: Option<Vec<SellWithStop>>,
//     #[serde(alias="notEnoughData")]
//     unknown: Option<Vec<Unknown>>,

#[function_component(RecommendationsComponent)]
pub fn recommendations(RecommendationProps{recommendations}: &RecommendationProps) -> Html {
    let potential_buy_rows = recommendations.potential_buys().clone().map(|entries| html! {
        <PotentialBuyRowsComponent potential_buys={entries}/>
    });

    let reduce_cost_buy_rows = recommendations.reduce_cost_buys().clone().map(|entries| html! {
        <ReduceCostBuyRowsComponent reduce_cost_buys={entries}/>
    });

    let potential_sell_rows = recommendations.potential_sells().clone().map(|entries| html! {
        <PotentialSellRowsComponent potential_sells={entries}/>
    });

    let sell_with_stop_rows = recommendations.stop_sells().clone().map(|entries| html! {
        <SellWithStopRowsComponent sells_with_stop={entries}/>
    });

    let unknown_rows = recommendations.unknown().clone().map(|entries| html! {
        <UnknownRowsComponent unknowns={entries}/>
    });

    html! {
        <>
        <div class="container">
        // <div>
            <h3> {"Potential Loss"}</h3>
            // <table>
            <table class="table table-danger table-striped table-hover">
                <thead>
                <tr>
                    <th>{"Token"}</th>
                    <th>{"Current Amount"}</th>
                    <th>{"Current Price"}</th>
                    <th>{"Historic Price"}</th>
                    <th>{"Loss"}</th>
                </tr>
                </thead>
                <tbody>
                {for potential_buy_rows}
                </tbody>
            </table>
        </div>

        <div class="container">
        // <div>
            <h3> {"Reduce token cost by 10%"}</h3>
            // <table>
            <table class="table table-danger table-striped table-hover">
                <thead>
                <tr>
                    <th>{"Token"}</th>
                    <th>{"Current Amount"}</th>
                    <th>{"Current Price"}</th>
                    <th>{"Historic Price"}</th>
                    <th>{"Amount To Buy"}</th>
                    <th>{"Cost"}</th>
                </tr>
                </thead>
                <tbody>
                {for reduce_cost_buy_rows}
                </tbody>
            </table>
        </div>

        <div>
            <h3> {"Potential Gain"}</h3>
            // <table>
            <table class="table table-success table-striped table-striped">
                <thead>
                <tr>
                    <th>{"Token"}</th>
                    <th>{"Current Amount"}</th>
                    <th>{"Current Price"}</th>
                    <th>{"Historic Price"}</th>
                    <th>{"Gain"}</th>
                </tr>
                </thead>
                <tbody>
                {for potential_sell_rows}
                </tbody>
            </table>
        </div>

        <div>
            <h3> {"Stop Loss (-5%) Gain"}</h3>
            // <table>
            <table class="table table-success table-striped table-hover">
                <thead>
                <tr>
                    <th>{"Token"}</th>
                    <th>{"Current Amount"}</th>
                    <th>{"Current Price"}</th>
                    <th>{"Historic Price"}</th>
                    <th>{"Stop Price"}</th>
                    <th>{"Gain On Stop"}</th>
                </tr>
                </thead>
                <tbody>
                {for sell_with_stop_rows}
                </tbody>
            </table>
        </div>

        <div>
            <h3> {"Not enough data for recommendation"}</h3>
            // <table>
            <table class="table table-warning table-striped table-hover">
                <thead>
                <tr>
                    <th>{"Token"}</th>
                    <th>{"Current Amount"}</th>
                    <th>{"Current Price"}</th>
                    <th>{"Historic Price"}</th>
                </tr>
                </thead>
                <tbody>
                {for unknown_rows}
                </tbody>
            </table>
        </div>

        </>
    }

}

#[function_component(PotentialBuyRowsComponent)]
pub fn potential_buy_row(PotentialBuyProps{potential_buys}: &PotentialBuyProps) -> Html {
    potential_buys.iter().map(|potential_buy|{
        html! {
            <tr key={potential_buy.token().as_str()}>
                <td>{potential_buy.token()}</td>
                <td>{potential_buy.current_amount()}</td>
                <td>{potential_buy.current_price()}</td>
                <td>{potential_buy.historic_data().price()}</td>
                <td>{potential_buy.loss()}</td>
            </tr>
        }
    }).collect()
}

#[function_component(ReduceCostBuyRowsComponent)]
pub fn reduce_cost_buy_row(ReduceCostBuyProps{reduce_cost_buys}: &ReduceCostBuyProps) -> Html {
    reduce_cost_buys.iter().map(|reduce_cost_buy|{
        html! {
            <tr key={reduce_cost_buy.token().as_str()}>
                <td>{reduce_cost_buy.token()}</td>
                <td>{reduce_cost_buy.current_amount()}</td>
                <td>{reduce_cost_buy.current_price()}</td>
                <td>{reduce_cost_buy.historic_data().price()}</td>
                <td>{reduce_cost_buy.amount_to_buy()}</td>
                <td>{reduce_cost_buy.cost()}</td>
            </tr>
        }
    }).collect()
}

#[function_component(PotentialSellRowsComponent)]
pub fn potential_buy_row(PotentialSellProps{potential_sells}: &PotentialSellProps) -> Html {
    potential_sells.iter().map(|potential_sell|{
        html! {
            <tr key={potential_sell.token().as_str()}>
                <td>{potential_sell.token()}</td>
                <td>{potential_sell.current_amount()}</td>
                <td>{potential_sell.current_price()}</td>
                <td>{potential_sell.historic_data().price()}</td>
                <td>{potential_sell.gain()}</td>
            </tr>
        }
    }).collect()
}

#[function_component(SellWithStopRowsComponent)]
pub fn potential_buy_row(SellWithStopProps{sells_with_stop}: &SellWithStopProps) -> Html {
    sells_with_stop.iter().map(|sell_with_stop|{
        html! {
            <tr key={sell_with_stop.token().as_str()}>
                <td>{sell_with_stop.token()}</td>
                <td>{sell_with_stop.current_amount()}</td>
                <td>{sell_with_stop.current_price()}</td>
                <td>{sell_with_stop.historic_data().price()}</td>
                <td>{sell_with_stop.stop_price()}</td>
                <td>{sell_with_stop.gain_on_stop()}</td>
            </tr>
        }
    }).collect()
}

#[function_component(UnknownRowsComponent)]
pub fn potential_buy_row(UnknownProps{unknowns}: &UnknownProps) -> Html {
    unknowns.iter().map(|unknown|{
        let historic_price = match unknown.historic_data() {
            Some(data) => format!("{:?}", data.price()),
            None => String::from("No data")
        };

        let current_price = match unknown.current_price() {
            Some(price) => format!("{:?}", price),
            None => String::from("No data")
        };

        let current_amount = match unknown.current_amount() {
            Some(amount) => format!("{:?}", amount),
            None => String::from("No data")
        };
        html! {
            <tr key={unknown.token().as_str()}>
                <td>{unknown.token()}</td>
                <td>{current_amount}</td>
                <td>{current_price}</td>
                <td>{historic_price}</td>
            </tr>
        }
    }).collect()
}
