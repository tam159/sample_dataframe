import polars as pl
from polars import col

student_df = pl.read_csv("notebooks/data/StudentsPerformance.csv")
language_df = pl.read_csv("notebooks/data/LanguageScore.csv")

print(student_df)

student_df = student_df.join(language_df, on="id", how="left")
student_df = student_df.with_columns(
    pl.coalesce(col("language score"), 0).alias("language_score")
).drop("language score")

student_df = student_df.with_columns(
    (
        col("math score")
        + col("reading score")
        + col("writing score")
        + col("language_score")
    ).alias("total score")
)

student_df = (
    student_df.groupby("race/ethnicity")
    .agg(pl.avg("total score").alias("average_score"))
    .filter(col("average_score") > 195)
    .sort(col("average_score"), descending=True)
    .limit(10)
)
print(student_df)


student_df.write_json("notebooks/data/StudentsAverageScore.json")
