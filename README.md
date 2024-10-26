**<h1>Router Pay Streaming Documentation</h1>**

**<h2>About this project</h2>**

Router Pay Streaming is an application designed to provide a convenient way for employees to receive their salaries. Instead of waiting until the end of the month to receive their entire month's salary, employees can now collect or receive their accumulated earnings at any time, on any chain, using Router Pay Streaming.

**<h2>Description</h2>**

The Router Pay Cosmwasm smart contract handles the following functionalities:

- Employees Salary Information: The smart contract maintains information about each employee's salary.
- Employee-wise current withdrawal available limit and ownership information: The contract keeps track of the withdrawal limit for each employee and their ownership information.
- Access-control of salary withdrawal from multi-chain accounts: The smart contract ensures that only authorized accounts can initiate salary withdrawals.
- Employer Account Information and access control: The contract stores information about the employer's account, including setting salaries, fees (if required), and on-hold amounts.
- All Salaries paid in the form of the router chain’s native coin (route token): The native coin of the router chain is used to pay salaries.
- Funds maintained on the smart contract: The employer maintains funds on the smart contract present on the router chain.
- All salary-related information and calculation performed on the router chain: All necessary salary calculations and information are processed on the router chain.
- Cross-chain token transfers using the router chain native coin bridge: All cross-chain token transfers are done through the router chain native coin bridge.

The payment to the user will be made on a per-second basis. The payer will create a pay stream for the payee, providing the required information. The stream will start from a specific time, and the payee will be able to withdraw accumulated route tokens until that point. Additionally, the payee can receive route tokens on any chain and withdraw them from any supported chain.
