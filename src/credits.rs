//! Structs and methods to deal with credits
//!

//  This has the following call-tree
//
//           ----- /credits                ----- /get
//                                         ----- /get  ----- /incomes
//                                                           /expenses
//                                                           /transfers
//                                                           /transactions
//                                                           /members
//                                                           /members      ----- /claim
//

/// All operations available
#[derive(Debug)]
enum Ops {
    Get = 1,
    Incomes,
    Expenses,
    Transfers,
    Transactions,
    Members,
    Claim,
}

/// Generate the proper URL for the service we want in the given category
fn set_url(ops: Ops) -> String {
    match ops {
        Ops::Get => "/credits/".to_string(),                 // /get
        Ops::Incomes => "/credits/incomes/".to_string(),     // /set
        Ops::Expenses => "/credits/expenses/".to_string(),   // /delete
        Ops::Transfers => "/credits/transfers/".to_string(), // /list
        Ops::Transactions => "/credits/transactions/".to_string(), // /list
        Ops::Members => "/credits/members/".to_string(),     // /create
        Ops::Claim => "/credits/members/claim/".to_string(), // /create
    }
}
