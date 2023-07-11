use polars::prelude::*;
use std::fs::File;

/// # Read file to DataFrame
fn read_csv_to_df(file_path: &str) -> DataFrame {
    CsvReader::from_path(file_path)
        .expect("file not found")
        .has_header(true)
        .finish()
        .expect("cannot read file")
}

/// # Read file to Lazy DataFrame
fn read_csv_lazy_df(file_path: &str) -> LazyFrame {
    LazyCsvReader::new(file_path)
        .has_header(true)
        .finish()
        .expect("cannot read file")
}

/// # Write DataFrame to csv file
fn write_df_to_csv(df: &mut DataFrame, file_path: &str) {
    let mut file = File::create(file_path).expect("create failed");
    CsvWriter::new(&mut file)
        .has_header(true)
        .finish(df)
        .expect("write failed");
}

/// # Write DataFrame to json file
fn write_df_to_json(df: &mut DataFrame, file_path: &str) {
    let mut file = File::create(file_path).expect("create failed");
    JsonWriter::new(&mut file).finish(df).expect("write failed");
}

fn main() {
    //! # Main function
    //! - Read data from csv file
    //! - Convert to DataFrame
    //! - Perform some operations
    //! - Write to csv file
    let student_df = read_csv_to_df("notebooks/data/StudentsPerformance.csv");
    let language_df = read_csv_to_df("notebooks/data/LanguageScore.csv");

    let student_df = student_df
        .left_join(&language_df, ["id"], ["id"])
        .expect("join failed")
        .lazy();
    /*
    let student_df = read_csv_lazy_df("notebooks/data/StudentsPerformance.csv");
    let language_df = read_csv_lazy_df("notebooks/data/LanguageScore.csv");

    let student_df = student_df.left_join(language_df, col("id"), col("id"));
    */

    let student_df = student_df.with_column(
        coalesce(&[col("language score"), lit(0)]).alias("language_score"),
    );

    // Add a new column with alias "total score" from "math score", "reading score", "writing score" and "language_score"
    let student_df = student_df.with_column(
        (col("math score")
            + col("reading score")
            + col("writing score")
            + col("language_score"))
        .alias("total score"),
    );
    // Calculate the average of "total score" group by "race/ethnicity" column
    let mut student_df = student_df
        .groupby([col("race/ethnicity")])
        .agg([col("total score").mean().alias("average_score")])
        .filter(col("average_score").gt(lit(200)))
        .sort(
            "average_score",
            SortOptions {
                descending: true,
                ..Default::default()
            },
        )
        .limit(10)
        .collect()
        .unwrap();

    println!("{:?}", student_df);

    write_df_to_csv(&mut student_df, "notebooks/data/StudentsAverageScore.csv");
    write_df_to_json(
        &mut student_df,
        "notebooks/data/StudentsAverageScore.json",
    )
}
