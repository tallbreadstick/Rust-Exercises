use rstest::rstest;
use crate::task::{Bank, BankAccount};

#[rstest]
#[case
(
    vec!["John Zomboid", "Steve Minecraft", "Clark Kent"],
    vec![
        // January - Year 1
        vec![("John Zomboid", 10_000.00), ("Steve Minecraft", 5_000.00), ("Clark Kent", 250_000.00)],
        // February - Year 1
        vec![],
        // March - Year 1
        vec![],
        // April - Year 1
        vec![("Clark Kent", -15_000.00)],
        // May - Year 1
        vec![("John Zomboid", -2_000.00), ("Steve Minecraft", 3_000.00)],
        // June - Year 1
        vec![],
        // July - Year 1
        vec![],
        // August - Year 1
        vec![("Clark Kent", -5_000.00)],
        // September - Year 1
        vec![],
        // October - Year 1
        vec![],
        // November - Year 1
        vec![("John Zomboid", 7_000.00)],
        // December - Year 1
        vec![("Steve Minecraft", -1_000.00), ("Clark Kent", -30_000.00)],
        // January - Year 2
        vec![],
        // February - Year 2
        vec![("Clark Kent", -40_000.00)],
        // March - Year 2
        vec![],
        // April - Year 2
        vec![("John Zomboid", -15_080.00)],
        // May - Year 2
        vec![("John Zomboid", 25_000.00), ("Clark Kent", -10_000.00)],
        // June - Year 2
        vec![],
        // July - Year 2
        vec![],
        // August - Year 2
        vec![("Clark Kent", -50_000.00)],
        // September - Year 2
        vec![],
        // October - Year 2
        vec![("Steve Minecraft", 8_000.00)],
        // November - Year 2
        vec![("John Zomboid", 3_000.00)],
        // December - Year 2
        vec![("Clark Kent", -90_000.00)],
    ],
    vec![
        vec![("Clark Kent", 237350.0), ("John Zomboid", 8080.0), ("Steve Minecraft", 8080.0)],
        vec![("Clark Kent", 204373.5), ("John Zomboid", 15230.8), ("Steve Minecraft", 7150.8)],
        vec![("Clark Kent", 155917.235), ("John Zomboid", 25402.308), ("Steve Minecraft", 7222.308)],
        vec![("Clark Kent", 14325.511499999988), ("John Zomboid", 28686.33108), ("Steve Minecraft", 15374.53108)]
    ]
)]
pub fn test_bank_system(
    #[case] users: Vec<&str>,
    #[case] months: Vec<Vec<(&str, f64)>>,
    #[case] reports: Vec<Vec<(&str, f64)>>
) {

    let mut bank = Bank::new();
    for user in users {
        bank.add_user(BankAccount::new(user));
    }

    for month in months[0..6].iter() {
        bank.handle_monthly_transactions(month);
    }
    bank.bi_annual_evaluation();
    for r in bank.generate_report().iter() {
        assert!(reports[0].contains(r));
    }

    for month in months[6..12].iter() {
        bank.handle_monthly_transactions(month);
    }
    bank.bi_annual_evaluation();
    for r in bank.generate_report().iter() {
        assert!(reports[1].contains(r));
    }

    for month in months[12..18].iter() {
        bank.handle_monthly_transactions(month);
    }
    bank.bi_annual_evaluation();
    for r in bank.generate_report().iter() {
        assert!(reports[2].contains(r));
    }

    for month in months[18..24].iter() {
        bank.handle_monthly_transactions(month);
    }
    bank.bi_annual_evaluation();
    for r in bank.generate_report().iter() {
        assert!(reports[3].contains(r));
    }

}