use crate::models::emailer::Emailer;

// COC = [(Monthly Cash flow (MCF) x 12) / Initial Total Investment (ITI)] x 100
fn cash_on_cash(monthly_cash_flow: f64, initial_total_investment: f64) -> f64 {
    ((monthly_cash_flow * 12.0) / initial_total_investment) * 100.0
}

// ITI = 29% of Purchase Price(PP)(Which comes from Zillow)
fn initial_total_investment(emailer: &Emailer, purchase_price: f64) -> f64 {
    (emailer.down_payment + emailer.closing_cost) * purchase_price
}

// MCF = Monthly Gross Income(MGI)(comes from Zillow) - Monthly Expenses - Monthly Debt Service
fn monthly_cash_flow(
    emailer: &Emailer,
    monthly_gross_income: f64,
    monthly_expenses: f64,
    monthly_debt_service: f64,
) -> f64 {
    monthly_gross_income
        - monthly_gross_income * emailer.vacancy
        - monthly_expenses
        - monthly_debt_service
}
// Monthly Expenses = Taxes(comes from Zillow) + Insurance($60) + Vacancy(5% of MGI) + Property Management(4% of MGI)+ Capex(5% of MGI) + Repairs(5% of MGI) + Utilities($0)
fn monthly_expenses(emailer: &Emailer, taxes: f64, monthly_gross_income: f64) -> f64 {
    let income = monthly_gross_income - emailer.vacancy * monthly_gross_income;

    let insurance = emailer.insurance;
    let property_management = emailer.property_management * income;
    let capex = emailer.capex * income;
    let repairs = emailer.repairs * income;
    let utilities = emailer.utilities;

    taxes + insurance + property_management + capex + repairs + utilities
}

// Monthly Debt Service = .61 % of Loan
fn monthly_debt_service(emailer: &Emailer, loan: f64) -> f64 {
    // i
    let monthly_interest = emailer.loan_interest / 12.0;
    // n
    let months = emailer.loan_months;
    // (1 + i)^-n
    let exponent = f64::powf(1.0 + monthly_interest, 1.0 / months);
    // 1 - (1 + i)^-n
    let denominator = 1.0 - exponent;
    // p * (i / (1 - (1 + i)^-n))
    loan * (monthly_interest / denominator)
}

// Loan = 75% of Purchase Price(comes from Zillow)
// (1.0 - 0.25) * 649999
fn loan(emailer: &Emailer, purchase_price: f64) -> f64 {
    (1.0 - emailer.down_payment) * purchase_price
}

pub fn calculate_coc(
    emailer: &Emailer,
    purchase_price: f64,
    taxes: f64,
    monthly_gross_income: f64,
) -> f64 {
    // 487,499.25
    let loan = loan(emailer, purchase_price);
    // 1,406,389,662.888143
    let monthly_debt_service = monthly_debt_service(emailer, loan);
    let monthly_expenses = monthly_expenses(emailer, taxes, monthly_gross_income)
        + emailer.additional_monthly_expenses;

    let initial_total_investment = initial_total_investment(emailer, purchase_price);

    let monthly_cash_flow = monthly_cash_flow(
        emailer,
        monthly_gross_income,
        monthly_expenses,
        monthly_debt_service,
    );

    let cash_on_cash = cash_on_cash(monthly_cash_flow, initial_total_investment);

    cash_on_cash
}

#[test]
fn calculate_cash_on_cash() {
    let emailer = Emailer {
        id: 0,
        authentication_id: String::from("abc"),
        search_param: String::from("northampton%20county"),
        frequency: String::from("daily"),
        insurance: 60.0,
        vacancy: 0.05,
        property_management: 0.04,
        capex: 0.05,
        repairs: 0.05,
        utilities: 0.0,
        down_payment: 0.25,
        closing_cost: 0.04,
        loan_interest: 0.041,
        loan_months: 240.0,
        additional_monthly_expenses: 0.0,
        no_bedrooms: Some(3),
        max_price: Some(200000.0),
        min_price: Some(100000.0),
        email: String::from("hgmaxwellking@gmail.com"),
        created_at: crate::utils::now(),
        updated_at: None,
        deleted_at: None,
        active: true,
    };

    let price = 649999.0;
    let taxes = 1000.0;
    let rent = 1850.0;

    assert_eq!(487499.25, loan(&emailer, price));
    assert_eq!(
        -117198748.25988717,
        monthly_debt_service(&emailer, loan(&emailer, price))
    );

    assert_eq!(-16.096, calculate_coc(&emailer, price, taxes, rent));
}
