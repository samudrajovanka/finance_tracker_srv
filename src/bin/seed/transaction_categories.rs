use sqlx::PgPool;

pub async fn seed_transaction_categories(pool: &PgPool) -> Result<(), sqlx::Error> {
    println!("Seeding transaction categories...");

    let expense_transaction_categories = vec![
        ("Bills", "expense"),
        ("Debt", "expense"),
        ("Education", "expense"),
        ("Entertainment", "expense"),
        ("Family", "expense"),
        ("Food & Drinks", "expense"),
        ("Healthcare", "expense"),
        ("Savings", "expense"),
        ("Shopping", "expense"),
        ("Top up", "expense"),
        ("Transportation", "expense"),
        ("Others", "expense")
    ];

    let income_transaction_categories = vec![
        ("Salary", "income"),
        ("Investments", "income"),
        ("Savings", "income"),
        ("Others", "income")
    ];

    let transaction_categories = expense_transaction_categories
        .into_iter()
        .chain(income_transaction_categories)
        .collect::<Vec<_>>();

    let mut builder = sqlx::QueryBuilder::new(
        "INSERT INTO transaction_categories (name, type)"
    );

    builder.push_values(transaction_categories, |mut b, (name, t_type)| {
        b.push_bind(name)
            .push_bind(t_type).push_unseparated("::transaction_type");
    });

    builder.push(" ON CONFLICT DO NOTHING");

    builder.build().execute(pool).await?;

    Ok(())
}