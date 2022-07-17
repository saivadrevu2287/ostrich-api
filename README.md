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
curl -X POST http://localhost:4000/login -H 'Content-Type: application/json' -d '{"username":"hgmaxwellking@gmail.com","password":"Password123!"}'

curl -X POST https://q0sku06vtg.execute-api.us-east-2.amazonaws.com/v1/auth/login -H 'Content-Type: application/json' -d '{"username":"hgmaxwellking@gmail.com","password":"Password123!"}'
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

###### Get All Users
Path: `/users`
Service: User
Live example:
```shell
curl https://q0sku06vtg.execute-api.us-east-2.amazonaws.com/v1/api/emailers -H 'Authorization: Bearer eyJraWQiOiJzUWpVT3MwQnBLTEtWYTVzS1pcL3REM1Q2VFA3OHliSGxxbFhEejA0Nnh5VT0iLCJhbGciOiJSUzI1NiJ9.eyJzdWIiOiJjZTFkYmM0Zi0yY2M0LTQxYjAtODM1OC02OGJiOWVkNDlmNjYiLCJlbWFpbF92ZXJpZmllZCI6dHJ1ZSwiaXNzIjoiaHR0cHM6XC9cL2NvZ25pdG8taWRwLnVzLWVhc3QtMi5hbWF6b25hd3MuY29tXC91cy1lYXN0LTJfbm5kM2g1WjFaIiwiY29nbml0bzp1c2VybmFtZSI6ImhnbWF4d2VsbGtpbmdAZ21haWwuY29tIiwib3JpZ2luX2p0aSI6ImQ0MzBkNTI0LWNmNzktNDBjYy04Mjk5LTNkZjRhOGY5ZjAwMSIsImF1ZCI6IjcwYXBiYXZsMWZzb2JlZDRqdDdsN21sMThoIiwiZXZlbnRfaWQiOiI5OTRjZDFhMy02NmE0LTQ3YTMtOTBiNi1lZDIzMWI2M2I1ODgiLCJ0b2tlbl91c2UiOiJpZCIsImF1dGhfdGltZSI6MTY1Nzk3MTAxOCwiZXhwIjoxNjU3OTc0NjE4LCJpYXQiOjE2NTc5NzEwMTgsImp0aSI6IjY3YjhjNDhjLWM4MTQtNDc2Ny1iMDcyLWIwMzNhMTQ5NTJlMiIsImVtYWlsIjoiaGdtYXh3ZWxsa2luZ0BnbWFpbC5jb20ifQ.HJhXyqTXUB_V8mDAxDH_x48VqDG6ef7QC2v2HEepPiZx1UCzpK5bFdbeDt4rX4Scuwyah9g0Q4X8PvqmC7oTVA9RFiUo7pgZ2gc-vCYVCEFLO7UKXVjSwMOseIKH9HZnzsyJE-sagPMb0-vkxxW4RIitqddiIzkjqa-eqrqcKM3JLrgiUMac_SHAGyqrqTgs70xcQnFYrW9uugTd1FQ_eOYz_I5N2mmaDeAOveu0-vUfsnbMSa_Xe1-lDuG8Lu6iBts69T9bgfaqLm9XECsQSpJ4SB5RhalbzbaY8nKQu1WafBUT_vxIrI5QgfyQST7houh7LHKt6LhFmGL-fa7YlA'
```

