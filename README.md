# Router Pay Streaming Documentation

## About the Project

Router PayStream is a DApp to provide a Convenient Way for Employees to Receive their Salaries. Instead of Waiting until end of the Month to Receive their Salary, Employees can now Receive their Salary anytime, on any Chain, using Router PayStream.

## Description

The Router PayStream Cosmwasm Smart Contract handles the following functionalities -

- **Employees Salary Information:** The Smart Contract Maintains Information about each Employee's Salary.
- **Employee-wise Current Withdrawal available Limit and Ownership Information:** The Contract keeps Track of the Withdrawal Limit for each Employee and their Ownership Information.
- **Access-control of Salary Withdrawal from Multi-chain Accounts:** The Smart Contract ensures that only Authorized Accounts can initiate Salary Withdrawals.
- **Employer Account Information and Access Control:** The Contract Stores Information about the Employer's Account, including Setting Salaries, fees (if required), and on-hold Amounts.
- **All Salaries Paid in the form of ROUTE Tokens:** ROUTE Token is used to Pay Salaries.
- **Funds Maintained on the Smart Contract:** The Employer Maintains funds on the Smart Contract present on the Router Chain.
- **All Salary-related Information and Calculation Performed on the Router Chain:** All necessary Salary Calculations and Information are processed on the Router Chain.
- **Cross-chain Token Transfers Using the Router Chain Native Coin Bridge**: All Cross-chain Token Transfers are Done through the Router Chain Native Coin Bridge.

The Payment to the User will be made on a per-second Basis. The Payer will Create a PayStream for the Payee, Providing the required Information. The Stream will Start from a Specific Time, and the Payee will be able to Withdraw accumulated ROUTE Tokens until that Point. Additionally, the Payee can Receive ROUTE Tokens on any Chain and Withdraw them from any Supported Chain.
