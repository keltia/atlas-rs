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

// -------------------------------------------------------------------------
// Standard library
use std::fmt;
use std::fmt::Formatter;

// External crates
use serde::{Serialize, Deserialize};

// Our crates
use crate::client::Client;
use crate::common::add_opts;
use crate::errors::*;
use crate::request::{Param, RequestBuilder};

// -------------------------------------------------------------------------

/// All operations available
#[derive(Debug)]
pub enum Ops {
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

// -------------------------------------------------------------------------

/// This is the structure describing credits
///
#[derive(Serialize, Deserialize, Debug)]
pub struct Credits {
    /// Current number of available credits
    pub current_balance: u32,
    /// Do we perform a credit check before creating the measurement
    pub credit_checked: bool,
    /// Maximum number of credits that can be spend in a day
    pub max_daily_credits: u32,
    /// Total estimated daily income from all income items
    pub estimated_daily_income: u32,
    /// Total estimated daily expenditure from all expense items
    pub estimated_daily_expenditure: u32,
    /// Estimated daily income minus estimated daily expenditure
    pub estimated_daily_balance: u32,
    /// Time that the estimates were produced
    pub calculation_time: String,
    /// Estimated elapsed time from `calculation_time` until total credit balance will drop to zero
    pub estimated_runout_seconds: Option<u32>,
    /// Number of results from user-defined measurements in the past 24 hours
    pub past_day_measurement_results: u32,
    /// Number of credits spent in the past 24 hours
    pub past_day_credits_spent: u32,
    /// Last time the account was debited
    pub last_date_debited: String,
    /// Last time the account was credited
    pub last_date_credited: String,
    /// URL of the income-items list
    pub income_items: String,
    /// URL of the expense-items list
    pub expense_items: String,
    /// URL of the transactions list
    pub transactions: String,
}

/// Implement the Display trait.
///
impl fmt::Display for Credits {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

impl Credits {
    pub fn dispatch<'cr>(
        mut r: RequestBuilder<'cr>,
        ops: Ops,
        data: Param,
    ) -> RequestBuilder<'cr> {
        let opts = r.c.opts.clone();

        let url = reqwest::Url::parse_with_params(
            r.r.url().as_str(),
            opts.iter(),
        )
            .unwrap();
        r.r = reqwest::blocking::Request::new(r.r.method().clone(), url);
        r
    }
}

// -------------------------------------------------------------------------

/// Show eligibility for claiming bonus credits for associated RIPE NCC
#[derive(Serialize, Deserialize, Debug)]
pub struct ClaimRequests {
    /// Array of RegIDs to claim credits for
    pub members: Vec<Member>,
}

/// struct for income generated by a probe
///
#[derive(Serialize, Deserialize, Debug)]
pub struct ProbeIncome {
    /// ID of the probe,
    pub probe_id: u32,
    /// URL of the probe in the probes API,
    pub probe: String,
    /// Description of this income item,
    pub description: String,
    /// Credits that would be earned from this probe being connected for a full day,
    pub max_daily_connected_income: u32,
    ///Estimated credits earned from this probe per day,
    pub estimated_daily_income: String,
}

/// struct for income generated by a probe that one's hosting
///
#[derive(Serialize, Deserialize, Debug)]
pub struct HostedProbeIncome {
    /// ID of the probe,
    pub probe_id: u32,
    /// URL of the probe in the probes API,
    pub probe: String,
    /// Description of this income item,
    pub description: String,
    /// Credits that would be earned from this probe being connected for a full day,
    pub max_daily_connected_income: u32,
    ///Estimated credits earned from this probe per day,
    pub estimated_daily_income: String,
    /// Credits awarded yesterday for measurement results returned by the probe
    pub yesterday_results_reward: u32,
}

/// struct for income generated by different kind of probes
///
#[derive(Serialize, Deserialize, Debug)]
pub struct IncomeGroups {
    /// Connected probes that you host or directly manage.
    pub hosted_probes: Vec<HostedProbeIncome>,
    /// Connected probes that you sponsor,
    pub sponsored_probes: Vec<ProbeIncome>,
    /// Connected probes for which you are an ambassador.
    pub ambassador_probes: Vec<ProbeIncome>,
    /// Connected anchors that you host or directly manage.
    pub hosted_anchors: Vec<HostedProbeIncome>,
    /// Connected anchors that you sponsor.
    pub sponsored_anchors: Vec<ProbeIncome>,
}

/// struct for expenses for a given measurement
///
#[derive(Serialize, Deserialize, Debug)]
pub struct MeasurementExpense {
    /// ID of the measurement.
    pub measurement_id: u32,
    /// URL of the measurement in the measurements API,
    pub measurement: String,
    /// Description of this expense item,
    pub description: String,
    /// Time when this measurement started or will start,
    pub scheduled_start_time: String,
    /// Time when this measurement is scheduled to stop, if at all,
    pub scheduled_stop_time: String,
    /// Amount of time in the next 24 hours that this measurement is scheduled to be running,
    pub next_day_running_time_seconds: u32,
    /// Best available estimate of number of participating probes,
    pub estimated_participants: u32,
    /// Estimated daily cost based on the measurement specification and participating probes
    pub estimated_daily_cost: u32,
}

/// Struct to hold all income items
///
#[derive(Serialize, Deserialize, Debug)]
pub struct IncomeItems {
    /// Grouped list of income items
    pub groups: Vec<IncomeGroups>,
    /// Total estimated daily income from all income items
    pub total_estimated_daily_income: u32,
}

/// Data for transferring credits to a specific user
///
pub struct Transfer {
    /// How many credits to transfer,
    pub amount: u32,
    /// Email address of a RIPE Access user,
    pub recipient: String,
}

/// Member's data
///
#[derive(Serialize, Deserialize, Debug)]
pub struct Member {
    /// RIPE NCC RegID,
    pub reg_id: String,
    /// Number of times credits were ever claimed for this member,
    pub total_claims: u32,
    /// Whether this member is currently eligible for claiming credits,
    pub can_claim: bool,
    /// The last time that any user claimed credits for this member,
    pub last_claim: String,
    /// The next time that this member will be eligible for claimimng credits,
    pub next_claim: String,
    /// Human-readable name of this member,
    pub name: String,
}

/// Struct to list of members
///
#[derive(Serialize, Deserialize, Debug)]
pub struct MemberListing {
    /// List
    pub members: Vec<Member>,
}

/// Struct representing an expense group.
///
#[derive(Serialize, Deserialize, Debug)]
pub struct ExpenseGroup {
    /// Measurements scheduled by and billed to you,
    owned_measurements: Vec<MeasurementExpense>,
    /// Measurements scheduled by other users that are billed to you,
    billed_measurements: Vec<MeasurementExpense>,
}

/// Struct to hold all expense items
///
#[derive(Serialize, Deserialize, Debug)]
pub struct ExpenseItems {
    /// Grouped list of expense groups
    pub groups: Vec<ExpenseGroup>,
    /// Total estimated daily expenditure from all expense items
    pub total_estimated_daily_expenditure: u32,
}