###### Insert New Emailer
Path: `/resend-code`
Service: Auth
Live example:
```shell
curl -X POST http://localhost:4000/emailers -H 'Content-Type: application/json' -d '{"search_param":"astoria%20nyc","frequency":"daily"}' -H "Authorization: Bearer eyJraWQiOiJzUWpVT3MwQnBLTEtWYTVzS1pcL3REM1Q2VFA3OHliSGxxbFhEejA0Nnh5VT0iLCJhbGciOiJSUzI1NiJ9.eyJzdWIiOiJjZTFkYmM0Zi0yY2M0LTQxYjAtODM1OC02OGJiOWVkNDlmNjYiLCJlbWFpbF92ZXJpZmllZCI6dHJ1ZSwiaXNzIjoiaHR0cHM6XC9cL2NvZ25pdG8taWRwLnVzLWVhc3QtMi5hbWF6b25hd3MuY29tXC91cy1lYXN0LTJfbm5kM2g1WjFaIiwiY29nbml0bzp1c2VybmFtZSI6ImhnbWF4d2VsbGtpbmdAZ21haWwuY29tIiwib3JpZ2luX2p0aSI6ImUxYTU1NjFlLTdhMjMtNGI5ZS1hNGI2LTkzMGRhOGUyMzExYiIsImF1ZCI6IjcwYXBiYXZsMWZzb2JlZDRqdDdsN21sMThoIiwiZXZlbnRfaWQiOiJlOGNjMzI0MS0zNWY3LTRjNTEtOWU5OC04MTIyZmZmOTRhZGYiLCJ0b2tlbl91c2UiOiJpZCIsImF1dGhfdGltZSI6MTY1ODAwNzg0NCwiZXhwIjoxNjU4MDExNDQ0LCJpYXQiOjE2NTgwMDc4NDQsImp0aSI6IjQ0NzE3OTcyLTY3NjQtNGZkZi1iYWY4LWJhNzZkN2I2MDNhMyIsImVtYWlsIjoiaGdtYXh3ZWxsa2luZ0BnbWFpbC5jb20ifQ.kzJbNEciNxkc1dNnZZltxOkftCdV_eBeF_E1trjcOQo-2S0DEX2PjqqLZ3QvBTAVNhde6X41cPhIASHCO0WFWEQLlTDcC1NKAExEI5THDrZ8NwmxNn3GTadJ4nAKSHwmNBHCRRrr_d3osktwvkd_Cp7nfwn9scmVJg6y2VScVY5bp0DlRHVtSl8E6685MIWk5jUr5P6JuoZAcA5kofrl4P3bGqnWDQLYf7e5qBtWT7DpY3VxK6HJmoDdu-60_GfxqQG50idJ3L5ABAnPwn1r4VbU-JEvEgfEvSxjVHhm_th2_EKxayQ8tT6gfAdaV1vu4zgQGHK_S0jklar170k71g"

curl -X POST https://q0sku06vtg.execute-api.us-east-2.amazonaws.com/v1/api/emailers -H 'Content-Type: application/json' -d '{"search_param":"astoria%20nyc","frequency":"daily"}' -H "Authorization: Bearer eyJraWQiOiJzUWpVT3MwQnBLTEtWYTVzS1pcL3REM1Q2VFA3OHliSGxxbFhEejA0Nnh5VT0iLCJhbGciOiJSUzI1NiJ9.eyJzdWIiOiJjZTFkYmM0Zi0yY2M0LTQxYjAtODM1OC02OGJiOWVkNDlmNjYiLCJlbWFpbF92ZXJpZmllZCI6dHJ1ZSwiaXNzIjoiaHR0cHM6XC9cL2NvZ25pdG8taWRwLnVzLWVhc3QtMi5hbWF6b25hd3MuY29tXC91cy1lYXN0LTJfbm5kM2g1WjFaIiwiY29nbml0bzp1c2VybmFtZSI6ImhnbWF4d2VsbGtpbmdAZ21haWwuY29tIiwib3JpZ2luX2p0aSI6ImVlZWM1NzFhLTZiZTEtNDA2MC04YjhiLTVmZGM5OWQwMTFjMyIsImF1ZCI6IjcwYXBiYXZsMWZzb2JlZDRqdDdsN21sMThoIiwiZXZlbnRfaWQiOiI2NmFlZGE4MC01MDUxLTQ2MGMtOTAzOS0zNTE5MDJjODM4ODYiLCJ0b2tlbl91c2UiOiJpZCIsImF1dGhfdGltZSI6MTY1Nzk3MDQ3OCwiZXhwIjoxNjU3OTc0MDc4LCJpYXQiOjE2NTc5NzA0NzgsImp0aSI6IjY5MGExOWNjLTJlNjctNGQ0My1iNTEzLTM4OWVkNzBlYjA4YSIsImVtYWlsIjoiaGdtYXh3ZWxsa2luZ0BnbWFpbC5jb20ifQ.nUSpgD8zaZLhcsA59_oLBs120Oo1sVaQDP_xfNUOl3fPkb7f30-UzNg2OdN3sgl-Uilu0jWd2o4IYgYHIxawvaHs5tlAq_nayx9MLR9Gze0w7_O82j7eNib4EplURPh00qW4WtHfBaPhJzfcLxU_bBYmFF9-zWXUBZRgXOYwencgW30S205GOu6H4zUIEmGYQBXcnDX6dJt-BicnGX1eo8Vt6RbBK22BCKxbDAsEuj70GBupcLlGuMKVCPJYJ9awM49liSgEsWTWXzSrjllOLTxvsXXOztRwTsqsQ53y5HixjUaML4elPz6ew9yV9EUxWb61IFysxjRj5JIHZBOsdQ"
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