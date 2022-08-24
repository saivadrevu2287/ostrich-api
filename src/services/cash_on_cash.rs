pub struct CashOnCashCalculationParameters {
    pub insurance: f64,
    pub vacancy: f64,
    pub property_management: f64,
    pub capex: f64,
    pub repairs: f64,
    pub utilities: f64,
    pub down_payment: f64,
    pub closing_cost: f64,
    pub loan_interest: f64,
    pub loan_months: f64,
    pub additional_monthly_expenses: f64,
}

// COC = [(Monthly Cash flow (MCF) x 12) / Initial Total Investment (ITI)] x 100
fn cash_on_cash(monthly_cash_flow: f64, initial_total_investment: f64) -> f64 {
    ((monthly_cash_flow * 12.0) / initial_total_investment) * 100.0
}

// ITI = 29% of Purchase Price(PP)(Which comes from Zillow)
fn initial_total_investment(
    coc_params: &CashOnCashCalculationParameters,
    purchase_price: f64,
) -> f64 {
    ((coc_params.down_payment / 100.0) + (coc_params.closing_cost / 100.0)) * purchase_price
}

// MCF = Monthly Gross Income(MGI)(comes from Zillow) - Monthly Expenses - Monthly Debt Service
fn monthly_cash_flow(
    coc_params: &CashOnCashCalculationParameters,
    monthly_gross_income: f64,
    monthly_expenses: f64,
    monthly_debt_service: f64,
) -> f64 {
    monthly_gross_income
        - monthly_gross_income * (coc_params.vacancy / 100.0)
        - monthly_expenses
        - monthly_debt_service
}
// Monthly Expenses = Taxes(comes from Zillow) + Insurance($60) + Vacancy(5% of MGI) + Property Management(4% of MGI)+ Capex(5% of MGI) + Repairs(5% of MGI) + Utilities($0)
fn monthly_expenses(
    coc_params: &CashOnCashCalculationParameters,
    taxes: f64,
    monthly_gross_income: f64,
) -> f64 {
    let income = monthly_gross_income - (coc_params.vacancy / 100.0) * monthly_gross_income;

    let insurance = coc_params.insurance;
    let property_management = (coc_params.property_management / 100.0) * income;
    let capex = (coc_params.capex / 100.0) * income;
    let repairs = (coc_params.repairs / 100.0) * income;
    let utilities = coc_params.utilities;

    taxes + insurance + property_management + capex + repairs + utilities
}

// Monthly Debt Service = .61 % of Loan
fn monthly_debt_service(coc_params: &CashOnCashCalculationParameters, loan: f64) -> f64 {
    // i
    let monthly_interest = (coc_params.loan_interest / 100.0) / 12.0;
    // n
    let months = coc_params.loan_months;
    // (1 + i)^-n
    let exponent = f64::powf(1.0 / (1.0 + monthly_interest), months);
    // 1 - (1 + i)^-n
    let denominator = 1.0 - exponent;
    // p * (i / (1 - (1 + i)^-n))
    loan * (monthly_interest / denominator)
}

// Loan = 75% of Purchase Price(comes from Zillow)
fn loan(coc_params: &CashOnCashCalculationParameters, purchase_price: f64) -> f64 {
    (1.0 - (coc_params.down_payment / 100.0)) * purchase_price
}

pub fn calculate_coc(
    coc_params: &CashOnCashCalculationParameters,
    purchase_price: f64,
    taxes: f64,
    monthly_gross_income: f64,
) -> f64 {
    let loan = loan(coc_params, purchase_price);
    let monthly_debt_service = monthly_debt_service(coc_params, loan);
    let monthly_expenses = monthly_expenses(coc_params, taxes, monthly_gross_income)
        + coc_params.additional_monthly_expenses;

    let initial_total_investment = initial_total_investment(coc_params, purchase_price);

    let monthly_cash_flow = monthly_cash_flow(
        coc_params,
        monthly_gross_income,
        monthly_expenses,
        monthly_debt_service,
    );

    let cash_on_cash = cash_on_cash(monthly_cash_flow, initial_total_investment);

    cash_on_cash
}

#[test]
fn calculate_cash_on_cash() {
    let coc_params = CashOnCashCalculationParameters {
        insurance: 60.0,
        vacancy: 5.0,
        property_management: 4.0,
        capex: 5.0,
        repairs: 5.0,
        utilities: 0.0,
        down_payment: 25.0,
        closing_cost: 4.0,
        loan_interest: 4.0,
        loan_months: 240.0,
        additional_monthly_expenses: 0.0,
    };

    let purchase_price = 290000.0;
    let taxes = 157.0;
    let monthly_gross_income = 2400.0;

    let loan = loan(&coc_params, purchase_price);
    let monthly_debt_service = monthly_debt_service(&coc_params, loan);
    let monthly_expenses = monthly_expenses(&coc_params, taxes, monthly_gross_income)
        + &coc_params.additional_monthly_expenses;

    let initial_total_investment = initial_total_investment(&coc_params, purchase_price);

    let monthly_cash_flow = monthly_cash_flow(
        &coc_params,
        monthly_gross_income,
        monthly_expenses,
        monthly_debt_service,
    );

    let cash_on_cash = cash_on_cash(monthly_cash_flow, initial_total_investment);

    // cashOnCash: 5.911588602925929
    // initialTotalInvestment: 84100
    // loan: 217500
    // monthlyCashFlow: 414.3038345883922
    // monthlyDebtService: 1329.4961654116078
    // monthlyExpenses: 536.2

    assert_eq!(217500.0, loan);

    // i
    let monthly_interest = (coc_params.loan_interest / 100.0) / 12.0;
    // n
    let months = coc_params.loan_months;
    // (1 / (1 + i))^n
    let exponent = f64::powf(1.0 / (1.0 + monthly_interest), months);
    // 1 - (1 + i)^-n
    let denominator = 1.0 - exponent;
    // p * (i / (1 - (1 + i)^-n))

    // denominator: 0.558952345507466
    // exponent: 0.441047654492534
    // monthlyInterest: 0.003416666666666667
    // months: 240
    assert_eq!(0.0033333333333333335, monthly_interest);
    assert_eq!(240.0, months);
    assert_eq!(0.44992713918832056, exponent);
    assert_eq!(0.5500728608116794, denominator);

    assert_eq!(1318.00721622623, monthly_debt_service);
    assert_eq!(536.2, monthly_expenses);
    assert_eq!(84100.0, initial_total_investment);
    assert_eq!(425.79278377377, monthly_cash_flow);
    assert_eq!(6.075521290469965, cash_on_cash);
}
