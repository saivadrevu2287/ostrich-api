# Rust Authentticator Service

## To Run
- install [rustup](https://rustup.rs/)
- run `cargo install diesel-cli`
- run `docker-compose up`
- copy over `.env` from example below
- run `diesel setup`
- run `cargo run --bin auth_service`

## Endpoints
###### Sign up
Path: `/sign-up`
Service: Auth
Live example:
```shell
curl -X POST http://localhost:4000/sign-up -H 'Content-Type: application/json' -d '{"username":"hgmaxwellking@gmail.com","password":"Password123!"}'

curl -X POST https://q0sku06vtg.execute-api.us-east-2.amazonaws.com/v1/auth/sign-up -H 'Content-Type: application/json' -d '{"username":"hgmaxwellking@gmail.com","password":"Password123!"}'
```
###### Log in
Path: `/login`
Service: Auth
Live example:
```shell
curl -X POST http://localhost:4000/login -H 'Content-Type: application/json' -d '{"username":"sabayone@gmail.com","password":"password"}'

curl -X POST https://q0sku06vtg.execute-api.us-east-2.amazonaws.com/v1/auth/login -H 'Content-Type: application/json' -d '{"username":"sabayone@gmail.com","password":"password"}'
```
###### Verify
Path: `/verify`
Service: Auth
Live example:
```
curl -X POST http://localhost:4000/verify -H 'Content-Type: application/json' -d '{"username":"hgmaxwellking@gmail.com","code":"12345"}'

curl -X POST https://q0sku06vtg.execute-api.us-east-2.amazonaws.com/v1/auth/verify -H 'Content-Type: application/json' -d '{"username":"hgmaxwellking@gmail.com","code":"332719"}'
```

###### Resend Confirmation Code
Path: `/resend-code`
Service: Auth
Live example:
```shell
curl -X POST http://localhost:4000/resend-code -H 'Content-Type: application/json' -d '{"username":"hgmaxwellking@gmail.com"}'

curl -X POST https://q0sku06vtg.execute-api.us-east-2.amazonaws.com/v1/auth/resend-code -H 'Content-Type: application/json' -d '{"username":"hgmaxwellking@gmail.com"}'
```

###### Get All Emailers
Path: `/users`
Service: User
Live example:
```shell
curl https://q0sku06vtg.execute-api.us-east-2.amazonaws.com/v1/api/emailers -H 'Authorization: Bearer [token]'
```

###### Insert New Emailer
Path: `/resend-code`
Service: Auth
Live example:
```shell
curl -X POST http://localhost:4000/emailers -H 'Content-Type: application/json' -d '{"search_param":"northampton%20county","frequency":"daily","insurance":60,"vacancy":0.05,"property_management":0.04,"capex":0.05,"repairs":0.05,"utilities":0,"down_payment":0.25,"closing_cost":0.04,"loan_interest":0.041,"loan_months":240,"additional_monthly_expenses":0,"no_bedrooms":3,"max_price":200000,"min_price":100000,"email":"hgmaxwellking@gmail.com"}'

curl -X POST https://q0sku06vtg.execute-api.us-east-2.amazonaws.com/v1/api/emailers -H 'Content-Type: application/json' -d '{"search_param":"westchester","frequency":"daily"}' -H "Authorization: Bearer [token]"
```

###### Insert New Emailer
Path: `/resend-code`
Service: Auth
Live example:
```shell
curl http://localhost:4000/emailers/test-search-param\?search_param=[your search param] -H "Authorization: Bearer [token]"

curl https://q0sku06vtg.execute-api.us-east-2.amazonaws.com/v1/api/emailers/test-search-param\?search_param=westchester%20new%20york -H "Authorization: Bearer eyJraWQiOiJzUWpVT3MwQnBLTEtWYTVzS1pcL3REM1Q2VFA3OHliSGxxbFhEejA0Nnh5VT0iLCJhbGciOiJSUzI1NiJ9.eyJzdWIiOiI4ZTNjMmM0ZC1iYjQwLTRjYTYtYjAzYi0zMDY0MGZjZjkwMmYiLCJlbWFpbF92ZXJpZmllZCI6dHJ1ZSwiaXNzIjoiaHR0cHM6XC9cL2NvZ25pdG8taWRwLnVzLWVhc3QtMi5hbWF6b25hd3MuY29tXC91cy1lYXN0LTJfbm5kM2g1WjFaIiwiY29nbml0bzp1c2VybmFtZSI6InNhYmF5b25lQGdtYWlsLmNvbSIsIm9yaWdpbl9qdGkiOiIyN2FhYWE4Mi00NGVkLTQ3YzUtYmQxNi0wNTJmMzY5OTQ5YTAiLCJhdWQiOiI3MGFwYmF2bDFmc29iZWQ0anQ3bDdtbDE4aCIsImV2ZW50X2lkIjoiMjJiOWFmOWItMTEzOS00YTBhLWE5NjctYjg5ZDI1NDFmMWQ4IiwidG9rZW5fdXNlIjoiaWQiLCJhdXRoX3RpbWUiOjE2NTgwNjE2ODAsImV4cCI6MTY1ODA2NTI4MCwiaWF0IjoxNjU4MDYxNjgwLCJqdGkiOiIyODAzOTQxNy1iZWI1LTRmMTctYjYwZS1mNDY2ODE5MDVhYjgiLCJlbWFpbCI6InNhYmF5b25lQGdtYWlsLmNvbSJ9.PtXMjXyIemGhwoQ2jCmFKw1Oev3oSK1AActR9C8VYB72zFOzORMmdGz04oZ04OuXJrHNF0VoZmnox-9LDBp8X1gebnIyPdggPQ5FjyPtQaKLFXrqClO0yTIgTaaCOuslr0JmIuapt-O_JdO-7VDiZUat0OB4QPbfhCtCd3i3s-Z-1xprVIh8XM1jFq22SL_JwLBK9thSmyii4iTCGhb_dHxwfaPG96ZsMoPQoPEc0qLlBSBuaJi8Ja2mTr8gS8Yp--YeAgoV6YsixhmJge_4v3nlg4NerJC_QrpoKjvPYvauJ1FhllPTlvoLNwJHsHg1nvZWy0irPOYA-pInH1iQ-g"
```

#### Env Example
```
POSTGRES_USER=unicorn_user
POSTGRES_PASSWORD=magical_password
POSTGRES_DB=rainbow_database
DATABASE_URL=postgres://unicorn_user:magical_password@localhost:5432/rainbow_database
PORT=4000
HOST=0.0.0.0
ENABLE_TLS=false
AWS_REGION=us-east-2
```


"insurance":60,"vacancy":0.05,"property":0.04,"capex":0.05,"repairs":0.05,"utilities":0,"down_payment":0.25,"closing_cost":0.04,"loan_interest":0.041,"loan_months":240,"additional_monthly_expenses":0,